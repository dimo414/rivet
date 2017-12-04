use responders;
use tiny_http;
use util::strip_prefix;

/// Basic Responder implementation just demonstrating the API.
pub struct Raw {}

impl responders::Responder for Raw {
    fn handle(&self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        tiny_http::Response::from_string(format!("Raw! {}", strip_prefix(request.url(), "/raw"))).boxed()
    }
}