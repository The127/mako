use crate::repositories::context::{DbContext, DbError};
use crate::repositories::namespaces::NamespaceRepository;
use crate::repositories::rqlite::namespaces::new_namespace_repository;
use crate::repositories::rqlite::values::new_value_repository;
use crate::repositories::values::ValueRepository;
use rqlite_client::{Connection, Mapping, response};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use crate::repositories::permissions::PermissionRepository;
use crate::repositories::rqlite::permissions::new_permission_repository;

pub struct Transaction {
    queries: Vec<Vec<serde_json::Value>>,
}

impl Transaction {
    pub fn push_sql_values<V>(&mut self, sql: &[V])
    where
        V: Into<serde_json::Value> + Clone,
    {
        self.queries
            .push(sql.iter().map(|v| v.clone().into()).collect());
    }

    fn clear(&mut self) {
        self.queries.clear();
    }
}

struct DbContextImpl {
    conn: Arc<Connection>,
    transaction: Rc<RefCell<Transaction>>,

    namespaces: Box<dyn NamespaceRepository>,
    values: Box<dyn ValueRepository>,
    permissions: Box<dyn PermissionRepository>,
}

pub fn new_context(conn: Arc<Connection>) -> Box<dyn DbContext> {
    let transaction = Rc::new(RefCell::new(Transaction { queries: vec![] }));

    Box::new(DbContextImpl {
        conn: conn.clone(),
        transaction: transaction.clone(),
        namespaces: new_namespace_repository(transaction.clone(), conn.clone()),
        values: new_value_repository(transaction.clone(), conn.clone()),
        permissions: new_permission_repository(transaction.clone(), conn.clone()),
    })
}

impl DbContext for DbContextImpl {
    fn save_changes(&mut self) -> Result<(), DbError> {
        let mut tx = self.transaction.borrow_mut();

        let mut query = self.conn.execute()
            .enable_transaction();

        for q in &tx.queries {
            query = query.push_sql_values(q);
        }

        let response_result = response::query::Query::from(query.request_run().unwrap());

        for mapping in response_result.into_iter() {
            match mapping {
                Mapping::Error(error) => {
                    return if error.error.contains("FOREIGN KEY constraint failed") {
                        Err(DbError::ForeignKeyViolation(error.error.clone()))
                    } else if error.error.contains("UNIQUE constraint failed") {
                        Err(DbError::UniqueViolation(error.error.clone()))
                    } else {
                        Err(DbError::Other(Box::new(error)))
                    }
                }
                _ => (),
            }
        }

        tx.clear();
        Ok(())
    }

    fn namespaces(&self) -> &dyn NamespaceRepository {
        self.namespaces.as_ref()
    }

    fn values(&self) -> &dyn ValueRepository {
        self.values.as_ref()
    }

    fn permissions(&self) -> &dyn PermissionRepository {
        self.permissions.as_ref()
    }
}
