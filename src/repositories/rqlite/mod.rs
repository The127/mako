use std::sync::Arc;
use rqlite_client::Connection;
use crate::repositories::context::DbContext;

mod namespaces;
mod context;

pub fn new_context(conn: Arc<Connection>) -> Box<dyn DbContext> {
    context::new_context(conn)
}