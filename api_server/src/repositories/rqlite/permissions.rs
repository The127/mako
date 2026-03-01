use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use rqlite_client::{response, Connection, Mapping};
use rqlite_client::response::mapping::Standard;
use crate::repositories::permissions::{Permission, PermissionRepository, PermissionType};
use crate::repositories::rqlite::context::Transaction;

struct PermissionModel {
    subject_id: String,
    path: String,
    permissions: String,
}

impl PermissionModel {
    fn scan(row: &Standard) -> Self {
        let mut idx = 0;

        let subject_id = row.value(0, idx).unwrap();
        idx += 1;

        let path = row.value(0, idx).unwrap();
        idx += 1;

        let permissions = row.value(0, idx).unwrap();

        PermissionModel {
            subject_id: subject_id.to_string(),
            path: path.to_string(),
            permissions: permissions.to_string(),
        }
    }
}

impl From<&Permission> for PermissionModel {
    fn from(permission: &Permission) -> Self {
        PermissionModel {
            subject_id: permission.subject_id(),
            path: permission.path(),
            permissions: permission.permissions()
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(","),
        }
    }
}

impl From<PermissionModel> for Permission {
    fn from(permission: PermissionModel) -> Self {
        let permissions = permission.permissions.split(",").map(|p| PermissionType::from_string(p).unwrap()).collect::<Vec<PermissionType>>();
        Permission::new(permission.subject_id, permission.path, permissions)
    }
}

pub fn new_permission_repository(
    transaction: Rc<RefCell<Transaction>>,
    conn: Arc<Connection>,
) -> Box<dyn PermissionRepository> {
    Box::new(PermissionRepositoryImpl { transaction, conn })
}

struct PermissionRepositoryImpl {
    transaction: Rc<RefCell<Transaction>>,
    conn: Arc<Connection>,
}

impl PermissionRepository for PermissionRepositoryImpl {
    fn insert(&self, permission: Permission) {
        let mapped = PermissionModel::from(&permission);

        let mut tx = self.transaction.borrow_mut();
        tx.push_sql_values(&[
            "
            insert into permissions (
                subject_id, path, permissions
            ) values (
                ?, ?, ?
            )",
            &mapped.subject_id,
            &mapped.path,
        ]);
    }

    fn get(&self, subject_id: &str, path: &str) -> Option<Permission> {
        let query = self.conn.query().push_sql_values(&[
            "
            select subject_id, path, permissions from permissions where subject_id = ? and path = ?
            ",
            subject_id,
            path,
        ]);

        let response_resul = response::query::Query::from(query.request_run().unwrap());

        match response_resul.results().next() {
            Some(Mapping::Standard(row)) => {
                if let Some(values) = &row.values {
                    log::info!("Permission {:?} found", values);
                    Some(PermissionModel::scan(&row).into())
                } else {
                    None
                }
            }
            Some(Mapping::Error(error)) => {
                log::error!("Error creating permission: {}", error);
                panic!("Error creating permission: {}", error);
            }
            _ => None,
        }
    }
}
