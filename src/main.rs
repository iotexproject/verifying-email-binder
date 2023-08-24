use std::env;

use sqlx::postgres::PgPoolOptions;
use verifying_email_binder::{server::handler::serve_http, service::HttpRpcHandler};

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .expect("could not connect to database");
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("migrate database error");

    let http = HttpRpcHandler::new(db);
    serve_http("127.0.0.1:3000".parse().unwrap(), http)
        .await
        .unwrap();
}
