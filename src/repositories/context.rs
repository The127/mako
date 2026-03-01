use crate::repositories::namespaces::NamespaceRepository;
use crate::repositories::values::ValueRepository;

pub enum DbError {
    ForeignKeyViolation(String),
    UniqueViolation(String),
    Other(Box<dyn std::error::Error>),
}

pub trait DbContext {
    fn save_changes(&mut self) -> Result<(), DbError>;

    fn namespaces(&self) -> &dyn NamespaceRepository;
    fn values(&self) -> &dyn ValueRepository;
}