use crate::repositories::namespaces::{Namespace, NamespaceRepository};
use crate::repositories::rqlite::context::Transaction;
use rqlite_client::response::mapping::Standard;
use rqlite_client::{Connection, Mapping, response};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

struct NamespaceModel {
    path: String,
}

impl NamespaceModel {
    fn scan(row: &Standard) -> Self {
        let path = row.value(0, 0).unwrap();

        NamespaceModel {
            path: path.to_string(),
        }
    }
}

impl From<&Namespace> for NamespaceModel {
    fn from(namespace: &Namespace) -> Self {
        NamespaceModel {
            path: namespace.path(),
        }
    }
}

impl From<NamespaceModel> for Namespace {
    fn from(namespace: NamespaceModel) -> Self {
        Namespace::new(namespace.path)
    }
}

pub fn new_namespace_repository(
    transaction: Rc<RefCell<Transaction>>,
    conn: Arc<Connection>,
) -> Box<dyn NamespaceRepository> {
    Box::new(NamespaceRepositoryImpl { transaction, conn })
}

struct NamespaceRepositoryImpl {
    transaction: Rc<RefCell<Transaction>>,
    conn: Arc<Connection>,
}

impl NamespaceRepository for NamespaceRepositoryImpl {
    fn insert(&self, namespace: Namespace) {
        let mapped = NamespaceModel::from(&namespace);

        let mut tx = self.transaction.borrow_mut();
        tx.push_sql_values(&[
            "
            insert into namespaces (
                path
            ) values (
                ?
            )",
            &mapped.path,
        ]);
    }

    fn get(&self, path: &str) -> Option<Namespace> {
        let query = self.conn.query().push_sql_values(&[
            "
            select path from namespaces where path = ?
            ",
            path,
        ]);

        let response_result = response::query::Query::from(query.request_run().unwrap());

        match response_result.results().next() {
            Some(Mapping::Standard(row)) => {
                if let Some(values) = &row.values {
                    log::info!("Namespace {:?} found", values);
                    Some(NamespaceModel::scan(&row).into())
                } else {
                    None
                }
            }
            Some(Mapping::Error(error)) => {
                log::error!("Error creating namespace: {}", error);
                panic!("Error creating namespace: {}", error);
            }
            _ => None,
        }
    }
}
