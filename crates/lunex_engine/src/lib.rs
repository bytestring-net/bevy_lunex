pub mod common;
pub use common::*;

pub mod core;
pub use core::*;

pub mod layout;
pub use layout::*;

pub mod nodes;
pub use nodes::*;


// #======================#
// #=== PRELUDE EXPORT ===#

pub mod prelude {
    pub use super::common::prelude::*;
    pub use super::core::prelude::*;
    pub use super::layout::prelude::*;
}

// #=========================#
// #=== CRATE ONLY EXPORT ===#

pub mod import {
    pub(crate) use std::borrow::Borrow;

    pub(crate) use indexmap::IndexMap as HashMap;
    pub(crate) use colored::Colorize;

    //pub(crate) use glam::{Vec2, Vec3, Vec4};          //Used to de-couple from bevy if needed
    //pub(crate) use thiserror::Error;
    pub(crate) use bevy::math::{Vec2, Vec3, Vec4};
    pub(crate) use bevy::utils::thiserror::Error;
}