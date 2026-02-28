use actix_web::web;

mod namespaces;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(namespaces::create_namespace);
}
