use crate::repositories::rqlite::context::Transaction;
use crate::repositories::values::{Value, ValueRepository};
use rqlite_client::{response, Connection, Mapping};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

struct ValueModel {
    path: String,
    key: String,
    value: String,
    version: i64,
}

impl ValueModel {
    fn scan(row: &Vec<serde_json::Value>) -> Self {
        let path = row[0].as_str().unwrap();
        let key = row[1].as_str().unwrap();
        let value = row[2].as_str().unwrap();
        let version = row[3].as_i64().unwrap();

        ValueModel {
            path: path.to_string(),
            key: key.to_string(),
            value: value.to_string(),
            version,
        }
    }
}

impl From<&Value> for ValueModel {
    fn from(value: &Value) -> Self {
        Self {
            path: value.path(),
            key: value.key(),
            value: value.value(),
            version: value.version(),
        }
    }
}

impl From<ValueModel> for Value {
    fn from(value: ValueModel) -> Self {
        Value::new_with_version(value.path, value.key, value.value, value.version)
    }
}

pub fn new_value_repository(
    transaction: Rc<RefCell<Transaction>>,
    conn: Arc<Connection>,
) -> Box<dyn ValueRepository> {
    Box::new(ValueRepositoryImpl { transaction, conn })
}

struct ValueRepositoryImpl {
    transaction: Rc<RefCell<Transaction>>,
    conn: Arc<Connection>,
}

impl ValueRepository for ValueRepositoryImpl {
    fn set(&self, value: Value) {
        let mapped = ValueModel::from(&value);

        let mut tx = self.transaction.borrow_mut();
        tx.push_sql_values(&["
            insert into \"values\" (
                path,
                key,
                value,
                version
            ) values (
                ?,
                ?,
                ?,
                1
            )
            on conflict (path, key)
            do update set
            value = excluded.value,
            version = version + 1",
            &mapped.path,
            &mapped.key,
            &mapped.value
        ]);
    }

    fn get(&self, path: &str, key: &str) -> Result<Option<Value>, Box<dyn std::error::Error>> {
        let query = self.conn.query().push_sql_values(&[
            "
            select path, key, value, version from \"values\" where path = ? and key = ?
            ",
            path,
            key,
        ]);

        let response_result = response::query::Query::from(query.request_run()?);

        match response_result.into_iter().next() {
            Some(Mapping::Standard(standard)) => {
                if let Some(mapping) = &standard.values {
                    if let Some(row) = mapping.first() {
                        Ok(Some(Value::from(ValueModel::scan(row))))
                    }else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
            Some(Mapping::Error(error)) => {
                Err(Box::<dyn std::error::Error>::from(error))
            }
            _ => Err(Box::<dyn std::error::Error>::from("Unexpected response format"))
        }
    }

    fn get_version(&self, path: &str, key: &str) -> Result<Option<i64>, Box<dyn std::error::Error>> {
        let query = self.conn.query().push_sql_values(&[
            "
            select version from \"values\" where path = ? and key = ?
            ",
            path,
            key,
        ]);

        let response_result = response::query::Query::from(query.request_run()?);

        if let Some(Mapping::Standard(standard)) = response_result.into_iter().next() {
            if let Some(mapping) = &standard.values {
                if let Some(row) = mapping.first() {
                    let version = row[0].as_i64().unwrap();
                    return Ok(Some(version));
                }
            }
        }

        Ok(None)
    }

    fn list(&self, path: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let query = self.conn.query().push_sql_values(&[
            "
            select path, key, value, version from \"values\" where path = ?
            ",
            path,
        ]);

        let response_result = response::query::Query::from(query.request_run()?);

        match response_result.into_iter().next() {
            Some(Mapping::Error(err)) => Err(Box::<dyn std::error::Error>::from(err)),
            Some(Mapping::Standard(standard)) => {
                if let Some(mapping) = &standard.values {
                    Ok(mapping.iter().map(|row| Value::from(ValueModel::scan(row))).collect())
                }else{
                    Ok(Vec::new())
                }
            },
            _ => unreachable!(),
        }
    }

    fn delete_if_exists(&self, path: &str, key: &str) {
        let mut tx = self.transaction.borrow_mut();
        tx.push_sql_values(&[
            "
            delete from \"values\" where path = ? and key = ?
            ",
            path,
            key,
        ]);
    }
}
