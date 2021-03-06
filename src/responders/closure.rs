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

use responders;
use std::collections::HashMap;
use tiny_http;
use util;

/// Use closures to provide dynamic dependencies based on the caller
/// Inspired by https://github.com/KodrAus/rust-ioc/blob/master/factories
pub struct Closure {
}

impl responders::Responder for Closure {
    fn handle(&self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/closure");

        // This is essentially a manually-written DI pattern - while dense conceptually this function could
        // be generated by a script, macro, codegen, or other tool.
        let cb: Box<Fn() -> tiny_http::ResponseBox> = match url_parts.path_components().first() {
            Some(path) => match path.as_ref() {
                "path" => Box::new(|| util::success(&params_only(url_parts.path_components()))),
                "query" => Box::new(|| util::success(&query_only(url_parts.query()))),
                "both" => Box::new(|| util::success(&both(url_parts.path_components(), url_parts.query()))),
                _ => Box::new(|| util::fail404("Not found!"))
            },
            None => Box::new( || util::success(&root()))
        };

        cb()
    }
}

fn root() -> String { "Try /path, /query, or /both".into() }

fn params_only(params: &Vec<String>) -> String {
    format!("Params Only! {:?}", params)
}

fn query_only(query: &HashMap<String, String>) -> String {
    format!("Query Only! {:?}", query)
}

fn both(params: &Vec<String>, query: &HashMap<String, String>) -> String {
    format!("Params: {:?} and Query: {:?}", params, query)
}
