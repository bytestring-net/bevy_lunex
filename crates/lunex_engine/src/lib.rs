#![doc = include_str!("../README.md")]

// #======================#
// #=== PRELUDE EXPORT ===#

pub mod core;
pub use core::*;

pub mod layout;
pub use layout::*;

pub mod nodes;
pub use nodes::*;


pub mod prelude {
    pub use super::core::prelude::*;
    pub use super::layout::prelude::*;
}

// #=========================#
// #=== CRATE ONLY IMPORT ===#

pub(crate) mod import {
    pub(crate) use std::borrow::Borrow;

    pub(crate) use indexmap::IndexMap;
    pub(crate) use bevy::utils::HashMap;
    pub(crate) use colored::Colorize;

    pub(crate) use bevy::math::{Vec2, Vec3, Vec4};
    pub(crate) use thiserror::Error;
}