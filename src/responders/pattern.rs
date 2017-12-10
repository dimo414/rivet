use regex;
use responders;
use std::collections::HashMap;
use tiny_http;
use util;

lazy_static! {
    // Add routes to this vector
    // Note that the order matters - the first matched pattern will be used
    static ref ROUTES: Vec<Route> = vec![
        Route::new("/foo/([^/]*)", handle_foo),
        Route::new("", handle)
    ];
}

struct Route {
    path: regex::Regex,
    callback: fn(regex::Captures, &HashMap<String, String>) -> String
}

impl Route {
    pub fn new(path: &str, callback: fn(regex::Captures, &HashMap<String, String>) -> String) -> Route {
        Route { path: regex::Regex::new(&format!("^{}$", path)).unwrap(), callback }
    }
}

fn handle(url_captures: regex::Captures, url_params: &HashMap<String, String>) -> String {
    format!("pattern!\nURL captures: {:?}\nQuery args: {:?}", url_captures, url_params)
}

fn handle_foo(_url_captures: regex::Captures, url_params: &HashMap<String, String>) -> String {
    format!("Foo!\nQuery args: {:?}", url_params)
}

/// Regex-based responder, routes requests to separate URLs to different functions
pub struct Pattern {
}

impl responders::Responder for Pattern {
    fn handle(&self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/pattern");

        for route in ROUTES.iter() {
            match route.path.captures(url_parts.path()) {
                Some(captures) => {
                    let callback = &route.callback;
                    let response = callback(captures, url_parts.query());
                    return util::success(&response);
                }
                None => {}
            }
        }

        util::fail404("No matched pattern")
    }
}