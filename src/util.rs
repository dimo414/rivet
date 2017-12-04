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
    static ref PATH_SEGMENTS: regex::Regex = regex::Regex::new("/([^/]*)").unwrap();
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
    UrlParts::new(strip_prefix(url, expected_prefix))
}

pub struct UrlParts {
    path: String,
    path_components: Vec<String>,
    query: HashMap<String, String>
}

impl UrlParts {
    pub fn new(url: &str) -> UrlParts {
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
        UrlParts {path: path.into(), path_components: url_components, query: url_query}
    }

    #[allow(dead_code)]
    pub fn path(&self) -> &str {
        &self.path
    }

    // TODO probably better to fix the Responder::handle method to take Vec<String>
    pub fn path_components(&self) -> Vec<&str> {
        // https://stackoverflow.com/a/33217302/113632
        self.path_components.iter().map(AsRef::as_ref).collect()
    }

    // TODO probably better to fix the Responder::handle method to take HashMap<String, String>
    pub fn query(&self) -> HashMap<&str, &str> {
        self.query.iter().map(|(k, v)| (k.as_ref(), v.as_ref())).collect()
    }
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
    fn split_url_basic() {
        let parts = UrlParts::new("/foo/bar/baz?bing&bang=boom");
        assert_eq!(parts.path, "/foo/bar/baz");
        assert_eq!(parts.path_components, vec!["foo", "bar", "baz"]);
        assert_eq!(parts.query.len(), 2);
//        assert_eq!(parts.query.get("bing").unwrap(), "");
//        assert_eq!(parts.query.get("bang").unwrap(), "boom");
    }

    #[test]
    fn split_url_empty() {
        let parts = UrlParts::new("");
        assert_eq!(parts.path, "");
        assert_eq!(parts.path_components.len(), 0);
        assert_eq!(parts.query.len(), 0);
    }
}