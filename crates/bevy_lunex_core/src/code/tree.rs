use std::borrow::Borrow;

use bevy::prelude::*;
use pathio::{prelude::*, PathTreeSingle, DirectorySingle};

use crate::{RelativeLayout, Container, LayoutPackage};
use crate::LunexError;

const ROOT_STARTING_DEPTH: f32 = 100.0;
const LEVEL_DEPTH_DIFFERENCE: f32 = 10.0;
const HIGHLIGHT_DEPTH_ADDED: f32 = 5.0;

// ===========================================================
// === UITREE STRUCT ===


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
}

impl UiT for PathTreeSingle<Container> {
    fn new (name: impl Borrow<str>) -> Self {
        let mut tree: PathTreeSingle<Container> = <PathTreeSingle<Container> as pathio::PathTreeInit>::new(name);
        let mut container = Container::new();
        container.set_layout(RelativeLayout::new());
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
}

pub trait UiD {
    /// Compute the layout starting at origin
    fn compute(&mut self, point: Vec2, width: f32, height: f32);
    
    /// Create a branch with given layout
    fn create_branch(&mut self, path: impl Borrow<str>, layout: impl Into<LayoutPackage>) -> Result<(), LunexError>;

    /// Borrows a branch on given path
    fn borrow_branch(&self, path: impl Borrow<str>) -> Result<&UiBranch, LunexError>;

    /// Borrows a branch on given path
    fn borrow_branch_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiBranch, LunexError>;

    /// Borrow a container from this branch
    fn get_container(&self) -> &Container;

    /// Borrow a container from this branch
    fn get_container_mut(&mut self) -> &mut Container;
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
        self.insert_file(path, container)?;
        Ok(())
    }

    fn borrow_branch(&self, path: impl Borrow<str>) -> Result<&UiBranch, LunexError> {
        Ok(self.borrow_directory(path)?)
    }

    fn borrow_branch_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiBranch, LunexError> {
        Ok(self.borrow_directory_mut(path)?)
    }

    fn get_container(&self) -> &Container {
        self.obtain_file().unwrap()
    }

    fn get_container_mut(&mut self) -> &mut Container {
        self.obtain_file_mut().unwrap()
    }
}

pub type UiTree = PathTreeSingle<Container>;
pub type UiBranch = DirectorySingle<Container>;



/*
/// # UiTree
/// A tree-like data structure holding all UI layout data and information, similar to hierarchy.
///
/// You can retrieve data from this structure using paths.
/// * `settings`
/// * `settings/display`
/// * `settings/display/button_1`
///
#[derive(Component, Default, Clone, Debug, PartialEq)]
pub struct UiTree {
    pub width: f32,
    pub height: f32,
    pub offset: Vec2,
    branch: UiBranch,
}
impl UiTree {
    // ===========================================================
    // === EXPOSED BRANCH BORROW ===

    /// Returns borrow of the main [`UiBranch`] that this [`UiTree`] is wrapped around
    pub(super) fn main_branch(&self) -> &UiBranch {
        &self.branch
    }

    /// Returns mut borrow of the main [`UiBranch`] that this [`UiTree`] is wrapped around
    pub(super) fn main_branch_mut(&mut self) -> &mut UiBranch {
        &mut self.branch
    }

    /// Returns borrow of [`UiBranch`], a branch nested within this tree on a given path
    pub fn branch_get(&self, path: &str) -> Result<&UiBranch, LunexError> {
        self.branch.borrow_linked_checked(path)
    }

    /// Returns mut borrow of [`UiBranch`], a branch nested within this tree on a given path
    pub fn branch_get_mut(&mut self, path: &str) -> Result<&mut UiBranch, LunexError> {
        self.branch.borrow_linked_checked_mut(path)
    }

    // ===========================================================
    // === EXPOSED TREE CONTROL ===

    /// Creates a new tree with the given name
    pub fn new(name: &str) -> UiTree {
        let mut branch = UiBranch::new(name.into(), 0, "".into(), 0.0, true);
        branch.container.set_layout(
            RelativeLayout::default(),
        );

        UiTree {
            width: 0.0,
            height: 0.0,
            offset: Vec2::new(0.0, 0.0),
            branch,
        }
    }

    /// Compute the layout starting at origin
    pub fn compute_at_origin(&mut self) {
        self.branch.cascade_compute_layout(Vec2::default(), self.width, self.height);
    }

    /// Compute the layout starting at offset instead
    pub fn compute_with_offset(&mut self) {
        self.branch.cascade_compute_layout(self.offset, self.width, self.height);
    }



    /// Returns the name of the tree
    pub fn get_name(&self) -> &String {
        &self.branch.name
    }

    /// Returns current depth
    pub fn get_depth(&self) -> f32 {
        self.branch.get_depth()
    }

    /// Set depth of the tree and all its branches
    pub fn set_depth(&mut self, depth: f32) {
        self.branch.set_depth(depth);
    }

    /// This will return visibility of the tree
    pub fn get_visibility(&self) -> bool {
        self.branch.get_visibility()
    }

    /// This will set visibility to the value given
    pub fn set_visibility(&mut self, visible: bool) {
        self.branch.set_visibility(visible)
    }


    // ===========================================================
    // === EXPOSED TREE DEBUG ===

    /// Generate overview of the inner tree in a stringified form
    pub fn generate_map(&self) -> String {
        self.branch.generate_map()
    }

    /// Generate debug view of the inner tree in a stringified form
    pub fn generate_map_debug(&self) -> String {
        self.branch.generate_map_debug()
    }
    
    /// Return a vector to iterate over containing all paths to all sub-branches
    pub fn collect_paths(&self) -> Vec<String> {
        self.branch.collect_paths()
    }

}
*/

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
/*
pub struct Spawner {
    list1: Vec<Box<dyn Bundle>>,
    //or
    list2: Vec<Box<impl Bundle>>,
}*/