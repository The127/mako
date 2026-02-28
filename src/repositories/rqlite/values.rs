use std::cell::RefCell;
use std::rc::Rc;
use crate::repositories::rqlite::context::Transaction;
use crate::repositories::values::{Value, ValueRepository};

struct ValueModel {
    path: String,
    key: String,
    value: String,
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
) -> Box<dyn ValueRepository> {
    Box::new(ValueRepositoryImpl { transaction })
}

struct ValueRepositoryImpl {
    transaction: Rc<RefCell<Transaction>>,
}

impl ValueRepository for ValueRepositoryImpl {
    fn insert(&self, value: Value) {
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
            )",
            &mapped.path,
            &mapped.key,
            &mapped.value,
        ]);
    }
}
