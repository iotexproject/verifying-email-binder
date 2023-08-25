use std::env;

use ethers::providers::{Http, Provider};
use sqlx::postgres::PgPoolOptions;
use verifying_email_binder::{
    server::handler::serve_http,
    service::{Context, HttpRpcHandler},
};

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .expect("could not connect to database");

    let provider = Provider::<Http>::try_from(env::var("RPC_URL").expect("RPC_URL must be set"))
        .expect("instance provider error");

    let context = Context {
        db,
        provider,
        guardian_address: env::var("RPC_URL").expect("RPC_URL must be set"),
    };

    sqlx::migrate!()
        .run(&context.db)
        .await
        .expect("migrate database error");

    let http = HttpRpcHandler::new(context);
    serve_http("127.0.0.1:3000".parse().unwrap(), http)
        .await
        .unwrap();
}
