use crate::repositories::namespaces::{Namespace, NamespaceRepository};
use crate::repositories::rqlite::context::Transaction;
use rqlite_client::response::mapping::Standard;
use rqlite_client::{response, Connection, Mapping};
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
    fn insert_if_not_exists(&self, namespace: Namespace) {
        let mapped = NamespaceModel::from(&namespace);

        let mut tx = self.transaction.borrow_mut();
        tx.push_sql_values(&[
            "
            insert into namespaces (
                path
            ) values (
                ?
            )
            on conflict (path) do nothing",
            &mapped.path,
        ]);
    }

    fn get(&self, path: &str) -> Result<Option<Namespace>, Box<dyn std::error::Error>> {
        let query = self.conn.query().push_sql_values(&[
            "
            select path from namespaces where path = ?
            ",
            path,
        ]);

        let response_result = response::query::Query::from(query.request_run().unwrap());

        match response_result.into_iter().next() {
            Some(Mapping::Standard(row)) => {
                if let Some(_) = &row.values {
                    Ok(Some(Namespace::from(NamespaceModel::scan(&row))))
                } else {
                    Ok(None)
                }
            }
            Some(Mapping::Error(error)) => {
                Err(Box::<dyn std::error::Error>::from(error))
            }
            _ => unreachable!(),
        }
    }

    fn exists(&self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.get(path)?.is_some())
    }

    fn delete_if_exists(&self, path: &str) {
        let mut tx = self.transaction.borrow_mut();
        tx.push_sql_values(&[
            "
            delete from namespaces where path = ?
            ",
            path,
        ]);
    }
}
