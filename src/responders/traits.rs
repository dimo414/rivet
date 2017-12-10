use responders;
use std::collections::HashMap;
use std::any::Any;
use tiny_http;
use util;

/// Use traits to expose a Map<Any> safely
pub struct Traits {
}

impl responders::Responder for Traits {
    fn handle(&self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/traits");

        let di_map = DIMap::new(url_parts);
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
    fn new(url_parts: util::UrlParts) -> DIMap {
        let mut store = HashMap::new();
        store.insert("PathParts".into(), Box::new(url_parts.path_components) as Box<Any>);
        store.insert("UrlParams".into(), Box::new(url_parts.query) as Box<Any>);
        DIMap { store }
    }
}

// These traits are functionally similar to Deref, but since they're traits and not types we can't
// use Deref, so users must explitly call .get(). See https://stackoverflow.com/q/29256519/113632

trait PathParts {
    fn get(&self) -> &Vec<String>;
}
impl PathParts for DIMap {
    fn get(&self) -> &Vec<String> {
        self.store.get("PathParts").unwrap().downcast_ref::<Vec<String>>().unwrap()
    }
}

trait UrlParams {
    fn get(&self) -> &HashMap<String, String>;
}


impl UrlParams for DIMap {
    fn get(&self) -> &HashMap<String, String> {
        self.store.get("UrlParams").unwrap().downcast_ref::<HashMap<String, String>>().unwrap()
    }
}
