#![doc = include_str!("../../../README.md")]

// #======================#
// #=== PRELUDE EXPORT ===#

pub mod interaction;
pub use interaction::*;

pub mod macros;

pub mod structs;
pub use structs::*;

pub mod systems;
pub use systems::*;


pub mod prelude {

    pub use super::Cursor2d;

    // BEVY-LUNEX SPECIFIC
    pub use super::systems::*;
    pub use super::structs::*;

    
    // RE-EXPORT LUNEX ENGINE
    pub use lunex_engine::prelude::*;
}