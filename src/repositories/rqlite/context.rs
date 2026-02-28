use crate::repositories::context::DbContext;
use crate::repositories::namespaces::NamespaceRepository;
use crate::repositories::rqlite::namespaces::new_namespace_repository;
use rqlite_client::{Connection, Mapping, response};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

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
}

pub fn new_context(conn: Arc<Connection>) -> Box<dyn DbContext> {
    let transaction = Rc::new(RefCell::new(Transaction { queries: vec![] }));

    Box::new(DbContextImpl {
        conn,
        transaction: transaction.clone(),
        namespaces: new_namespace_repository(transaction.clone()),
    })
}

impl DbContext for DbContextImpl {
    fn save_changes(&mut self) {
        let mut tx = self.transaction.borrow_mut();

        let mut query = self.conn.execute().enable_transaction();

        for q in &tx.queries {
            query = query.push_sql_values(q);
        }

        let response_result = response::query::Query::from(query.request_run().unwrap());

        if let Some(Mapping::Standard(success)) = response_result.results().next() {
            let row = 0;
            let col = 0;
            if let Some(rows_found) = &success.value(row, col) {
                log::info!("Rows found: {}", rows_found);
            }
        } else if let Some(Mapping::Error(error)) = response_result.results().next() {
            log::error!("Error creating namespace: {}", error);
        }

        tx.clear()
    }

    fn namespaces(&self) -> &dyn NamespaceRepository {
        self.namespaces.as_ref()
    }
}
