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
        self.constructors.insert(
            s.to_string(), 
            Box::new(value.construct()) as Box<Any>
        );
    }
    
    fn resolve<T: 'static>(&self, s: &str) -> T {
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

impl Constructors<BaseRef> for BaseRef {
    fn construct<'a>(self) -> Construct<'a, BaseRef> {
        Construct { build: Box::new(self) }
    }
}

impl Constructors<DepRef> for DepRef {
    fn construct<'a>(self) -> Construct<'a, DepRef> {
        Construct { build: Box::new(self) }
    }
}

trait Builder<T> {
    fn c(&self) -> T;
}

impl Builder<BaseRef> for BaseRef {
    fn c(&self) -> BaseRef {
        self.clone()
    }
}

impl Builder<DepRef> for DepRef {
    fn c(&self) -> DepRef {
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
  b: BaseRef
}

impl DepRef {
  fn new(b: BaseRef) -> DepRef {
    DepRef { b: b }
  }
}

fn main() {
    let mut container = Container::new();
    
    container.add("baseref", BaseRef::new());
    let test: BaseRef = container.resolve("baseref");
    container.add("depref", DepRef::new(test));
    
    let test2: BaseRef = container.resolve("baseref");
    println!("{}", test2.int);
}
