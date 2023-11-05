use std::borrow::Borrow;

use bevy::prelude::*;
use pathio::{prelude::*, PathTreeSingle, DirectorySingle};

use crate::{RelativeLayout, Container, LayoutPackage};
use crate::LunexError;


const LEVEL_DEPTH_DIFFERENCE: f32 = 10.0;

// ===========================================================
// === PATHIO IMPLEMENTATION ===

/// # UiT
/// This trait is required whenewer you iteract with [`UiTree`].
/// It abstacts and unifies complex hierarchy logic added by pathio crate into one trait.
/// 
/// It implements every function you expect [`UiTree`] to have.
pub trait UiT {
    /// Creates a new UiTree
    fn new (name: impl Borrow<str>) -> Self;

    /// Compute the layout starting at origin
    fn compute(&mut self, point: Vec2, width: f32, height: f32);

    /// Creates a branch with given layout
    fn create_branch(&mut self, path: impl Borrow<str>, layout: impl Into<LayoutPackage>) -> Result<(), LunexError>;

    /// Borrows a branch on given path
    fn borrow_branch(&self, path: impl Borrow<str>) -> Result<&UiBranch, LunexError>;

    /// Borrows a branch on given path
    fn borrow_branch_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiBranch, LunexError>;

    /// Drops a branch on given path
    fn drop_branch(&mut self, path: impl Borrow<str>) -> Result<(), LunexError>;

    /// Merges UiTree or UiBranch content into itself
    fn merge(&mut self, directory: impl Into<DirectorySingle<Container>>) -> Result<(), LunexError>;

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
impl UiT for PathTreeSingle<Container> {
    fn new (name: impl Borrow<str>) -> Self {
        let mut tree: PathTreeSingle<Container> = <PathTreeSingle<Container> as pathio::PathTreeInit>::new(name);
        let mut container = Container::new();
        container.set_layout(RelativeLayout::new());
        container.set_render_depth(100.0);
        tree.add_file(container);
        tree
    }

    fn compute(&mut self, point: Vec2, width: f32, height: f32) {
        self.directory.compute(point, width, height);
    }

    fn create_branch(&mut self, path: impl Borrow<str>, layout: impl Into<LayoutPackage>) -> Result<(), LunexError> {
        self.directory.create_branch(path, layout)
    }

    fn borrow_branch(&self, path: impl Borrow<str>) -> Result<&UiBranch, LunexError> {
        Ok(self.borrow_directory(path)?)
    }

    fn borrow_branch_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiBranch, LunexError> {
        Ok(self.borrow_directory_mut(path)?)
    }

    fn drop_branch(&mut self, path: impl Borrow<str>) -> Result<(), LunexError> {
        match self.remove_directory(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn merge(&mut self, directory: impl Into<DirectorySingle<Container>>) -> Result<(), LunexError> {
        let mut dir:DirectorySingle<Container> = directory.into();
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
        self.obtain_file().unwrap().get_visibility()
    }

    fn set_visibility(&mut self, visibility: bool) {
        self.obtain_file_mut().unwrap().set_visibility(visibility);
        self.cascade_update_inherited_visibility();
    }

    fn is_visible(&self) -> bool {
        self.obtain_file().unwrap().is_visible()
    }
}

/// # UiD
/// This trait is required whenewer you iteract with [`UiBranch`].
/// It abstacts and unifies complex hierarchy logic added by pathio crate into one trait.
/// 
/// It implements every function you expect [`UiBranch`] to have.
pub trait UiD {
    /// Compute the layout starting at origin
    fn compute(&mut self, point: Vec2, width: f32, height: f32);
    
    /// Create a branch with given layout
    fn create_branch(&mut self, path: impl Borrow<str>, layout: impl Into<LayoutPackage>) -> Result<(), LunexError>;

    /// Borrows a branch on given path
    fn borrow_branch(&self, path: impl Borrow<str>) -> Result<&UiBranch, LunexError>;

    /// Borrows a branch on given path
    fn borrow_branch_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiBranch, LunexError>;

    /// Drops a branch on given path
    fn drop_branch(&mut self, path: impl Borrow<str>) -> Result<(), LunexError>;

    /// Merges UiTree or UiBranch content into itself
    fn merge(&mut self, directory: impl Into<DirectorySingle<Container>>) -> Result<(), LunexError>;

    /// Generate a tree-like printable structure of the dir
    fn tree(&self) -> String;

    /// Borrow a container from this branch
    fn get_container(&self) -> &Container;

    /// Borrow a container from this branch
    fn get_container_mut(&mut self) -> &mut Container;

    /// Return branch depth
    fn get_depth(&self) -> f32;

    /// Return branch visibility. Does not mean the branch is going to be visible due to inherited visibility
    fn get_visibility(&self) -> bool;

    /// Set branch visibility
    fn set_visibility(&mut self, visibility: bool);

    /// Return if branch is visible or not. Counts in inherited visibility
    fn is_visible(&self) -> bool;
}
impl UiD for DirectorySingle<Container> {
    fn compute(&mut self, point: Vec2, width: f32, height: f32) {
        let container = self.obtain_file_mut().unwrap();

        container.calculate(point, width, height);
        let pos = container.get_position().clone();
        for x in &mut self.directory {
            x.1.compute(pos.point_1, pos.width, pos.height);
        }
    }
    
    fn create_branch(&mut self, path: impl Borrow<str>, layout: impl Into<LayoutPackage>) -> Result<(), LunexError> {
        self.create_directory(path.borrow())?;
        let mut container = Container::new();
        container.set_layout(layout);
        container.set_inherited_visibility(self.file.as_ref().unwrap().is_visible());
        container.set_render_depth(self.file.as_ref().unwrap().get_render_depth());
        self.insert_file(path, container)?;
        Ok(())
    }

    fn borrow_branch(&self, path: impl Borrow<str>) -> Result<&UiBranch, LunexError> {
        Ok(self.borrow_directory(path)?)
    }

    fn borrow_branch_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiBranch, LunexError> {
        Ok(self.borrow_directory_mut(path)?)
    }

    fn drop_branch(&mut self, path: impl Borrow<str>) -> Result<(), LunexError> {
        match self.remove_directory(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn merge(&mut self, directory: impl Into<DirectorySingle<Container>>) -> Result<(), LunexError> {
        let mut dir:DirectorySingle<Container> = directory.into();
        let _ = dir.take_file();
        Ok(pathio::PathioHierarchy::merge(self, dir)?)
    }

    fn tree(&self) -> String {
        pathio::PathioHierarchy::tree_dir(self)
    }

    fn get_container(&self) -> &Container {
        self.obtain_file().unwrap()
    }

    fn get_container_mut(&mut self) -> &mut Container {
        self.obtain_file_mut().unwrap()
    }

    fn get_depth(&self) -> f32 {
        pathio::PathioHierarchy::get_depth(self) * LEVEL_DEPTH_DIFFERENCE
    }

    fn get_visibility(&self) -> bool {
        self.obtain_file().unwrap().get_visibility()
    }

    fn set_visibility(&mut self, visibility: bool) {
        self.obtain_file_mut().unwrap().set_visibility(visibility);
        self.cascade_update_inherited_visibility();
    }

    fn is_visible(&self) -> bool {
        self.obtain_file().unwrap().is_visible()
    }
}

trait CustomDirectoryRecursion {
    fn cascade_update_inherited_visibility(&mut self);
}
impl CustomDirectoryRecursion for DirectorySingle<Container> {
    fn cascade_update_inherited_visibility(&mut self) {
        let visibility = self.is_visible();
        for (_, subdir) in &mut self.directory {
            subdir.obtain_file_mut().unwrap().set_inherited_visibility(visibility);
            subdir.cascade_update_inherited_visibility();
        }
    }
}
impl CustomDirectoryRecursion for PathTreeSingle<Container> {
    fn cascade_update_inherited_visibility(&mut self) {
        let visibility = self.is_visible();
        self.directory.obtain_file_mut().unwrap().set_inherited_visibility(visibility);
        self.directory.cascade_update_inherited_visibility();
    }
}

pub type UiTree = PathTreeSingle<Container>;
pub type UiBranch = DirectorySingle<Container>;




// ===========================================================
// === BRANCH STRUCT ===
/* 
/// A struct that can nest another branches inside itself, implemented as tree like structure
/// 
/// Holds all data, layout and handles the logic of Lunex.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct UiBranch {
    //# CACHING =======
    /// Only the name from path
    name: String,
    /// The ID it is indexed under
    id: usize,
    /// Path without the branch name
    path: String,

    //# RENDERING =======
    /// How deep the branch is in UiTree
    level: f32,
    /// Z index calculated from branch depth
    depth: f32,
    /// If branch is activated, can be used to check for interactivity
    active: bool,
    /// If branch has visibility enabled
    visible: bool,
    /// If branch is currently highligted
    in_focus: bool,
    /// If the parenting container is visible
    parent_visible: bool,

    //# MOUNTED DATA =======
    container: Container,
    data: Option<Data>,

    //# RECURSION =======
    inventory: HashMap<usize, UiBranch>,
    shortcuts: HashMap<String, String>,
}
impl UiBranch {


    /// Returns current depth with focus counted in
    pub fn get_depth(&self) -> f32 {
        if self.in_focus {
            self.level * LEVEL_DEPTH_DIFFERENCE + self.depth + HIGHLIGHT_DEPTH_ADDED
        } else {
            self.level * LEVEL_DEPTH_DIFFERENCE + self.depth
        }
    }

    /// Set depth of the branch and all its sub-branches
    pub fn set_depth(&mut self, depth: f32) {
        self.cascade_set_depth(depth);
    }

    /// Constructs the path from local cache on the same branch, is not guaranteed to be valid
    pub fn get_path(&self) -> String {
        if self.level == 0.0 {
            "".to_string()
        } else if !self.path.is_empty() {
            format!("{}/{}", self.path, self.name)
        } else {
            String::from(&self.name)
        }
    }

    /// Returns if branch is in focus (highlighted, helps to decide which branch in the same layer to prefer)
    pub fn get_focus(&self) -> bool {
        self.in_focus
    }

    /// Set focus of the current branch (helpful when you have overlaying branches in the same layer)
    pub fn set_focus(&mut self, focus: bool) {
        self.in_focus = focus;
    }

    /// This will check if branch is overall visible, including local and inherited visibility from parent branches
    pub fn is_visible(&self) -> bool {
        self.visible == true && self.parent_visible == true
    }

    /// This will return local visibility of the branch
    pub fn get_visibility(&self) -> bool {
        self.visible
    }

    /// This will set local visibility to the value given, but it doesn't mean the branch will be 100% visible
    pub fn set_visibility(&mut self, visible: bool) {
        let old = self.is_visible();
        self.visible = visible;
        let new = self.is_visible();
        if new != old {
            self.cascade_compute_visibility()
        }
    }



    /// Return a vector to iterate over containing all paths to all sub-branches
    pub fn collect_paths(&self) -> Vec<String> {
        let mut list = Vec::new();
        self.cascade_collect_paths(&mut list, "");
        list
    }


    /// Computes the branches layout and recursively computes the sub-branches
    pub(super) fn cascade_compute_layout(&mut self, origin: Vec2, width: f32, height: f32) {
        self.container.calculate(origin, width, height);
        for x in &mut self.inventory {
            let pos = self.container.get_position();
            x.1.cascade_compute_layout(pos.point_1, pos.width, pos.height);
        }
    }

    /// Checks branches visibility and recursively overwrites sub-branches parent visibility
    pub(super) fn cascade_compute_visibility(&mut self) {
        let visibility = self.is_visible();
        for x in &mut self.inventory {
            x.1.parent_visible = visibility;
            x.1.cascade_compute_visibility()
        }
    }

    /// Recursively overwrites the subbranches depth
    pub(super) fn cascade_set_depth(&mut self, depth: f32) {
        self.depth = depth;
        for x in &mut self.inventory {
            x.1.cascade_set_depth(depth);
        }
    }


    // ===========================================================
    // === BRANCH CREATION ===

    /// Create this struct from given arguments
    fn new(name: String, id: usize, path: String, level: f32, parent_visible: bool) -> UiBranch {
        UiBranch {
            name,
            id,
            path,

            level,
            depth: ROOT_STARTING_DEPTH,
            active: true,
            visible: true,
            in_focus: false,
            parent_visible,

            container: Container::new(),
            data: None,

            inventory: HashMap::new(),
            shortcuts: HashMap::new(),
        }
    }

    /// Register new shortcut if any and calls `create_simple` to make new branch
    pub(super) fn create_linked(&mut self, name: &str, position: LayoutPackage) -> Result<String, LunexError> {
        if name.is_empty() {
            Ok(self.create_simple("", position))
        } else {
            if !self.shortcuts.contains_key(name) {
                let path = self.create_simple(name, position);
                self.shortcuts.insert(name.to_string(), path);
                Ok(name.into())
            } else {
                Err(LunexError::NameInUse(name.into()))
            }
        }
    }


}

// ===========================================================
// === DATA MOUNTED ON BRANCH ===

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Data {
    pub f32s: HashMap<String, f32>,
    pub vec2s: HashMap<String, Vec2>,
    pub vec3s: HashMap<String, Vec3>,
    pub vec4s: HashMap<String, Vec4>,
    pub bools: HashMap<String, bool>,
    pub strings: HashMap<String, String>,
}
impl Data {
    pub fn new() -> Data {
        Data {
            f32s: HashMap::new(),
            vec2s: HashMap::new(),
            vec3s: HashMap::new(),
            vec4s: HashMap::new(),
            bools: HashMap::new(),
            strings: HashMap::new(),
        }
    }
}
*/