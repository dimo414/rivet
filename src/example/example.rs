// Copyright 2017 Google LLC, Matthew Vilim
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// based on https://github.com/Nercury/di-rs

use std::any::Any;
use std::collections::HashMap;

pub struct Scope<T> {
    pub parent: T,
    children: Vec<Box<Any>>,
}

pub trait Resolve<T> {
    fn resolve(self, s: &str, deps: &Dependencies) -> Scope<T>;
}

impl<T: Any> Resolve<T> for T {
    fn resolve(self, s: &str, deps: &Dependencies) -> Scope<T> {
        deps.run_constructors(s, self)
    }
}

pub struct Dependencies {
    constructors: HashMap<String, Vec<Box<Fn(&Dependencies, &Any) -> Box<Any>>>>,
}

impl Dependencies {
    pub fn new() -> Dependencies {
        Dependencies {
            constructors: HashMap::new()
        }
    }

    pub fn run_constructors<P: Any>(&self, s: &str, parent: P) -> Scope<P> {
        match self.constructors.get(s) {
            Some(list) => {
                let deps: Vec<_> = list.iter() .map(|construct| construct(&self, &parent)) .collect();
                Scope { parent: parent, children: deps }
            },
            None => Scope { parent: parent, children: vec![] },
        }
    }

    pub fn add<P, C, F>(&mut self, s: &str, constructor: F)
        where P: 'static + Any, C: 'static + Any, F: for<'r> Fn(&'r Dependencies, &P) -> C + 'static
    {
        match self.constructors.entry(s.to_string()) {
            std::collections::hash_map::Entry::Occupied(mut list) => {
                list.get_mut().push(box_constructor(constructor));
            },
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(vec![box_constructor(constructor)]);
            },
        };
    }
}

fn box_constructor<P, C, F>(constructor: F) -> Box<Fn(&Dependencies, &Any) -> Box<Any>>
    where F: for<'r> Fn(&'r Dependencies, &P) -> C + 'static, P: 'static + Any, C: 'static + Any
{
    Box::new(move |deps: &Dependencies, parent: &Any| -> Box<Any> {
        let concrete_parent = parent.downcast_ref::<P>().unwrap();
        let child = constructor(deps, concrete_parent);
        Box::new(child)
    })
}

struct BaseRef {
}

struct DepRef {
}

fn main() {
    let mut deps = Dependencies::new();

    deps.add("DepRef", |_deps, _parent: &DepRef| println!("DepRef created"));
    deps.add("BaseRef", |deps, _parent: &BaseRef| DepRef{}.resolve("DepRef", deps));

    BaseRef{}.resolve("BaseRef", &deps);
}
