use crate::repositories::permissions::{Permission, PermissionRepository, PermissionType};
use crate::repositories::rqlite::context::Transaction;
use rqlite_client::{response, Connection, Mapping};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

struct PermissionModel {
    subject_id: String,
    path: String,
    permissions: String,
}

impl PermissionModel {
    fn scan(row: &Vec<serde_json::Value>) -> Self {
        let subject_id = row[0].as_str().unwrap();
        let path = row[1].as_str().unwrap();
        let permissions = row[2].as_str().unwrap();

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
            permissions: permission
                .permissions()
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(","),
        }
    }
}

impl From<PermissionModel> for Permission {
    fn from(permission: PermissionModel) -> Self {
        let permissions = permission
            .permissions
            .split(",")
            .map(|p| PermissionType::from_string(p).unwrap())
            .collect::<Vec<PermissionType>>();
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
    fn set(&self, permission: Permission) {
        let mapped = PermissionModel::from(&permission);

        let mut tx = self.transaction.borrow_mut();
        tx.push_sql_values(&[
            "
            insert into permissions (
                subject_id, path, permissions
            ) values (
                ?, ?, ?
            )
            on conflict (subject_id, path)
            do update set
            permissions = excluded.permissions",
            &mapped.subject_id,
            &mapped.path,
            &mapped.permissions,
        ]);
    }

    fn get(
        &self,
        subject_id: &str,
        path: &str,
    ) -> Result<Option<Permission>, Box<dyn std::error::Error>> {
        let query = self.conn.query().push_sql_values(&[
            "
            select subject_id, path, permissions from permissions where subject_id = ? and path = ?
            ",
            subject_id,
            path,
        ]);

        let response_result = response::query::Query::from(query.request_run().unwrap());

        match response_result.into_iter().next() {
            Some(Mapping::Standard(standard)) => {
                if let Some(mapping) = &standard.values {
                    if let Some(row) = mapping.first() {
                        Ok(Some(Permission::from(PermissionModel::scan(row))))
                    }else{
                        Ok(None)
                    }
                }else{
                    Ok(None)
                }
            }
            Some(Mapping::Error(error)) => {
                Err(Box::<dyn std::error::Error>::from(error))
            }
            _ => Err(Box::<dyn std::error::Error>::from("Unexpected response format"))
        }
    }

    fn list(&self, path: &str) -> Result<Vec<Permission>, Box<dyn std::error::Error>> {
        let query = self.conn.query().push_sql_values(&[
            "
            select subject_id, path, permissions from permissions where path = ?
            ",
            path,
        ]);

        let response_result = response::query::Query::from(query.request_run().unwrap());

        match response_result.into_iter().next() {
            Some(Mapping::Standard(standard)) => {
                if let Some(mapping) = &standard.values {
                    Ok(mapping.iter().map(|row| Permission::from(PermissionModel::scan(row))).collect())
                }else{
                    Ok(Vec::new())
                }
            }
            Some(Mapping::Error(err)) => Err(Box::<dyn std::error::Error>::from(err)),
            _ => unreachable!(),
        }
    }

    fn delete(&self, subject_id: &str, path: &str) {
        let mut tx = self.transaction.borrow_mut();
        tx.push_sql_values(&[
            "
            delete from permissions where subject_id = ? and path = ?
            ",
            subject_id,
            path,
        ]);
    }
}
