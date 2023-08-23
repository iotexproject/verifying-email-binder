use verifying_email_binder::{server::handler::serve_http, service::HttpRpcHandler};

#[tokio::main]
async fn main() {
    let http = HttpRpcHandler::new();
    serve_http("127.0.0.1:3000".parse().unwrap(), http)
        .await
        .unwrap();
}
