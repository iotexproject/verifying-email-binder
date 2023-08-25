use chrono::{DateTime, Utc};
use rand::Rng;
use sqlx::PgPool;

use crate::service::error::Result;

#[derive(Debug)]
pub struct BindCode {
    pub id: i32,
    pub account: String,
    pub email: String,
    pub code: String,
    pub status: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub async fn generate_code(db: &PgPool, account: String, email: String) -> Result<String> {
    let codes = sqlx::query_as!(
        BindCode,
        "select id, account, email, code, status, created_at, updated_at from bind_code where account = $1 and email = $2 order by id desc limit 1",
        &account, &email
    ).fetch_all(db).await?;

    if codes.len() > 0
        && codes[0].status < 2
        && codes[0].created_at.timestamp() + 300 > chrono::Local::now().timestamp()
    {
        return Ok("Success".to_string());
    }

    let code = {
        let mut rng = rand::thread_rng();
        rng.gen_range(100000..999999).to_string()
    };

    let _ = sqlx::query!(
        r#"INSERT INTO bind_code(account, email, code, status) VALUES ($1, $2, $3, $4)"#,
        &account,
        &email,
        &code,
        0
    )
    .execute(db)
    .await?;
    Ok("Success".to_string())
}
