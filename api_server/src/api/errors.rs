use actix_web::error::{ErrorConflict, ErrorInternalServerError, ErrorNotFound};
use crate::repositories::DbError;

impl From<DbError> for actix_web::Error {
    fn from(e: DbError) -> Self {
        match e {
            DbError::ForeignKeyViolation(msg) => ErrorNotFound(msg),
            DbError::UniqueViolation(msg) => ErrorConflict(msg),
            DbError::Other(e) => {
                log::error!("{:?}", e);
                ErrorInternalServerError("Internal server error".to_string())
            },
        }
    }
}