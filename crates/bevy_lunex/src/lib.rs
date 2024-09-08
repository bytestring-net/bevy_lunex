#![doc = include_str!("../README.md")]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

// #==============================#
// #=== IMPORTS FOR THIS CRATE ===#

pub (crate) use std::{borrow::Borrow, marker::PhantomData};
use bevy::app::PluginGroupBuilder;
pub (crate) use bevy::prelude::*;
pub (crate) use lunex_engine::prelude::*;
pub (crate) use bevy_mod_picking::prelude::*;

#[cfg(feature = "verbose")]
pub (crate) use colored::Colorize;



// #======================#
// #=== GENERAL PLUGIN ===#

/// Plugin group implementing generic logic for given marker.
#[derive(Debug, Default, Clone)]
pub struct UiGenericPlugins <T:Component = MainUi, N:Default + Component = NoData>(PhantomData<T>, PhantomData<N>);
impl <T:Component, N:Default + Component> UiGenericPlugins<T, N> {
    pub fn new() -> Self {
        UiGenericPlugins::<T, N>(PhantomData, PhantomData)
    }
}
impl <T:Component, N:Default + Component> PluginGroup for UiGenericPlugins<T, N> {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        // Add core logic
        builder = builder.add(UiCorePlugin::<T, N>::new());

        // Add functionality logic
        builder = builder.add(UiStateLogicPlugin::<T, N>::new());

        // Add debug logic
        #[cfg(feature = "debug")]
        {builder = builder.add(UiDebugPlugin::<T, N>::new());}

        // Return the plugin group
        builder
    }
}


/// Plugin group implementing minimal default logic.
pub struct UiMinimalPlugins;
impl PluginGroup for UiMinimalPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        // Add core logic
        builder = builder.add(UiCorePlugin::<MainUi>::new());

        // Add debug logic
        #[cfg(feature = "debug")]
        {builder = builder.add(UiDebugPlugin::<MainUi>::new());}

        // Return the plugin group
        builder
    }
}


/// Plugin group implementing all UI logic + required plugins for other functionality to work, including picking.
pub struct UiDefaultPlugins;
impl PluginGroup for UiDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        // Add default MainUi plugins
        builder = builder.add_group(UiGenericPlugins::<MainUi>::new());

        // Add non generic state logic
        builder = builder.add(UiLogicPlugin);

        // Add picking
        builder = builder.add(UiLunexPickingPlugin);
        builder = builder.add_group(DefaultPickingPlugins.build().disable::<InputPlugin>());

        // Return the plugin group
        builder
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
    pub use super::UiGenericPlugins;
    pub use super::UiMinimalPlugins;
    pub use super::UiDefaultPlugins;
    pub use super::systems::{UiSystems, UiDebugPlugin};
    pub use super::structs::*;

    pub use super::PickingPortal;

    // RE-EXPORT BEVY MOD PICKING
    pub use bevy_mod_picking::prelude::*;
    
    // RE-EXPORT LUNEX ENGINE
    pub use lunex_engine::prelude::*;
}