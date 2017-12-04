use responders;
use std::collections::HashMap;
use tiny_http;
use util;

/// Stringly-typed responder, treats URLs as strings, application logic must do parsing
pub struct Stringly {
}

impl responders::Responder for Stringly {
    fn handle(&self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/stringly");

        let response = respond(url_parts.path_components(), url_parts.query());
        tiny_http::Response::from_string(response).boxed()
    }
}

fn respond(url_components: Vec<&str>, url_params: HashMap<&str, &str>) -> String {
    format!("stringly!\nURL parts: |{}|\nQuery args: {:?}", url_components.join("|"), url_params)
}
