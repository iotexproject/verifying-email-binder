use sqlx::PgPool;

use crate::service::{code::BindCode, error::Result};

use super::error::ServiceError;

pub async fn verify_code(db: &PgPool, email: String, code: String) -> Result<String> {
    let codes = sqlx::query_as!(
        BindCode,
        "select id, code, email, status, created_at, updated_at from bind_code where email = $1 and code = $2 and status = $3 order by id desc limit 1",
        &email, &code, 1,
    ).fetch_all(db).await?;

    if codes.len() == 0 {
        return Err(ServiceError::InvalidRequest("error code".to_string()));
    }
    Ok("Success".to_string())
}
