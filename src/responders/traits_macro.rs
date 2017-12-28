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

/// Same pattern as traits.rs, but using macros to reduce boilerplate
pub struct TraitsMacro {
}

binder!(DI);
binding!(DI, UrlParts, util::UrlParts);
provider!(DI, PathParts, Vec<String>, UrlParts, |d: &'a UrlParts| d.get().path_components());
provider!(DI, UrlParams, HashMap<String, String>, UrlParts, |d: &'a UrlParts| d.get().query());

impl responders::Responder for TraitsMacro {
    fn handle(&self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/traits_macro");

        let callback = dispatcher(&url_parts);

        let mut deps = DI::new();
        bind!(deps, UrlParts, url_parts);

        callback(&deps)
    }
}

fn dispatcher(url_parts: &util::UrlParts) -> Box<Fn(&DI) -> tiny_http::ResponseBox> {
    match url_parts.path_components().first() {
        Some(path) => match path.as_ref() {
            "path" => inject_http_success!(DI, paths_only, 1),
            "query" => inject_http_success!(DI, query_only, 1),
            "both" => inject_http_success!(DI, both, 2),
            "all" => inject_http_success!(DI, all, 3),
            _ => Box::new(|_deps|util::fail404("Not found")),
        }
        _ => inject_http_success!(DI, root, 0),
    }
}

fn root() -> String { "Try /path, /query, /both, or /all".into() }


fn paths_only<P: PathParts>(paths: &P) -> String {
    format!("Paths Only! {:?}", paths.get())
}

fn query_only<Q: UrlParams>(query: &Q) -> String {
    format!("Query Only! {:?}", query.get())
}

fn both<P: PathParts, Q: UrlParams>(parts: &P, query: &Q) -> String {
    format!("Paths: {:?} and Query: {:?}", parts.get(), query.get())
}

fn all<U: UrlParts, P: PathParts, Q: UrlParams>(url: &U, parts: &P, query: &Q) -> String {
    format!("URL: {}, Paths: {:?}, and Query: {:?}", url.get().path(), parts.get(), query.get())
}
