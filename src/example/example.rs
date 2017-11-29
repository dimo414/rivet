use std::collections::HashMap;
use std::any::Any;
use std::rc::Rc;

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

#[derive(Clone)]
struct BaseRef {
  int: i32
}

impl BaseRef {
  fn new() -> BaseRef {
    BaseRef { int: 1 }
  }
}

#[derive(Clone)]
struct DepRef {
  b: Rc<BaseRef>
}

impl DepRef {
  fn new(b: Rc<BaseRef>) -> DepRef {
    DepRef { b: b.clone() }
  }
}

fn main() {
    let mut container = Container::new();
    
    container.add("shared", Rc::new(BaseRef::new()));
    let base: Rc<BaseRef> = container.resolve("shared");
    container.add("depref1", DepRef::new(base));
    let base: Rc<BaseRef> = container.resolve("shared");
    container.add("depref2", DepRef::new(base));
    
    let dep1: DepRef = container.resolve("depref1");
    let dep2: DepRef = container.resolve("depref2");
    println!("{}",  dep1.b.int);
    println!("{}",  dep2.b.int);
}
