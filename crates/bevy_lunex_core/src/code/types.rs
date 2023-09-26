use bevy::utils::thiserror::Error;
use pathio::PathioError;

#[derive(Debug, Error)]
pub enum LunexError {
    /// Error that happens when merging branches. Two branches have the same name.
    #[error("Container from merging branch was not dropped before merging")]
    ContainerConflict,

    /// Error that happens when merging branches. Two branches have the same name.
    #[error("Duplicate name conflict for '{0:}' when trying to merge directory")]
    DuplicateName (String),

    /// Error that happens when merging branches. Two branches have the same name.
    #[error("Name '{0:}' is already in use")]
    NameInUse (String),

    /// Error that happens when path provided is not allowed.
    #[error("Path '{0:}' is not allowed")]
    InvalidPath (String),

    /// Error that happens when you try to locate a branch that doesn't exist.
    #[error("Unable to locate '{0:}' branch")]
    NoBranch (String),




    /// Error that happens when [`crate::Widget`] fails to locate itself in [`crate::UiTree`].
    #[error("could not find '{path:}': {cause:}")]
    FetchError {
        path: String,
        cause: Box<LunexError>,
    },
}
impl From<PathioError> for LunexError {
    fn from(value: PathioError) -> Self {
        match value {
            PathioError::FileConflict => LunexError::ContainerConflict,
            PathioError::DuplicateName (v) => LunexError::DuplicateName (v),
            PathioError::NameInUse (v) => LunexError::NameInUse (v),
            PathioError::NoDirectory (v) => LunexError::NoBranch (v),
            PathioError::NoFile (_) => panic!("API should NOT ALLOW you to get this error"),
            PathioError::InvalidPath (v) => LunexError::InvalidPath (v),
        }
    }
}