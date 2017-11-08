extern crate tiny_http;

use responders;

/// Basic Responder implementation just demonstrating the API.
pub struct Raw {}

impl responders::Responder for Raw {
    fn handle(&self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        tiny_http::Response::from_string("Raw! ".to_string() + request.url()).boxed()
    }
}