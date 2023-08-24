use bevy::utils::thiserror::Error;
use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum LunexError {
    /// Error that happens when merging branches. 2 child branches have the same name.
    #[error("duplicate name '{0:}'")]
    DuplicateName(String),

    /// Error that happens when attempted to create a branch with a name that is used by another branch already.
    #[error("name '{0:}' already in use")]
    NameInUse(String),

    /// Error that happens when you try to locate a branch by name that doesn't exist.
    #[error("no shortcut '{0:}'")]
    NoShortcut(String),

    /// Error that happens when you try to locate a branch by ID that doesn't exist.
    #[error("branch with ID #{0:} doesn't exist")]
    NoBranch(usize),

    /// Syntax error that happens when provided ID string is invalid.
    #[error("invalid branch ID: {0:}")]
    InvalidId(ParseIntError),

    /// Syntax error that happens when the path provided is invalid (miss-use of '/').
    #[error("the path syntax is invalid")]
    InvalidPathSyntax,

    /// Error that happens when [`crate::Widget`] fails in locating itself in [`crate::UiTree`].
    #[error("could not find '{path:}': {cause:}")]
    FetchError {
        path: String,
        cause: Box<LunexError>,
    },

    /// Error that happens when provided 2D vec is not suitable to be turned to grid.
    #[error("Grid column {c1:} (len: {len_c1:}) has a different length to column 0 (len: {len_c0:})")]
    GridFormat {
        c1: usize,
        len_c1: usize,
        len_c0: usize,
    },
}