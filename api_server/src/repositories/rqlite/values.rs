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
}

impl ValueModel {
    fn scan(row: &Vec<serde_json::Value>) -> Self {
        let path = row[0].as_str().unwrap();
        let key = row[1].as_str().unwrap();
        let value = row[2].as_str().unwrap();

        ValueModel {
            path: path.to_string(),
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl From<&Value> for ValueModel {
    fn from(value: &Value) -> Self {
        Self {
            path: value.path(),
            key: value.key(),
            value: value.value(),
        }
    }
}

impl From<ValueModel> for Value {
    fn from(value: ValueModel) -> Self {
        Value::new(value.path, value.key, value.value)
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
                value
            ) values (
                ?,
                ?,
                ?
            ) on conflict (path, key) do update set value = excluded.value",
            &mapped.path,
            &mapped.key,
            &mapped.value,
        ]);
    }

    fn get(&self, path: &str, key: &str) -> Result<Option<Value>, Box<dyn std::error::Error>> {
        let query = self.conn.query().push_sql_values(&[
            "
            select path, key, value from \"values\" where path = ? and key = ?
            ",
            path,
            key,
        ]);

        let response_result = response::query::Query::from(query.request_run().unwrap());

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

    fn list(&self, path: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let query = self.conn.query().push_sql_values(&[
            "
            select path, key, value from \"values\" where path = ?
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
}
