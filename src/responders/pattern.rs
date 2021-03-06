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
    callback: fn(&regex::Captures, &HashMap<String, String>) -> String
}

impl Route {
    pub fn new(path: &str, callback: fn(&regex::Captures, &HashMap<String, String>) -> String) -> Route {
        Route { path: regex::Regex::new(&format!("^{}$", path)).unwrap(), callback }
    }
}

fn handle(url_captures: &regex::Captures, url_params: &HashMap<String, String>) -> String {
    format!("pattern!\nURL captures: {:?}\nQuery args: {:?}", url_captures, url_params)
}

fn handle_foo(_url_captures: &regex::Captures, url_params: &HashMap<String, String>) -> String {
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
                    let response = callback(&captures, url_parts.query());
                    return util::success(&response);
                }
                None => {}
            }
        }

        util::fail404("No matched pattern")
    }
}
