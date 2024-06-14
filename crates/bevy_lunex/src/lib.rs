#![doc = include_str!("../README.md")]

// #==============================#
// #=== IMPORTS FOR THIS CRATE ===#

pub (crate) use std::{borrow::Borrow, marker::PhantomData};
pub (crate) use bevy::prelude::*;
pub (crate) use lunex_engine::prelude::*;
pub (crate) use bevy_mod_picking::prelude::*;

#[cfg(feature = "debug")]
pub (crate) use colored::Colorize;



// #======================#
// #=== GENERAL PLUGIN ===#

/// Plugin implementing general logic.
pub struct UiGeneralPlugin;
impl Plugin for UiGeneralPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiPlugin::<MainUi>::new())
            .add_plugins(DefaultPickingPlugins)

            .add_plugins(crate::LunexBackend)
            .add_plugins(LogicPlugin)
            .add_plugins(CursorPlugin);
    }
}


// #======================#
// #=== PRELUDE EXPORT ===#

pub mod interaction;
pub use interaction::*;

pub mod logic;
pub use logic::*;

pub mod picking;
pub use picking::*;

pub mod structs;
pub use structs::*;

pub mod systems;
pub use systems::*;


pub mod prelude {

    pub use super::Cursor2d;
    pub use super::actions;

    pub use super::logic::*;

    // BEVY-LUNEX SPECIFIC
    pub use super::UiGeneralPlugin;
    pub use super::systems::{UiSystems, UiPlugin, UiDebugPlugin};
    pub use super::structs::*;

    
    // RE-EXPORT LUNEX ENGINE
    pub use lunex_engine::prelude::*;
}