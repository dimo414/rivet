#[macro_use] extern crate lazy_static;
extern crate tiny_http;

use std::collections::HashMap;
use tiny_http::{Server, Response, StatusCode};

mod responders;

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
        let mut m: HashMap<&str, Box<responders::Responder>> = HashMap::new();
        m.insert("", Box::new(RootResponder {}));
        m.insert("pattern", Box::new(responders::pattern::Pattern {}));
        m.insert("raw", Box::new(responders::raw::Raw {}));
        m.insert("stringly", Box::new(responders::stringly::Stringly {}));
        m // now the map is immutable
    };

    // Start server
    let server = Server::http("0.0.0.0:8000").unwrap();
    println!("server started: http://localhost:8000");

    // Single-threaded server - tiny_http supports multi-threading, but it's not necessary for the
    // initial proof-of-concept
    for request in server.incoming_requests() {
        // Fallback shutdown mechanism in case Ctrl+C isn't propagated properly.
        // According to https://github.com/rust-lang/cargo/issues/2343 it should be, but at least on
        // my system it's not working - might be https://github.com/rust-lang/cargo/issues/4575
        if request.url() == "/quit" {
            let _ = request.respond(Response::from_string("Shutting Down!"));
            break;
        }

        // TODO logging framework?
        println!("received {:?} request for url {:?}", request.method(), request.url());

        // Lookup the right responder for the request
        let response = match responders.get(url_prefix(&request.url())) {
            Some(responder) => responder.handle(&request),
            _ => Response::from_string("No responder found")
                .with_status_code(StatusCode::from(404)).boxed()
        };

        // Note that respond takes ownership of request at this point (self vs. &self)
        let _ = request.respond(response); // ignore Result, it's a client-side error
    }
    // When `server` goes out of scope the server is shut down
}

/// Get the first section of a URL, effectively matching the pattern `/([^/]+)/.*`.
fn url_prefix(url: &str) -> &str {
    let without_slash = &url[1..];
    match without_slash.find('/') {
        Some(index) => &without_slash[..index],
        None => without_slash,
    }
}

/// A responder for the homepage (`/`)
// TODO output a list of installed responders, as links, for ease of navigation
struct RootResponder {}
impl responders::Responder for RootResponder {
    fn handle(&self, _request: &tiny_http::Request) -> tiny_http::ResponseBox {
        Response::from_string("hello world").boxed()
    }
}
