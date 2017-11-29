
extern crate tiny_http;
extern crate regex;

use responders;
use std::collections::HashMap;

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
        lazy_static! {
            static ref URL_SPLIT: regex::Regex = regex::Regex::new("^/pattern([^?]*)(?:\\?(.*))?$").unwrap();
            static ref QUERY_SEGMENT: regex::Regex = regex::Regex::new("^([^=]*)(?:=(.*))?$").unwrap();
        }

        let url_split = URL_SPLIT.captures(request.url()).unwrap();
        let path = &url_split[1];
        let query_str = url_split.get(2);
        println!("path: {} query: {:?}", path, query_str);

        let url_query = match query_str {
            Some(query) => {
                query.as_str().split('&')
                    .map(|q| QUERY_SEGMENT.captures(q).unwrap())
                    .map(|cap|
                        (cap.get(1).unwrap().as_str(),
                         cap.get(2).map(|m| m.as_str()).unwrap_or("")))
                    .collect()
            },
            None => HashMap::new()
        };

        for route in ROUTES_GEN.iter() {
            let url_pattern = &route.1;
            match url_pattern.captures(path) {
                Some(captures) => {
                    let callback = &route.2;
                    let response = callback(captures, url_query);
                    return tiny_http::Response::from_string((response)).boxed();
                }
                None => {}
            }
        }

        tiny_http::Response::from_string("No matched pattern")
            .with_status_code(tiny_http::StatusCode::from(404)).boxed()
    }
}