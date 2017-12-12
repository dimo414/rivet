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
    // NOTE: https://doc.rust-lang.org/error-index.html#method-has-no-receiver
    fn new() -> Self where Self: Sized;
    fn handle(& mut self, &tiny_http::Request) -> tiny_http::ResponseBox;
}
