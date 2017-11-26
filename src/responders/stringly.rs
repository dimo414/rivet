
extern crate tiny_http;
extern crate regex;

use responders;
use std::collections::HashMap;

/// Stringly-typed responder, treats URLs as strings, application logic must do parsing
pub struct Stringly {
}

impl responders::Responder for Stringly {
    fn handle(&self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        lazy_static! {
            static ref URL_SPLIT: regex::Regex = regex::Regex::new("^/stringly([^?]*)(?:\\?(.*))?$").unwrap();
            static ref PATH_SEGMENTS: regex::Regex = regex::Regex::new("/([^/]*)").unwrap();
            static ref QUERY_SEGMENT: regex::Regex = regex::Regex::new("^([^=]*)(?:=(.*))?$").unwrap();
        }

        let url_split = URL_SPLIT.captures(request.url()).unwrap();
        let path = &url_split[1];
        let query_str = url_split.get(2);
        println!("path: {} query: {:?}", path, query_str);

        let url_components: Vec<_> = PATH_SEGMENTS.captures_iter(path)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect();

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

        let response = respond(url_components, url_query);
        tiny_http::Response::from_string(response).boxed()
    }
}

fn respond(url_components: Vec<&str>, get_params: HashMap<&str, &str>) -> String {
    format!("stringly!\nURL parts: |{}|\nQuery args: {:?}", url_components.join("|"), get_params)
}
