pub mod actions;
use actions::ActionsPlugin;

pub mod core;
pub use core::*;

//pub mod hover;
//pub use hover::*;


// #====================#
// #=== LOGIC PLUGIN ===#

use bevy::prelude::*;

/// Plugin adding all our route logic
pub (crate) struct LogicPlugin;
impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ActionsPlugin)
            .add_plugins(CorePlugin);
    }
}