/// # Import Use
/// Macro for bulk `pub mod` and `pub use`
/// 
/// ```rust
/// import_use!(code, lib)
/// ```
/// will translate to
/// ```rust
/// pub mod code;
/// pub use code::*;
/// pub mod lib;
/// pub use lib::*;
/// ```
#[macro_export]
macro_rules! import_use {
    ($( $imp:ident ),*) => {
        $(
            pub mod $imp;
            pub use $imp::*;
        )*
    };
}


/// # Script Plugin
/// Generates a plugin with proper setup and implementations for packing all local systems into one package.
/// 
/// Implements `new()` for easy creation.
/// 
/// Note that you should follow proper naming pattern. If you bundle logic for a main menu, name the plugin
/// `MenuPlugin` or `MainMenuPlugin`, etc.
/// 
/// Creates `MyPlugin` with the following systems.
/// ```rust
/// script_plugin!(MyPlugin, add_systems(Update, system1), add_systems(Update, system2));
/// ```
/// 
/// In case you use generic system for `UiTree<T>`, you can use that generic here too.
/// ```rust
/// script_plugin!(MyPlugin, add_systems(Update, generic_system::<T>));
/// ```
#[macro_export]
macro_rules! script_plugin {
    ($struct_name:ident, $( $method:ident ( $($args:expr),* ) ),*) => {
        #[derive(Debug, Clone, Default)]
        pub(super) struct $struct_name<T: bevy::prelude::Component + Default>(pub std::marker::PhantomData<T>);
        impl<T: bevy::prelude::Component + Default> $struct_name<T> {
            #[allow(dead_code)]
            pub fn new() -> Self {
                $struct_name::<T>(std::marker::PhantomData)
            }
        }
        impl<T: bevy::prelude::Component + Default> bevy::prelude::Plugin for $struct_name<T> {
            fn build(&self, app: &mut bevy::prelude::App) {
                $(
                    app.$method($($args),*);
                )*
            }
        }
    };
    ($struct_name:ident) => {
        #[derive(Debug, Clone, Default)]
        pub(super) struct $struct_name<T: bevy::prelude::Component + Default>(pub std::marker::PhantomData<T>);
        impl<T: bevy::prelude::Component + Default> $struct_name<T> {
            #[allow(dead_code)]
            pub fn new() -> Self {
                $struct_name::<T>(std::marker::PhantomData)
            }
        }
        impl<T: bevy::prelude::Component + Default> bevy::prelude::Plugin for $struct_name<T> {
            fn build(&self, _app: &mut bevy::prelude::App) {
            }
        }
    };
}