/// A set of macros that provide a basic Dependency Injection pattern
/// Always fully-qualify imports here so callers don't need to add unnecessary use statements
/// See also https://doc.rust-lang.org/book/first-edition/ufcs.html wrt invoking traits

/// Constructs a "binder", a struct that can hold arbitrary types, installed via the bind! macro.
///   Usage: binder!(BinderTypeName)
///     BinderTypeName:  Name of the struct to define.
macro_rules! binder {
    ($store:ident) => {
        struct $store {
            store: ::std::collections::HashMap<String, Box<::std::any::Any>>
        }

        impl $store {
            fn new() -> $store {
                $store { store: ::std::collections::HashMap::new() }
            }
        }
    }
}

/// Binds a value to to a binder instance - effectively just a wrapper for
///   BindingTrait::put(&mut binder, value)
/// but can be used for consistency with the other macro APIs
///   Usage: bind!(store, BindingTrait, Binding)
///     BinderInstance:  A Binder instance, where the binding will be stored
///     BindingTrait:    Trait which will provide Binding
///     Binding:         Instance to bind to the BindingTrait
macro_rules! bind {
    ($map:ident, $bnd:ident, $value:expr) => {
        $bnd::put(&mut $map, $value);
    }
}

/// Registers a binding, creating a trait with the given name
///   Usage: binding!(BinderType, BindingTraitName, BindingType)
///     BinderType:        A binder type, created by binder!()
///     BindingTraitName:  Trait to create that will provide the given binding
///     BindingType:       Type that BindingTrait will provide
macro_rules! binding {
    ($store:ident, $name:ident, $ty:ty) => {
        trait $name { fn get(&self) -> &$ty; fn put(&mut self, value: $ty); }

        impl $name for $store {
            fn get(&self) -> &$ty {
                match self.store.get(stringify!($name).into()) {
                    Some(dep) => { match dep.downcast_ref::<$ty>() {
                        Some(dep) => dep,
                        None => panic!("Could not downcast {} to {} - wrong binding! type?",
                            stringify!($name), stringify!($ty))
                    }
                    },
                    None => panic!("{} has no binding for {}!\n\tBound types: {:?}\n",
                         stringify!($store), stringify!($name), self.store.keys())
                }
            }
            fn put(&mut self, value: $ty) {
                match self.store.entry(stringify!($name).into()) {
                    ::std::collections::hash_map::Entry::Occupied(entry) => {
                        let existing: &$ty = entry.get().downcast_ref::<$ty>().unwrap();
                        panic!(
                            "Conflicting binding for {}; cannot bind to {:?} already bound to {:?}",
                            stringify!($bnd), value, existing)
                    },
                    ::std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(Box::new(value) as Box<::std::any::Any>);
                    }
                }
                //self.store.insert(stringify!($name).into(), Box::new(value) as Box<Any>);
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
    binder!(MyDeps);
    binding!(MyDeps, MyBinding, String);
    provider!(MyDeps, ProvidedBinding, str, MyBinding, |dep: &'a MyBinding| &dep.get()[..3]);

    #[test]
    fn basic_di() {
        let mut deps = MyDeps::new();
        bind!(deps, MyBinding, "FooBar".to_string());

        let my_binding: &MyBinding = &deps;
        let my_provided_binding: &ProvidedBinding = &deps;
        assert_eq!(my_binding.get(), "FooBar");
        assert_eq!(my_provided_binding.get(), "Foo");
    }

    #[test]
    #[should_panic(expected = "MyDeps has no binding for MyBinding!")]
    fn basic_di_missing_binding() {
        let deps = MyDeps::new();
        let my_binding: &MyBinding = &deps;
        my_binding.get();
    }

    // TODO more tests
}