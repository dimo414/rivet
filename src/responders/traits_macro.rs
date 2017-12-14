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
    fn handle(&mut self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/traits_macro");

        let callback = dispatcher(&url_parts);

        let deps = DI::new(move |binder| {
            // note that we can't bind anything directly from request, since DI would then own it
            bind!(binder, UrlParts, url_parts);
        });

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
