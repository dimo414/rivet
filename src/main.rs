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

#[macro_use] extern crate lazy_static;
#[macro_use] mod macros;

extern crate regex;
extern crate tiny_http;

use std::collections::HashMap;
use tiny_http::{Server};

mod responders;
mod util;

/// Server entry point - starts up a web server and routes requests to the known responders.
///
/// This is essentially a meta-plugin layer, in order to support different plugin paridigms in the
/// same server. Each `Responder` is mapped to a URL prefix (e.g. all `/raw` URLs are routed to the
/// `Raw` responder) and if a request matches a registered prefix it's routed to the associated
/// `Responder` to compute a `Response`. Complex `Responder` implementations will generally then
/// implement an "actual" plugin model and provide a nicer API for processing individual requests.
///
/// For example `/nice-plugin/foo` and `/nice-plugin/bar` would both be routed to the `NicePlugin`
/// responder (assuming such a responder is installed) but each path might be handled by different
/// code paths registered with the `NicePlugin` responder.
fn main() {
    // Register responders here
    let responders = {
        let mut m: HashMap<String, Box<responders::Responder>> = HashMap::new();
        m.insert("".into(), Box::new(RootResponder {}));
        m.insert("closure".into(), Box::new(responders::closure::Closure {}));
        m.insert("factory".into(), Box::new(responders::factory::Factory::new()));
        m.insert("pattern".into(), Box::new(responders::pattern::Pattern {}));
        m.insert("raw".into(), Box::new(responders::raw::Raw {}));
        m.insert("stringly".into(), Box::new(responders::stringly::Stringly {}));
        m.insert("traits".into(), Box::new(responders::traits::Traits {}));
        m.insert("traits_macro".into(), Box::new(responders::traits_macro::TraitsMacro {}));
        m // now the map is immutable
    };

    // Start server
    // OSX prompts to permit cargo to listen on a port every time `cargo run` is called
    // https://apple.stackexchange.com/a/150711/69703 resolves this:
    //   sudo codesign --force --deep --sign - $(which cargo)
    let server = Server::http("0.0.0.0:8000").unwrap();
    println!("server started: http://localhost:8000");

    // Single-threaded server - tiny_http supports multi-threading, but it's not necessary for the
    // initial proof-of-concept
    for request in server.incoming_requests() {
        // TODO logging framework?
        print!("received {:?} request for url {:?}", request.method(), request.url());

        // Fallback shutdown mechanism in case Ctrl+C isn't propagated properly.
        // According to https://github.com/rust-lang/cargo/issues/2343 it should be, but at least on
        // my system it's not working - might be https://github.com/rust-lang/cargo/issues/4575
        if request.url() == "/quit" {
            let _ = request.respond(util::success("Shutting Down!"));
            println!();
            break;
        }

        // Lookup the right responder for the request
        let url_prefix = url_prefix(&request.url()).to_string();
        let response = match responders.get(&url_prefix) {
            Some(responder) => {
                if url_prefix.len() > 0 { print!(" - routed to {}", url_prefix); }
                responder.handle(&request)
            },
            _ => util::fail404("No responder found")
        };
        println!();

        // Note that respond takes ownership of request at this point (self vs. &self)
        let _ = request.respond(response); // ignore Result, it's a client-side error
    }
    // When `server` goes out of scope the server is shut down
}

/// Get the first section of a URL, effectively matching the pattern `/([^/]+)/.*`.
fn url_prefix(url: &str) -> &str {
    let without_slash = &url[1..];
    match without_slash.find(|c| c == '/' || c == '?') {
        Some(index) => &without_slash[..index],
        None => without_slash,
    }
}

/// A responder for the homepage (`/`)
struct RootResponder {}
impl responders::Responder for RootResponder {
    fn handle(&self, _request: &tiny_http::Request) -> tiny_http::ResponseBox {
        // TODO better names / clearer descriptions
        util::success_html(
            "<ul>
            <li><a href=\"/raw/foo/bar?baz\">Raw</a> - handle Request object directly</li>
            <li><a href=\"/stringly/foo/bar?baz\">Stringly</a> - pass in fixed request details</li>
            <li><a href=\"/pattern/foo/bar?baz\">Pattern</a> - route requests by regex patterns</li>
            <li><a href=\"/closure/both/bar?baz\">Closure</a> - route requests to user-specified closures</li>
            <li><a href=\"/factory/\">Factory</a> - DI pattern using generic factory</li>
            <li><a href=\"/traits/bar?baz\">Traits</a> - DI pattern providing some type safety via traits</li>
            <li><a href=\"/traits_macro/bar?baz\">Traits Macro</a> - same as above, but simplified by macros</li>
            </ul>"
        )
    }
}
