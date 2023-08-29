use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use sqlx::PgPool;
use tracing::{error, info};

use crate::service::code::BindCode;

pub async fn send_mails(db: &PgPool, key: &str, from: &str, host: &str) {
    let codes = sqlx::query_as!(
        BindCode,
        "select id, account, email, code, status, created_at, updated_at from bind_code where status = 0 order by id desc limit 100",
    ).fetch_all(db).await;

    match codes {
        Ok(codes) => {
            for code in codes {
                let email: Message = Message::builder()
                    .from(from.parse().unwrap())
                    .to(code.email.parse().unwrap())
                    .subject(format!("Bind email code: {}", code.code))
                    .body("".to_string())
                    .unwrap();

                let creds: Credentials = Credentials::new(from.to_string(), key.to_string());
                let mailer: SmtpTransport = SmtpTransport::relay(&host)
                    .unwrap()
                    .credentials(creds)
                    .build();
                match mailer.send(&email) {
                    Ok(_) => {
                        let _ = sqlx::query!(
                            r#"Update bind_code set status = $1, updated_at = now() where id = $2"#,
                            1,
                            code.id,
                        )
                        .execute(db)
                        .await;
                        info!(target: "email", id = ?code.id, email = ?code.email, "send email success")
                    }
                    Err(err) => {
                        error!(target: "email", id = ?code.id, email = ?code.email, err = ?err, "send email")
                    }
                };
            }
        }
        Err(err) => error!(target: "email", ?err, "query codes error"),
    }
}
