pub mod code;
pub mod error;
pub mod serde_helpers;

use sqlx::PgPool;
use tracing::trace;

use self::error::ToRpcResponseResult;
use self::serde_helpers::sequence;
use crate::{rpc::response::ResponseResult, server::handler::RpcHandler};

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum ApiRequest {
    #[serde(rename = "send_code", with = "sequence")]
    SendCode(String),
}

#[derive(Clone)]
pub struct HttpRpcHandler {
    db: PgPool,
}

impl HttpRpcHandler {
    pub fn new(db: PgPool) -> Self {
        HttpRpcHandler { db }
    }

    pub async fn execute(&self, request: ApiRequest) -> ResponseResult {
        trace!(target: "rpc::api", "executing eth request");
        match request {
            ApiRequest::SendCode(email) => {
                code::generate_code(&self.db, email).await.to_rpc_result()
            }
        }
    }
}

#[async_trait::async_trait]
impl RpcHandler for HttpRpcHandler {
    type Request = ApiRequest;

    async fn on_request(&self, request: Self::Request) -> ResponseResult {
        self.execute(request).await
    }
}
