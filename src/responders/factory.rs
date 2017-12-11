use responders;
use tiny_http;
use util;
use std::collections::HashMap;
use std::any::Any;

struct Container {
    constructors: HashMap<String, Box<Any>>,
}

impl Container {
    fn new() -> Container {
        Container {
            constructors: HashMap::new(),
        }
    }
    
    fn add<T: Constructors<T> + 'static>(&mut self, s: &str, value: T) {
        self.constructors.insert(s.to_string(), Box::new(value.construct()) as Box<Any>);
    }
    
    fn resolve<T: Clone + 'static>(&self, s: &str) -> T {
        let item = self.constructors.get(s).unwrap();
        let construct = item.downcast_ref::<Construct<T>>().unwrap();
        construct.c()
    }
}

struct Construct<'a, T> {
    build: Box<Builder<T> + 'a>,
}

impl<'a, T> Construct<'a, T> {
    fn c(&self) -> T {
        self.build.c()
    }
}

trait Constructors<T> {
    fn construct<'a>(self) -> Construct<'a, T>;
}

impl<T: Clone + 'static> Constructors<T> for T {
    fn construct<'a>(self) -> Construct<'a, T> {
        Construct { build: Box::new(self) }
    }
}

trait Builder<T> {
    fn c(&self) -> T;
}

impl<T: Clone> Builder<T> for T {
    fn c(&self) -> T {
        self.clone()
    }
}

pub struct Factory {
}

impl responders::Responder for Factory {
    fn handle(&self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/factory");

        let mut container = Container::new();
        container.add("url_parts", url_parts);
        dispatcher(&container)
    }
}

fn dispatcher(container: &Container) -> tiny_http::ResponseBox {
    let u: util::UrlParts  = container.resolve("url_parts");
    let cb: Box<Fn() -> tiny_http::ResponseBox> = match u.path_components().get(0) {
        Some(path) => match path.as_ref() {
            "path" => Box::new(|| util::success(&params_only(u.path_components()))),
            "query" => Box::new(|| util::success(&query_only(u.query()))),
            "both" => Box::new(|| util::success(&both(u.path_components(), u.query()))),
            _ => Box::new(|| util::fail404("Not found!"))
        },
        None => Box::new( || util::success(&root()))
    };

    cb()
}

fn root() -> String { "Try /path, /query, or /both".into() }

fn params_only(params: &Vec<String>) -> String {
    format!("Params Only! {:?}", params)
}

fn query_only(query: &HashMap<String, String>) -> String {
    format!("Query Only! {:?}", query)
}

fn both(params: &Vec<String>, query: &HashMap<String, String>) -> String {
    format!("Params: {:?} and Query: {:?}", params, query)
}
