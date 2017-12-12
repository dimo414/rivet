/// A set of macros that provide a basic Dependency Injection pattern
/// TODO these macros rely on HashMap, hash_map::Entry, and Any being in-scope, which is tedious

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
///     Binder:        The argument to the binder's closure constructor
///     BindingTrait:  Trait which will provide Binding
///     Binding:       Instance to bind to the BindingTrait
macro_rules! bind {
    ($map:ident, $bnd:ident, $value:expr) => {
        // TODO validate $value is of the appropriate type for $bnd
        match $map.entry(stringify!($bnd).into()) {
            Entry::Vacant(entry) => entry.insert(Box::new($value) as Box<Any>),
            Entry::Occupied(_) =>
                // TODO include the existing binding in the error
                panic!("Conflicting binding for {}, cannot bind to {:?}", stringify!($bnd),  $value)
        }
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
                    Some(dep) => { match dep.downcast_ref::<$ty>() {
                        Some(dep) => dep,
                        None => panic!("Could not downcast {} to {} - wrong binding! type?",
                            stringify!($name), stringify!($ty))
                    }
                    },
                    None => panic!("{} has no binding for {}!\n\t{:?}\n",
                         stringify!($store), stringify!($name), self.store.keys())
                }
            }
        }
    }
}

// Registers a provider of a binding, introducing a recursive dependency on another binding
// Note this can only provide references, not owned types (because the closure would be the owner,
// and it goes out of scope upon returning).
//   Usage: provider!(BinderType, ProviderTraitName, ProviderType, DependantTrait, Closure)
///     BinderType:         A binder type, created by binder!()
///     ProviderTraitName:  Trait to create that will provide the given binding
///     ProviderType:       Type that ProviderTrait will provide
///     DependantTrait:     Binding trait that the provider depends on
///     Closure:            A closure of the form |d: &'a DependantTrait| ... that returns a
///                         reference to an value of ProviderType
/// TODO can closure signature be simplified?
macro_rules! provider {
    ($store:ident, $name:ident, $ty:ty, $dep:ty, $provider_fn:expr) => {
        trait $name { fn get(&self) -> &$ty; }

        impl $name for $store {
            fn get<'a>(&'a self) -> &$ty {
                &$provider_fn(self as &$dep)
            }
        }
    }
}

// Invokes a func with n repetitions of the given argument
/// See also http://danielkeep.github.io/tlborm/book/pat-push-down-accumulation.html, but I don't
/// think that pattern makes this use-case much cleaner. And
/// https://stackoverflow.com/q/33173235/113632.
/// Also posted to https://stackoverflow.com/q/47767910/113632
macro_rules! call_n {
    ($func:ident, $arg:expr, 0) => { $func() };
    ($func:ident, $arg:expr, 1) => { $func($arg) };
    ($func:ident, $arg:expr, 2) => { $func($arg, $arg) };
    ($func:ident, $arg:expr, 3) => { $func($arg, $arg, $arg) };
}

/// Support arbitrary injections by wrapping a call to the given func in a closure that takes a
/// BinderType and passes it to all parameters of the function.
///   Usage: inject(BinderType, InjectedFunction, NumArguments
///     BinderType:        A binder type, created by binder!()
///     InjectedFunction:  A function that takes 0 or more arguments, all of BinderType traits
///     NumArguments:      The number of arguments the function takes
#[allow(unused_macros)]
macro_rules! inject {
    ($store:ident, $func:ident, $num_args:tt) => {
        |_deps: &$store| util::success(&call_n!($func, _deps, $num_args))
    };
}
/// Same as inject!, but the closure is Boxed
#[allow(unused_macros)]
macro_rules! inject_box {
    ($store:ident, $func:ident, $num_args:tt) => { Box::new(inject!($store, $func, $num_args)) }
}
/// Same as inject!, but transforms the result into a ResponseBox too
macro_rules! inject_http_success {
    ($store:ident, $func:ident, $num_args:tt) => {
        Box::new(|_deps: &$store| util::success(&call_n!($func, _deps, $num_args)))
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::collections::hash_map::Entry;
    use std::any::Any;

    //use super::*;

    binder!(MyDeps);
    binding!(MyDeps, MyBinding, String);

    #[test]
    fn basic_di() {
        let deps = MyDeps::new(|binder| {
            bind!(binder, MyBinding, "FooBar".to_string());
        });

        let my_binding: &MyBinding = &deps;
        assert_eq!(my_binding.get(), "FooBar");
    }

    // TODO more tests
}