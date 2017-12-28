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

/// Stringly-typed responder, treats URLs as strings, application logic must do parsing
// TODO does this example have anything that pattern doesn't? Should it be deleted?
pub struct Stringly {
}

impl responders::Responder for Stringly {
    fn handle(&self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/stringly");

        let response = respond(url_parts.path_components(), url_parts.query());
        util::success(&response)
    }
}

fn respond(url_components: &Vec<String>, url_params: &HashMap<String, String>) -> String {
    format!("stringly!\nURL parts: |{}|\nQuery args: {:?}", url_components.join("|"), url_params)
}
