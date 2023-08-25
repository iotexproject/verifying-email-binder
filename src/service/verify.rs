use sqlx::PgPool;

use crate::service::{code::BindCode, error::Result};

use super::error::ServiceError;

pub async fn verify_code(
    db: &PgPool,
    account: String,
    email: String,
    code: String,
) -> Result<String> {
    let codes = sqlx::query_as!(
        BindCode,
        "select id, account, email, code, status, created_at, updated_at from bind_code where account = $1 and email = $2 and code = $3 and status = $4 order by id desc limit 1",
        &account, &email, &code, 1,
    ).fetch_all(db).await?;

    if codes.len() == 0 {
        return Err(ServiceError::InvalidRequest("error code".to_string()));
    }
    Ok("Success".to_string())
}
