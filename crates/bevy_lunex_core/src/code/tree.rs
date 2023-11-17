use std::borrow::Borrow;
use bevy::prelude::*;
use pathio::{prelude::*, PathTreeSingle, DirectorySingle};
use crate::{RelativeLayout, Container, LayoutPackage};
use crate::LunexError;
use super::types::DataWrap;

const LEVEL_DEPTH_DIFFERENCE: f32 = 10.0;

// ===========================================================
// === PATHIO IMPLEMENTATION ===

/// # UiT - UiTree logic
/// This trait implements every method that you expect from [`UiTree`].
/// It abstacts and builds on top of hierarchy logic added by `Pathio` crate.
/// 
/// It is recommended to use methods primarly from this trait, instead of `Pathio` native methods.
/// 
/// If you *NEED* more low level control over the struct, you can opt-in by adding Pathio to your cargo file and importing
/// the [`PathioHierarchy`] trait, which should allow you deeper control over the hierarchy. Note that name conflicts are bound to happen.
/// Use the turbofish syntax.
pub trait UiT<T:Default> {
    /// Creates a new UiTree
    fn new (name: impl Borrow<str>) -> Self;

    /// Compute the layout starting at origin
    fn compute(&mut self, point: Vec2, width: f32, height: f32);

    /// Creates a branch with given layout
    fn create_branch(&mut self, path: impl Borrow<str>, layout: impl Into<LayoutPackage>) -> Result<String, LunexError>;

    /// Borrows a branch on given path
    fn borrow_branch(&self, path: impl Borrow<str>) -> Result<&UiBranch<T>, LunexError>;

    /// Borrows a branch on given path
    fn borrow_branch_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiBranch<T>, LunexError>;

    /// Drops a branch on given path
    fn drop_branch(&mut self, path: impl Borrow<str>) -> Result<(), LunexError>;

    /// Merges UiTree or UiBranch content into itself
    fn merge(&mut self, directory: impl Into<UiBranch<T>>) -> Result<(), LunexError>;

    /// Generate a tree-like printable structure of the dir
    fn tree(&self) -> String;

    /// Return branch depth
    fn get_depth(&self) -> f32;

    /// Return branch visibility. Does not mean the branch is going to be visible due to inherited visibility
    fn get_visibility(&self) -> bool;

    /// Set branch visibility
    fn set_visibility(&mut self, visibility: bool);

    /// Return if branch is visible or not. Counts in inherited visibility
    fn is_visible(&self) -> bool;
}
impl <T:Default> UiT<T> for UiTree<T> {
    fn new (name: impl Borrow<str>) -> Self {
        let mut tree: UiTree<T> = pathio::PathTreeInit::new(name);
        let mut container = Container::new();
        container.set_layout(RelativeLayout::new());
        container.set_render_depth(100.0);
        tree.add_file(DataWrap::new(container));
        tree
    }

    fn compute(&mut self, point: Vec2, width: f32, height: f32) {
        self.directory.compute(point, width, height);
    }

    fn create_branch(&mut self, path: impl Borrow<str>, layout: impl Into<LayoutPackage>) -> Result<String, LunexError> {
        self.directory.create_branch(path, layout)
    }

    fn borrow_branch(&self, path: impl Borrow<str>) -> Result<&UiBranch<T>, LunexError> {
        Ok(self.borrow_directory(path)?)
    }

    fn borrow_branch_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiBranch<T>, LunexError> {
        Ok(self.borrow_directory_mut(path)?)
    }

    fn drop_branch(&mut self, path: impl Borrow<str>) -> Result<(), LunexError> {
        match self.remove_directory(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn merge(&mut self, directory: impl Into<UiBranch<T>>) -> Result<(), LunexError> {
        let mut dir:UiBranch<T> = directory.into();
        let _ = dir.take_file();
        Ok(pathio::PathioHierarchy::merge(self, dir)?)
    }

    fn tree(&self) -> String {
        pathio::PathioHierarchy::tree_dir(self)
    }

    fn get_depth(&self) -> f32 {
        0.0 //pathio::PathioHierarchy::get_depth(self)
    }

    fn get_visibility(&self) -> bool {
        self.obtain_file().unwrap().container().get_visibility()
    }

    fn set_visibility(&mut self, visibility: bool) {
        self.obtain_file_mut().unwrap().container_mut().set_visibility(visibility);
        self.cascade_update_inherited_visibility();
    }

    fn is_visible(&self) -> bool {
        self.obtain_file().unwrap().container().is_visible()
    }
}

/// # UiD - UiBranch logic (Directory)
/// This trait implements every method that you expect from [`UiBranch`].
/// It abstacts and builds on top of hierarchy logic added by `Pathio` crate.
/// 
/// It is recommended to use methods primarly from this trait, instead of `Pathio` native methods.
/// 
/// If you *NEED* more low level control over the struct, you can opt-in by adding Pathio to your cargo file and importing
/// the [`PathioHierarchy`] trait, which should allow you deeper control over the hierarchy. Note that name conflicts are bound to happen.
/// Use the turbofish syntax.
pub trait UiD<T:Default> {
    /// Compute the layout starting at origin
    fn compute(&mut self, point: Vec2, width: f32, height: f32);
    
    /// Create a branch with given layout
    fn create_branch(&mut self, path: impl Borrow<str>, layout: impl Into<LayoutPackage>) -> Result<String, LunexError>;

    /// Borrows a branch on given path
    fn borrow_branch(&self, path: impl Borrow<str>) -> Result<&UiBranch<T>, LunexError>;

    /// Borrows a branch on given path
    fn borrow_branch_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiBranch<T>, LunexError>;

    /// Drops a branch on given path
    fn drop_branch(&mut self, path: impl Borrow<str>) -> Result<(), LunexError>;

    /// Merges UiTree or UiBranch content into itself
    fn merge(&mut self, directory: impl Into<UiBranch<T>>) -> Result<(), LunexError>;

    /// Generate a tree-like printable structure of the dir
    fn tree(&self) -> String;

    /// Borrow a container from this branch
    fn get_container(&self) -> &Container;

    /// Borrow a container from this branch
    fn get_container_mut(&mut self) -> &mut Container;

    /// Borrow data from this branch
    fn get_data(&mut self) -> &T;

    /// Borrow data from this branch
    fn get_data_mut(&mut self) -> &mut T;

    /// Return branch depth
    fn get_depth(&self) -> f32;

    /// Return branch visibility. Does not mean the branch is going to be visible due to inherited visibility
    fn get_visibility(&self) -> bool;

    /// Set branch visibility
    fn set_visibility(&mut self, visibility: bool);

    /// Return if branch is visible or not. Counts in inherited visibility
    fn is_visible(&self) -> bool;
}
impl <T:Default> UiD<T> for UiBranch<T> {
    fn compute(&mut self, point: Vec2, width: f32, height: f32) {
        let container = self.get_container_mut();

        container.calculate(point, width, height);
        let pos = container.get_position().clone();
        for x in &mut self.directory {
            x.1.compute(pos.point_1, pos.width, pos.height);
        }
    }
    
    fn create_branch(&mut self, path: impl Borrow<str>, layout: impl Into<LayoutPackage>) -> Result<String, LunexError> {
        let name = self.create_directory(path.borrow())?;
        let parent = self.get_container();
        let mut container = Container::new();
        container.set_layout(layout);
        container.set_inherited_visibility(parent.is_visible());
        container.set_render_depth(parent.get_render_depth());
        self.insert_file(name.clone(), DataWrap::new(container))?;      //EXPECTS THAT PATH is NAME only, no "/"
        Ok(name)
    }

    fn borrow_branch(&self, path: impl Borrow<str>) -> Result<&UiBranch<T>, LunexError> {
        Ok(self.borrow_directory(path)?)
    }

    fn borrow_branch_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiBranch<T>, LunexError> {
        Ok(self.borrow_directory_mut(path)?)
    }

    fn drop_branch(&mut self, path: impl Borrow<str>) -> Result<(), LunexError> {
        match self.remove_directory(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn merge(&mut self, directory: impl Into<UiBranch<T>>) -> Result<(), LunexError> {
        let mut dir:UiBranch<T> = directory.into();
        let _ = dir.take_file();
        Ok(pathio::PathioHierarchy::merge(self, dir)?)
    }

    fn tree(&self) -> String {
        pathio::PathioHierarchy::tree_dir(self)
    }

    fn get_container(&self) -> &Container {
        self.obtain_file().unwrap().container()
    }

    fn get_container_mut(&mut self) -> &mut Container {
        self.obtain_file_mut().unwrap().container_mut()
    }

    fn get_data(&mut self) -> &T {
        self.obtain_file_mut().unwrap().data()
    }

    fn get_data_mut(&mut self) -> &mut T {
        self.obtain_file_mut().unwrap().data_mut()
    }

    fn get_depth(&self) -> f32 {
        pathio::PathioHierarchy::get_depth(self) * LEVEL_DEPTH_DIFFERENCE
    }

    fn get_visibility(&self) -> bool {
        self.get_container().get_visibility()
    }

    fn set_visibility(&mut self, visibility: bool) {
        self.get_container_mut().set_visibility(visibility);
        self.cascade_update_inherited_visibility();
    }

    fn is_visible(&self) -> bool {
        self.get_container().is_visible()
    }
}

/// # Custom Directory Recursion
/// trait used for recursive logic not exposed in the API, only for inner purposes.
trait CustomDirectoryRecursion {
    fn cascade_update_inherited_visibility(&mut self);
}
impl <T:Default> CustomDirectoryRecursion for UiBranch<T> {
    fn cascade_update_inherited_visibility(&mut self) {
        let visibility = self.is_visible();
        for (_, subdir) in &mut self.directory {
            subdir.get_container_mut().set_inherited_visibility(visibility);
            subdir.cascade_update_inherited_visibility();
        }
    }
}
impl <T:Default> CustomDirectoryRecursion for UiTree<T> {
    fn cascade_update_inherited_visibility(&mut self) {
        let visibility = self.is_visible();
        self.directory.get_container_mut().set_inherited_visibility(visibility);
        self.directory.cascade_update_inherited_visibility();
    }
}

/// # UiTree
/// Special HashMap-like struct for storing widget information.
/// All data is structured into a tree hierarchy.
pub type UiTree<T> = PathTreeSingle<DataWrap<T>>;

/// # UiBranch
/// A type located inside of [`UiTree`]
pub type UiBranch<T> = DirectorySingle<DataWrap<T>>;
