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
use tiny_http;
use util;

/// Basic Responder implementation just demonstrating the API.
pub struct Raw {}

impl responders::Responder for Raw {
    fn handle(&self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        util::success(&format!("Raw! {}", util::strip_prefix(request.url(), "/raw")))
    }
}
