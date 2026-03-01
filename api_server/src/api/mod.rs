use actix_web::web;

mod namespaces;
mod values;
mod errors;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(namespaces::create_namespace);
    cfg.service(namespaces::list_namespace_kvs);

    cfg.service(values::set_value);
    cfg.service(values::get_value);
}
