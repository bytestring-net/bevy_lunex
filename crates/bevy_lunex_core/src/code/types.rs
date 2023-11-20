use bevy::{utils::thiserror::Error, ecs::component::Component};
use pathio::PathioError;
use crate::Container;

#[derive(Clone, Component, Debug, Error, PartialEq)]
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


/// # DataWrap
/// Wrapping struct that automates working with the appended data on the branch.
/// Holds custom widget data (T) and [`Container`] data.
#[derive(Clone, Component, Debug, Default, PartialEq)]
pub struct DataWrap<T:Default> {
    container: Container,
    data: Option<T>,
}
impl <T: Default> DataWrap<T> {
    pub fn new(container: Container) -> Self {
        DataWrap {
            container,
            data: None,
        }
    }
    /// Borrow a container wrapped in the struct.
    pub fn container(&self) -> &Container {
        &self.container
    }
    /// Mutably borrow a container wrapped in the struct.
    pub fn container_mut(&mut self) -> &mut Container {
        &mut self.container
    }
    /// Borrow a data(T) wrapped in the struct, still requires mut access
    /// because the data struct is created using default method if it doesn't exist.
    pub fn data(&mut self) -> &T {
        if let None = &self.data {
            self.data = Some(T::default());
        }
        match &self.data {
            Some(t) => t,
            None => unreachable!()
        }
    }
    /// Mutably borrow a data(T) wrapped in the struct, struct is created using default method if it doesn't exist. 
    pub fn data_mut(&mut self) -> &mut T {
        if let None = &self.data {
            self.data = Some(T::default());
        }
        match &mut self.data {
            Some(t) => t,
            None => unreachable!()
        }
    }
}

#[derive(Clone, Component, Debug, Default, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// # Modifier
/// A special enum dictating if *(T)* is supposed to be
/// **added** to a specific property or **override** it instead
#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub enum Modifier<T> {
    Add (T),
    Set (T),
}