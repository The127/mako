use crate::repositories::namespaces::NamespaceRepository;

pub trait DbContext {
    fn save_changes(&mut self);

    fn namespaces(&self) -> &dyn NamespaceRepository;
}