use responders;
use std::collections::HashMap;
use std::any::Any;
use tiny_http;
use util;

/// Constructs a "binder", a struct that can hold arbitrary types, installed via the bind! macro.
///   Usage: binder!(BinderTypeName)
///     BinderTypeName:  Name of the struct to define.
macro_rules! binder {
    ($store:ident) => {
        struct $store {
            store: HashMap<String, Box<Any>>
        }

        impl $store {
            fn new<F>(binding_closure: F) -> $store where
                F: FnOnce(&mut HashMap<String, Box<Any>>) -> () {
                let mut store = HashMap::new();
                binding_closure(&mut store);
                $store { store }
            }
        }
    }
}

/// Used inside the closure passed to a "binder" constructor to bind instances.
///   Usage: bind!(store, BindingTrait, Binding)
///     store:         Literal "store" - this is a leaky abstraction point
///     BindingTrait:  Trait which will provide Binding
///     Binding:       Instance to bind to the BindingTrait
macro_rules! bind {
    ($map:ident, $type:ident, $value:expr) => {
        // TODO validate $map doesn't already contain $type
        // TODO validate $value is of the appropriate type for $type
        $map.insert(stringify!($type).into(), Box::new($value) as Box<Any>);
    }
}

/// Registers a binding, creating a trait with the given name
///   Usage: binding!(BinderType, BindingTraitName, BindingType)
///     BinderType:        A binder type, created by binder!()
///     BindingTraitName:  Trait to create that will provide the given binding
///     BindingType:       Type that BindingTrait will provide
macro_rules! binding {
    ($store:ident, $name:ident, $ty:ty) => {
        trait $name { fn get(&self) -> &$ty; }

        impl $name for $store {
            fn get(&self) -> &$ty {
                match self.store.get(stringify!($name).into()) {
                    Some(dep) => dep.downcast_ref::<$ty>().unwrap(),
                    None => panic!("{} has no binding for {}!\n\t{:?}\n",
                         stringify!($store), stringify!($name), self.store)
                }
            }
        }
    }
}

/// Same pattern as traits.rs, but using macros to reduce boilerplate
pub struct TraitsMacro {
}

impl responders::Responder for TraitsMacro {
    fn handle(&self, request: &tiny_http::Request) -> tiny_http::ResponseBox {
        let url_parts = util::strip_url_prefix(request.url(), "/trait");

        let deps = DI::new(move |store| {
            bind!(store, PathParts, url_parts.path_components);
            bind!(store, UrlParams, url_parts.query);
        });

        util::success(&dispatch(&deps, &deps))
    }
}

fn dispatch<P: PathParts, Q: UrlParams>(parts: &P, query: &Q) -> String {
    format!("Trait {:?} {:?}", parts.get(), query.get())
}

binder!(DI);
binding!(DI, PathParts, Vec<String>);
binding!(DI, UrlParams, HashMap<String, String>);