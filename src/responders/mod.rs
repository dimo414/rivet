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

pub mod closure;
pub mod factory;
pub mod pattern;
pub mod raw;
pub mod stringly;
pub mod traits;
pub mod traits_macro;

use tiny_http;

/// Our plugins implement this trait, accepting HTTP requests and returning HTTP responses.
///
/// They should in turn expose a more user-friendly API for how those requests should be handled.
/// For example, a plugin might support parsing data out of the URL path and provide those values
/// to the callback.
pub trait Responder {
    fn handle(& mut self, &tiny_http::Request) -> tiny_http::ResponseBox;
}
