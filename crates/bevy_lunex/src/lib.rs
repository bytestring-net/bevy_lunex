#![doc = include_str!("../README.md")]

// #==============================#
// #=== IMPORTS FOR THIS CRATE ===#

pub (crate) use std::{borrow::Borrow, marker::PhantomData};
pub (crate) use bevy::prelude::*;
pub (crate) use lunex_engine::prelude::*;

#[cfg(feature = "debug")]
pub (crate) use colored::Colorize;

#[cfg(feature = "picking")]
pub (crate) use bevy_mod_picking::prelude::*;


// #======================#
// #=== GENERAL PLUGIN ===#

/// Plugin implementing general logic.
pub struct UiGeneralPlugin;
impl Plugin for UiGeneralPlugin {
    fn build(&self, app: &mut App) {

        #[cfg(feature = "picking")]
        app.add_plugins(crate::LunexBackend);

        app
            .add_plugins(crate::CursorPlugin)
            .add_plugins(crate::UiEventPlugin);
    }
}


// #======================#
// #=== PRELUDE EXPORT ===#

pub mod events;
pub use events::*;

pub mod interaction;
pub use interaction::*;

pub mod logic;
pub use logic::*;

//pub mod macros;

#[cfg(feature = "picking")]
pub mod picking;
#[cfg(feature = "picking")]
pub use picking::*;

pub mod structs;
pub use structs::*;

pub mod systems;
pub use systems::*;


pub mod prelude {

    pub use super::Cursor2d;
    pub use super::events::{SetColor, SetUiLayout};

    // BEVY-LUNEX SPECIFIC
    pub use super::UiGeneralPlugin;
    pub use super::systems::*;
    pub use super::structs::*;

    
    // RE-EXPORT LUNEX ENGINE
    pub use lunex_engine::prelude::*;
}