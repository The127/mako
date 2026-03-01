use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use rqlite_client::{response, Connection, Mapping};
use rqlite_client::response::mapping::Standard;
use crate::repositories::rqlite::context::Transaction;
use crate::repositories::values::{Value, ValueRepository};

struct ValueModel {
    path: String,
    key: String,
    value: String,
}

impl ValueModel {
    fn scan(row: &Standard) -> Self {
        let path = row.value(0, 0).unwrap();
        let key = row.value(0, 1).unwrap();
        let value = row.value(0, 2).unwrap();

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
            Some(Mapping::Standard(row)) => {
                if let Some(_) = &row.values {
                    Ok(Some(Value::from(ValueModel::scan(&row))))
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

        let response_result = response::query::Query::from(query.request_run().unwrap());

        response_result.into_iter().map(|mapping| {
            match mapping {
                Mapping::Error(err) => Err(Box::<dyn std::error::Error>::from(err)),
                Mapping::Standard(row) => Ok(Value::from(ValueModel::scan(&row))),
                _ => unreachable!(),
            }
        }).collect()
    }
}
