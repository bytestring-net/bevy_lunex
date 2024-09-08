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
use crate::*;

/// Plugin adding all our route logic
pub struct UiLogicPlugin;
impl Plugin for UiLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ActionsPlugin)
            .add_plugins(CorePlugin)
            .add_plugins(CursorPlugin)
            .add_plugins(DefaultStatesPlugin)
            .add_plugins(StylePlugin);
    }
}

/// Plugin adding all our route logic
#[derive(Debug, Default, Clone)]
pub struct UiStateLogicPlugin <T:Component = MainUi, N:Default + Component = NoData>(PhantomData<T>, PhantomData<N>);
impl <T:Component, N:Default + Component> UiStateLogicPlugin<T, N> {
    pub fn new() -> Self {
        UiStateLogicPlugin::<T, N>(PhantomData, PhantomData)
    }
}
impl <T:Component, N:Default + Component> Plugin for UiStateLogicPlugin<T, N> {
    fn build(&self, app: &mut App) {
        app            
            //.add_plugins(StatePlugin::<T, N, Base>::new())
            .add_plugins(StatePlugin::<T, N, Hover>::new())
            .add_plugins(StatePlugin::<T, N, Clicked>::new())
            .add_plugins(StatePlugin::<T, N, Selected>::new())
            .add_plugins(StatePlugin::<T, N, Intro>::new())
            .add_plugins(StatePlugin::<T, N, Outro>::new());
    }
}