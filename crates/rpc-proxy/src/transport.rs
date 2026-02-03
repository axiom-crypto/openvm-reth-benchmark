//! Custom transport layer that logs the full JSON-RPC request payload on failure.

use alloy::transports::{TransportError, TransportErrorKind, TransportFut};
use alloy_json_rpc::{RequestPacket, ResponsePacket};
use std::{
    fmt,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::error;

/// A layer that wraps a transport and logs the full request payload when an RPC call fails.
#[derive(Clone, Debug)]
pub struct LogOnErrorLayer;

impl<S> Layer<S> for LogOnErrorLayer {
    type Service = LogOnErrorService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LogOnErrorService { inner }
    }
}

/// A service that logs the full request payload when an RPC call fails.
#[derive(Clone)]
pub struct LogOnErrorService<S> {
    inner: S,
}

impl<S> fmt::Debug for LogOnErrorService<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogOnErrorService").finish_non_exhaustive()
    }
}

impl<S> Service<RequestPacket> for LogOnErrorService<S>
where
    S: Service<RequestPacket, Response = ResponsePacket, Error = TransportError>
        + Send
        + Sync
        + Clone
        + 'static,
    S::Future: Send,
{
    type Response = ResponsePacket;
    type Error = TransportError;
    type Future = TransportFut<'static>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: RequestPacket) -> Self::Future {
        let mut inner = self.inner.clone();

        // Serialize the request to JSON for logging
        let request_json = serde_json::to_string(&request)
            .unwrap_or_else(|e| format!("Failed to serialize request: {}", e));

        Box::pin(async move {
            let result = inner.call(request).await;

            if let Err(ref err) = result {
                error!(
                    error = %err,
                    request_payload = %request_json,
                    "RPC request failed. Full request payload logged for debugging."
                );
            } else if let Ok(ResponsePacket::Single(ref response)) = result {
                // Also log if the RPC returned an error response
                if let Some(ref rpc_error) = response.payload.as_error() {
                    error!(
                        rpc_error_code = rpc_error.code,
                        rpc_error_message = %rpc_error.message,
                        request_payload = %request_json,
                        "RPC returned error response. Full request payload logged for debugging."
                    );
                    // Convert to transport error so the caller knows it failed
                    return Err(TransportError::from(TransportErrorKind::custom_str(
                        &format!("RPC error {}: {}", rpc_error.code, rpc_error.message),
                    )));
                }
            } else if let Ok(ResponsePacket::Batch(ref responses)) = result {
                // Check batch responses for errors
                for response in responses {
                    if let Some(ref rpc_error) = response.payload.as_error() {
                        error!(
                            rpc_error_code = rpc_error.code,
                            rpc_error_message = %rpc_error.message,
                            request_payload = %request_json,
                            "RPC batch response contains error. Full request payload logged for debugging."
                        );
                    }
                }
            }

            result
        })
    }
}
