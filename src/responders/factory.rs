use responders;
use tiny_http;
use util;
use std::collections::HashMap;
use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;

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
  container: Container,
}

impl Factory {
    pub fn new() -> Factory {
        let mut c = Container::new();
        let count = Rc::new(RefCell::new(0));
        c.add("count", count);
        Factory { container: c }
    }
}

impl responders::Responder for Factory {
    fn handle(&mut self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/factory");

        self.container.add("url_parts", url_parts);
        let count: Rc<RefCell<i32>> = self.container.resolve("count");
        *count.borrow_mut() += 1;
        util::success(&format!("Count {:?}", count))
    }
}

