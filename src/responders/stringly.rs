use responders;
use std::collections::HashMap;
use tiny_http;
use util;

/// Stringly-typed responder, treats URLs as strings, application logic must do parsing
// TODO does this example have anything that pattern doesn't? Should it be deleted?
pub struct Stringly {
}

impl responders::Responder for Stringly {
    fn new() -> Stringly { Stringly {} }

    fn handle(& mut self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/stringly");

        let response = respond(url_parts.path_components(), url_parts.query());
        util::success(&response)
    }
}

fn respond(url_components: &Vec<String>, url_params: &HashMap<String, String>) -> String {
    format!("stringly!\nURL parts: |{}|\nQuery args: {:?}", url_components.join("|"), url_params)
}
