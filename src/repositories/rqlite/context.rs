use crate::repositories::context::DbContext;
use crate::repositories::namespaces::NamespaceRepository;
use crate::repositories::rqlite::namespaces::new_namespace_repository;
use rqlite_client::{Connection, Mapping, response};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use crate::repositories::rqlite::values::new_value_repository;
use crate::repositories::values::ValueRepository;

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
}

pub fn new_context(conn: Arc<Connection>) -> Box<dyn DbContext> {
    let transaction = Rc::new(RefCell::new(Transaction { queries: vec![] }));

    Box::new(DbContextImpl {
        conn: conn.clone(),
        transaction: transaction.clone(),
        namespaces: new_namespace_repository(transaction.clone(), conn.clone()),
        values: new_value_repository(transaction.clone()),
    })
}

impl DbContext for DbContextImpl {
    fn save_changes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut tx = self.transaction.borrow_mut();

        let mut query = self.conn.execute().enable_transaction();

        for q in &tx.queries {
            query = query.push_sql_values(q);
        }

        let response_result = response::query::Query::from(query.request_run().unwrap());

        match response_result.results().next() {
            Some(Mapping::Error(error)) => return Err(Box::new(error.clone())),
            _ => {},
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
}
