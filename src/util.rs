// Copyright 2017 Google LLC, Matthew Vilim
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use regex;
use std::collections::HashMap;
use tiny_http;

/// Common utilities that may be used across responders

pub fn success(response: &str) -> tiny_http::ResponseBox {
    tiny_http::Response::from_string(response).boxed()
}

pub fn success_html(response: &str) -> tiny_http::ResponseBox {
    tiny_http::Response::from_string(response)
        .with_header("Content-type: text/html".parse::<tiny_http::Header>().unwrap()).boxed()
}

pub fn fail404(response: &str) -> tiny_http::ResponseBox {
    tiny_http::Response::from_string(response)
        .with_status_code(tiny_http::StatusCode::from(404)).boxed()
}

lazy_static! {
    static ref URL_SPLIT: regex::Regex = regex::Regex::new(r"^([^?]*)(?:\?(.*))?$").unwrap();
    static ref PATH_SEGMENTS: regex::Regex = regex::Regex::new("/([^/]+)").unwrap();
    static ref QUERY_SEGMENT: regex::Regex = regex::Regex::new("^([^=]*)(?:=(.*))?$").unwrap();
}

/// Strips the given prefix from the front of s if s starts with that prefix.
/// If the prefix does not match panic - only strings known to start with the prefix should be
/// passed to this function.
pub fn strip_prefix<'a>(s: &'a str, expected_prefix: &str) -> &'a str {
    if s.starts_with(expected_prefix) {
        return &s[expected_prefix.len()..]
    }
    panic!("Expected {} to start with {}", s, expected_prefix)
}

pub fn strip_url_prefix(url: &str, expected_prefix: &str) -> UrlParts {
    let suffix = strip_prefix(url, expected_prefix);
    UrlParts::new(if suffix.is_empty() { "/" } else { suffix })
}

#[derive(Debug, Clone)]
pub struct UrlParts {
    pub path: String,
    pub path_components: Vec<String>,
    pub query: HashMap<String, String>,
    _private: () // https://github.com/rust-unofficial/patterns/blob/master/idioms/priv-extend.md
}

impl UrlParts {
    pub fn new(url: &str) -> UrlParts {
        if ! url.starts_with('/') { panic!("Invalid URL - must start with a /, was {}", url); }
        let url_split = URL_SPLIT.captures(url).unwrap();
        let path = &url_split[1];
        let query_str = url_split.get(2);

        let url_components: Vec<_> = PATH_SEGMENTS.captures_iter(path)
            .map(|cap| cap.get(1).unwrap().as_str().into())
            .collect();

        let url_query = match query_str {
            Some(query) => {
                query.as_str().split('&')
                    .map(|q| QUERY_SEGMENT.captures(q).unwrap())
                    .map(|cap|
                        (cap.get(1).unwrap().as_str().into(),
                         cap.get(2).map(|m| m.as_str()).unwrap_or("").into()))
                    .collect()
            },
            None => HashMap::new()
        };
        UrlParts {path: path.into(), path_components: url_components, query: url_query, _private:()}
    }

    #[allow(dead_code)]
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn path_components(&self) -> &Vec<String> { &self.path_components }

    pub fn query(&self) -> &HashMap<String, String> { &self.query }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_prefix_basic() {
        assert_eq!(strip_prefix("/foo/bar/baz", "/foo"), "/bar/baz");
    }

    #[test]
    #[should_panic(expected = "Expected /foo/bar/baz to start with /bar")]
    fn strip_prefix_panic() {
        strip_prefix("/foo/bar/baz", "/bar");
    }

    #[test]
    fn urlparts_basic() {
        let parts = UrlParts::new("/foo/bar/baz?bing&bang=boom");
        assert_eq!(parts.path, "/foo/bar/baz");
        assert_eq!(parts.path_components, vec!["foo", "bar", "baz"]);
        assert_eq!(parts.query.len(), 2);
        assert_eq!(parts.query.get("bing").unwrap(), "");
        assert_eq!(parts.query.get("bang").unwrap(), "boom");
    }

    #[test]
    fn urlparts_empty() {
        let parts = UrlParts::new("/");
        assert_eq!(parts.path, "/");
        assert_eq!(parts.path_components.len(), 0);
        assert_eq!(parts.query.len(), 0);
    }

    #[test]
    #[should_panic(expected = "Invalid URL")]
    fn urlparts_panic() {
        UrlParts::new("foo/bar/baz");
    }

    #[test]
    fn split_url_basic() {
        assert_eq!(strip_url_prefix("/foo", "/foo").path, "/");
        assert_eq!(strip_url_prefix("/foo/", "/foo").path, "/");
        assert_eq!(strip_url_prefix("/foo/bar", "/foo").path, "/bar");
    }
}
