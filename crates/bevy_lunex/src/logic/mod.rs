pub mod core;
pub use core::*;

//pub mod hover;
//pub use hover::*;


// #====================#
// #=== LOGIC PLUGIN ===#

use bevy::prelude::*;

/// Plugin adding all our route logic
pub struct LogicPlugin;
impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CorePlugin);
            //.add_plugins(HoverPlugin);
    }
}