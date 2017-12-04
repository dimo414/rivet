use regex;
use responders;
use std::collections::HashMap;
use tiny_http;
use util;

lazy_static! {
    // Add routes to this vector
    // Note that the order matters - the first matched pattern will be used
    static ref ROUTES: Vec<(&'static str, fn(regex::Captures, HashMap<&str, &str>) -> String)> = vec![
        // TODO expand this a little, demo responding with different functions
        ("/foo/([^/]*)", handle),
        ("", handle)
    ];
    // Computed vector containing compiled patterns
    // Ideally the ROUTES vector could just be this directly, but the types don't seem to work out
    // without the intermediate variable
    static ref ROUTES_GEN: Vec<(&'static str, regex::Regex, fn(regex::Captures, HashMap<&str, &str>) -> String)> =
        ROUTES.iter().map(|t| (t.0, regex::Regex::new(&format!("^{}$", t.0)).unwrap(), t.1)).collect();
}

fn handle(url_captures: regex::Captures, url_params: HashMap<&str, &str>) -> String {
    format!("pattern!\nURL captures: {:?}\nQuery args: {:?}", url_captures, url_params)
}

/// Regex-based responder, routes requests to separate URLs to different functions
pub struct Pattern {
}

impl responders::Responder for Pattern {
    fn handle(&self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/pattern");

        for route in ROUTES_GEN.iter() {
            let url_pattern = &route.1;
            match url_pattern.captures(url_parts.path()) {
                Some(captures) => {
                    let callback = &route.2;
                    let response = callback(captures, url_parts.query());
                    return tiny_http::Response::from_string((response)).boxed();
                }
                None => {}
            }
        }

        tiny_http::Response::from_string("No matched pattern")
            .with_status_code(tiny_http::StatusCode::from(404)).boxed()
    }
}