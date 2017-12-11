use responders;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::any::Any;
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
    fn new() -> TraitsMacro { TraitsMacro {} }

    fn handle(&mut self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/traits_macro");

        let deps = DI::new(move |binder| {
            // note that we can't bind anything directly from request, since DI would then own it
            bind!(binder, UrlParts, url_parts);
        });

        util::success(&dispatch(&deps, &deps))
    }
}

fn dispatch<P: PathParts, Q: UrlParams>(parts: &P, query: &Q) -> String {
    format!("Traits Macro {:?} {:?}", parts.get(), query.get())
}
