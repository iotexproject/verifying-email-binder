use crate::rpc::{error::RpcError, response::ResponseResult};
use serde::Serialize;
use tracing::error;

pub(crate) type Result<T> = std::result::Result<T, ServiceError>;

pub enum ServiceError {
    DatabaseError(String),
}

pub(crate) trait ToRpcResponseResult {
    fn to_rpc_result(self) -> ResponseResult;
}

pub fn to_rpc_result<T: Serialize>(val: T) -> ResponseResult {
    match serde_json::to_value(val) {
        Ok(success) => ResponseResult::Success(success),
        Err(err) => {
            error!("Failed serialize rpc response: {:?}", err);
            ResponseResult::error(RpcError::internal_error())
        }
    }
}

impl<T: Serialize> ToRpcResponseResult for Result<T> {
    fn to_rpc_result(self) -> ResponseResult {
        match self {
            Ok(val) => to_rpc_result(val),
            Err(err) => match err {
                ServiceError::DatabaseError(str) => RpcError::invalid_params(str),
            }
            .into(),
        }
    }
}
