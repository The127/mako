use actix_web::web;

mod namespaces;
mod values;
mod errors;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(namespaces::create_namespace);
    cfg.service(namespaces::list_namespaces);

    cfg.service(values::create_value);
}
