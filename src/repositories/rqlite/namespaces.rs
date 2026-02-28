use crate::repositories::namespaces::{Namespace, NamespaceRepository};
use crate::repositories::rqlite::context::Transaction;
use std::cell::RefCell;
use std::rc::Rc;

struct NamespaceModel {
    path: String,
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

pub fn new_namespace_repository<'a>(
    transaction: Rc<RefCell<Transaction>>,
) -> Box<dyn NamespaceRepository> {
    Box::new(NamespaceRepositoryImpl { transaction })
}

struct NamespaceRepositoryImpl {
    transaction: Rc<RefCell<Transaction>>,
}

impl NamespaceRepository for NamespaceRepositoryImpl {
    fn insert(&self, namespace: Namespace) {
        let mapped = NamespaceModel::from(&namespace);

        let mut tx = self.transaction.borrow_mut();
        tx.push_sql_values(&["
            insert into namespaces (
                path
            ) values (
                ?
            )",
            &mapped.path,
        ]);
    }
}
