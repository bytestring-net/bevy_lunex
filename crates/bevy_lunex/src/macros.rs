// OUTDATED AND WILL NEED TO BE FIXED LATER!!!

/* 
/// ## Ui Plugin
/// Bundles all your Ui systems into one plugin for clarity and ease of use.
/// 
/// Implements `new()` for easy creation.
/// 
/// Note that you should follow proper naming pattern. If you bundle logic for a main menu, name the plugin
/// `MenuPlugin` or `MainMenuPlugin`, etc.
/// 
/// Creates `MyPlugin` with the following systems.
/// ```rust
/// ui_plugin!(MyPlugin, add_systems(Update, system1), add_systems(Update, system2));
/// ```
/// 
/// In case you use generic system for `Ui<T>`, you can use that generic here too.
/// ```rust
/// ui_plugin!(MyPlugin, add_systems(Update, generic_system::<T>));
/// ```
#[macro_export]
macro_rules! ui_plugin {
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
} */