use std::collections::HashMap;
use std::any::Any;

struct Container {
  contain: HashMap<String, Box<(FnMut(Container) -> Box<Any>)>>
}

impl Container {
  fn new() -> Container {
    Container { contain: HashMap::new() }
  }

  fn add(&mut self, n: String, f: Box<(FnMut(Container) -> Box<Any>)>) {
    self.contain.insert(n, f);
  }

  fn resolve<T>(self, n: &String) {
    self.contain.get(n).unwrap()(self);
  }
}

struct BaseRef {
  int: i32
}

impl BaseRef {
  fn new() -> BaseRef {
    BaseRef { int: 1 }
  }
}

struct DepRef {
  b: BaseRef
}

impl DepRef {
  fn new(b: BaseRef) -> DepRef {
    DepRef { b: b }
  }
}

fn main() {
  let mut c = Container::new();
  c.add("baseref".to_string(), Box::new(|c| {
    BaseRef::new();
  }));
  c.add("depref".to_string(), |c| {
    DepRef::new(c.resolve("baseref".to_string()).downcast_ref::<BaseRef>().unwrap());
  });

}
