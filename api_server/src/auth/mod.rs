use crate::extractors::auth::AuthUser;
use crate::repositories::DbContext;
use crate::repositories::permissions::PermissionType;

pub struct OidcConfiguration {
    pub admin_role: String,
    pub writer_role: String,
    pub reader_role: String,
}

pub enum OperationType {
    Read,
    Write,
    Admin,
}

pub fn has_access(
    user: &AuthUser,
    oidc_config: &OidcConfiguration,
    operation_type: OperationType,
) -> bool
{
    match user {
        AuthUser::Admin => true,
        AuthUser::Oidc { sub, roles } => {
            if roles.contains(&oidc_config.admin_role) {
                true
            } else {
                match operation_type {
                    OperationType::Read => roles.contains(&oidc_config.reader_role),
                    OperationType::Write => roles.contains(&oidc_config.writer_role),
                    OperationType::Admin => false,
                }
            }
        }
        AuthUser::Anonymous => false,
    }
}

pub fn has_access_to_value(
    ctx: &Box<dyn DbContext>,
    path: &str,
    user: &AuthUser,
    oidc_config: &OidcConfiguration,
    operation_type: OperationType,
) -> Result<bool, Box<dyn std::error::Error>> {
    if has_access(user, oidc_config, OperationType::Read) {
        match user {
            AuthUser::Admin => Ok(true),
            AuthUser::Oidc { sub, roles } => {
                let permission = ctx.permissions().get(sub, path)?;
                match permission {
                    Some(permission) => {
                        match operation_type {
                            OperationType::Read => Ok(permission.has_permission(PermissionType::Read)),
                            OperationType::Write => Ok(permission.has_permission(PermissionType::Write)),
                            OperationType::Admin => Ok(false),
                        }
                    },
                    None => Ok(false),
                }
            },
            AuthUser::Anonymous => Ok( false),
        }
    } else {
        Ok(false)
    }
}
