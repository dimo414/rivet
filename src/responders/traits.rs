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
        let url_parts = util::strip_url_prefix(request.url(), "/trait");

        let di_map = DIMap::new(url_parts);
        util::success(&dispatch(&di_map, &di_map))
    }
}

fn dispatch<P: PathParts, Q: UrlParams>(parts: &P, query: &Q) -> String {
    format!("Trait {:?} {:?}", parts.get(), query.get())
}

struct DIMap {
    store: HashMap<String, Box<Any>>
}

impl DIMap {
    fn new(url_parts: util::UrlParts) -> DIMap {
        // TODO is there some way to use the values directly in the UrlParts struct?
        // Note .path_components() currently returns a Vec<&str>, but even if it's changed to return
        // &Vec<String> that doesn't solve the problem. Need a way to own the value from the struct.
        let list_copy: Vec<String> =
            url_parts.path_components().iter().map(|s| s.to_string()).collect();
        let map_copy: HashMap<String, String> =
            url_parts.query().iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();

        let mut store = HashMap::new();
        store.insert("PathParts".into(), Box::new(list_copy) as Box<Any>);
        store.insert("UrlParams".into(), Box::new(map_copy) as Box<Any>);
        DIMap { store }
    }
}

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
