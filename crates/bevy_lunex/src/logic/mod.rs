pub mod actions;
use actions::ActionsPlugin;

pub mod core;
pub use core::*;

pub mod cursor;
pub use cursor::*;

pub mod states;
pub use states::*;

pub mod style;
pub use style::*;


// #====================#
// #=== LOGIC PLUGIN ===#

use bevy::prelude::*;

/// Plugin adding all our route logic
pub struct LogicPlugin;
impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ActionsPlugin)
            .add_plugins(CorePlugin)
            .add_plugins(CursorPlugin)
            .add_plugins(DefaultStatesPlugin)
            .add_plugins(StylePlugin);
    }
}