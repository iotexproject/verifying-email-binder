pub mod serde_helpers;

use self::serde_helpers::sequence;
use tracing::trace;

use crate::{rpc::response::ResponseResult, server::handler::RpcHandler};

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum ApiRequest {
    #[serde(rename = "send_code", with = "sequence")]
    SendCode(String),
}

#[derive(Clone)]
pub struct HttpRpcHandler {}

impl HttpRpcHandler {
    pub fn new() -> Self {
        HttpRpcHandler {}
    }

    pub async fn execute(&self, request: ApiRequest) -> ResponseResult {
        trace!(target: "rpc::api", "executing eth request");
        match request {
            ApiRequest::SendCode(email) => ResponseResult::success(email),
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
