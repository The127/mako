use actix_web::web;

mod namespaces;
mod values;
mod errors;
pub mod acl;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(namespaces::create_namespace);
    cfg.service(namespaces::list_namespace_kvs);
    cfg.service(namespaces::delete_namespace);
    cfg.service(namespaces::list_namespaces);

    cfg.service(values::set_value);
    cfg.service(values::get_value);
    cfg.service(values::delete_value);

    cfg.service(acl::set_acl);
    cfg.service(acl::get_acls);
    cfg.service(acl::get_acl);
    cfg.service(acl::delete_acl);
}
