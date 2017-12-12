use responders;
use tiny_http;
use util;

/// Basic Responder implementation just demonstrating the API.
pub struct Raw {}

impl responders::Responder for Raw {
    fn handle(& mut self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        util::success(&format!("Raw! {}", util::strip_prefix(request.url(), "/raw")))
    }
}
