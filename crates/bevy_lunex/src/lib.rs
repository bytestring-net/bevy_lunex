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
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        app.add_plugins(UiDebugPlugin::<MainUi>::new());

        app
            .add_plugins(UiGenericPlugin::<MainUi>::new())
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(LunexBackend)
            .add_plugins(LogicPlugin);
    }
}


// #======================#
// #=== PRELUDE EXPORT ===#

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
    pub use super::UiPlugin;
    pub use super::systems::{UiSystems, UiGenericPlugin, UiDebugPlugin};
    pub use super::structs::*;

    // RE-EXPORT BEVY MOD PICKING
    pub use bevy_mod_picking::prelude::*;
    
    // RE-EXPORT LUNEX ENGINE
    pub use lunex_engine::prelude::*;
}