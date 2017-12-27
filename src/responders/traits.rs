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
use std::any::Any;
use tiny_http;
use util;

/// Use traits to expose a Map<_, Any> safely
pub struct Traits {
}

impl responders::Responder for Traits {
    fn handle(&mut self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/traits");

        let mut di_map = DIMap::new();
        PathParts::put(&mut di_map, url_parts.path_components);
        UrlParams::put(&mut di_map, url_parts.query);
        util::success(&dispatch(&di_map, &di_map))
    }
}

fn dispatch<P: PathParts, Q: UrlParams>(parts: &P, query: &Q) -> String {
    format!("Traits {:?} {:?}", parts.get(), query.get())
}

struct DIMap {
    store: HashMap<String, Box<Any>>
}

impl DIMap {
    fn new() -> DIMap {
        DIMap { store: HashMap::new() }
    }
}

// These traits are functionally similar to Deref, but since they're traits and not types we can't
// use Deref, so users must explicitly call .get(). See https://stackoverflow.com/q/29256519/113632

trait PathParts {
    fn get(&self) -> &Vec<String>;

    fn put(&mut self, value: Vec<String>);
}
impl PathParts for DIMap {
    fn get(&self) -> &Vec<String> {
        self.store.get("PathParts").unwrap().downcast_ref::<Vec<String>>().unwrap()
    }
    fn put(&mut self, value: Vec<String>) {
        self.store.insert("PathParts".into(), Box::new(value) as Box<Any>);
    }
}

trait UrlParams {
    fn get(&self) -> &HashMap<String, String>;

    fn put(&mut self, value: HashMap<String, String>);
}


impl UrlParams for DIMap {
    fn get(&self) -> &HashMap<String, String> {
        self.store.get("UrlParams").unwrap().downcast_ref::<HashMap<String, String>>().unwrap()
    }

    fn put(&mut self, value: HashMap<String, String>) {
        self.store.insert("UrlParams".into(), Box::new(value) as Box<Any>);
    }
}
