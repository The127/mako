use crate::repositories::namespaces::NamespaceRepository;
use crate::repositories::values::ValueRepository;

pub trait DbContext {
    fn save_changes(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    fn namespaces(&self) -> &dyn NamespaceRepository;
    fn values(&self) -> &dyn ValueRepository;
}