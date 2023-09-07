use ahash::AHashMap as HashMap;
use bevy::prelude::*;
use colored::Colorize;

use crate::{RelativeLayout, Container, LayoutPackage};
use crate::{is_numerical_id, extract_id, LunexError};

const ROOT_STARTING_DEPTH: f32 = 100.0;
const LEVEL_DEPTH_DIFFERENCE: f32 = 10.0;
const HIGHLIGHT_DEPTH_ADDED: f32 = 5.0;

// ===========================================================
// === UITREE STRUCT ===

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
        branch.container.layout_set(
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


    /// # Merge
    /// This method will merge another branch into this branch. As long as there are no name collision, the merge will be succesfull.
    ///
    /// ## Important!
    /// It is worth noting that internal IDs of the merged widgets WILL change. That means if there are unnamed widgets in the root of
    /// the merged branch, their paths will become invalid if the preserved branch is not empty.
    ///
    /// To work around this, all widgets located in the root of the merging MUST be named and accessed through their names!
    /// ```
    /// let mut existing_tree = UiTree::new(); //Let's say it contains other widgets...
    ///
    /// let mut merged_tree = UiTree::new();    //This is blank new tree, so it's empty...
    /// let background = Widget::create(&mut merged_tree, "background", Layout::Solid::default().pack())?;  //It's first so ID is '0'
    /// let image = Widget::create(&mut merged_tree, &background.end(""), Layout::Solid::default().pack())?;  //unnamed widgets not in the root are fine
    ///
    /// existing_tree.merge(merged_tree)?;     //The `background` after merge is no longer ID '0', but is offset by widgets that already existed there.
    /// ```
    /// ## Bad practice! Avoid!
    /// ```
    /// let mut existing_tree = UiTree::new(); //Let's say it contains other widgets...
    ///
    /// let mut merged_tree = UiTree::new();    //This is blank new tree, so it's empty...
    /// let background = Widget::create(&mut merged_tree, "", Layout::Solid::default().pack())?;  //No name but ID is '0'
    ///
    /// existing_tree.merge(merged_tree)?;     //ID changed so we have no way of accessing the widget!!!
    /// ```
    ///
    pub fn merge(&mut self, tree: UiTree) -> Result<(), LunexError> {
        self.branch.merge(tree.branch)
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


// ===========================================================
// === BRANCH STRUCT ===

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
    // ===========================================================
    // === EXPOSED BRANCH BORROW ===

    /// Returns borrow of [`Data`] struct mounted on this branch
    pub fn data_get(&self) -> &Option<Data> {
        &self.data
    }
    
    /// Returns mut borrow of [`Data`] struct mounted on this branch
    pub fn data_get_mut(&mut self) -> &mut Option<Data> {
        &mut self.data
    }

    /// Returns borrow of layout data of this branch
    pub fn layout_get(&self) -> &LayoutPackage {
        self.container.layout_get()
    }
    
    /// Returns mut borrow of layout data of this branch
    pub fn layout_get_mut(&mut self) -> &mut LayoutPackage {
        self.container.layout_get_mut()
    }

    /// Returns borrow of [`Container`] struct mounted on this branch
    pub fn container_get(&self) -> &Container {
        &self.container
    }
    
    /// Returns mut borrow of [`Container`] struct mounted on this branch
    pub fn container_get_mut(&mut self) -> &mut Container {
        &mut self.container
    }

    /// Returns borrow of [`UiBranch`], a sub-branch nested within this branch on a given path
    pub fn branch_get(&self, path: &str) -> Result<&UiBranch, LunexError> {
        self.borrow_linked_checked(path)
    }

    /// Returns mut borrow of [`UiBranch`], a sub-branch nested within this branch on a given path
    pub fn branch_get_mut(&mut self, path: &str) -> Result<&mut UiBranch, LunexError> {
        self.borrow_linked_checked_mut(path)
    }


    // ===========================================================
    // === EXPOSED BRANCH CONTROL ===

    /// # Merge
    /// This method will merge another branch into this branch. As long as there are no name collision, the merge will be succesfull.
    ///
    /// ## Important!
    /// It is worth noting that internal IDs of the merged widgets WILL change. That means if there are unnamed widgets in the root of
    /// the merged branch, their paths will become invalid if the preserved branch is not empty.
    ///
    /// To work around this, all widgets located in the root of the merging MUST be named and accessed through their names!
    /// ```
    /// let mut existing_tree = UiTree::new(); //Let's say it contains other widgets...
    ///
    /// let mut merged_tree = UiTree::new();    //This is blank new tree, so it's empty...
    /// let background = Widget::create(&mut merged_tree, "background", Layout::Solid::default().pack())?;  //It's first so ID is '0'
    /// let image = Widget::create(&mut merged_tree, &background.end(""), Layout::Solid::default().pack())?;  //unnamed widgets not in the root are fine
    ///
    /// existing_tree.merge(merged_tree)?;     //The `background` after merge is no longer ID '0', but is offset by widgets that already existed there.
    /// ```
    /// ## Bad practice! Avoid!
    /// ```
    /// let mut existing_tree = UiTree::new(); //Let's say it contains other widgets...
    ///
    /// let mut merged_tree = UiTree::new();    //This is blank new tree, so it's empty...
    /// let background = Widget::create(&mut merged_tree, "", Layout::Solid::default().pack())?;  //No name but ID is '0'
    ///
    /// existing_tree.merge(merged_tree)?;     //ID changed so we have no way of accessing the widget!!!
    /// ```
    ///
    pub fn merge(&mut self, mut branch: UiBranch) -> Result<(), LunexError> {
        // Check if there is a name collision
        for (name, _) in branch.shortcuts.iter() {
            if self.shortcuts.contains_key(name) {
                return Result::Err(LunexError::DuplicateName(name.to_string()));
            }
        }

        //Merge it
        for (name, path) in branch.shortcuts.iter() {
            match path.split_once('/') {
                Some ((numeric_path, rest_of_path)) => {

                    //Extract child branch from merging branch
                    let old_id = extract_id(numeric_path).unwrap();
                    let mut e_branch = branch.inventory.remove(&old_id).unwrap();

                    //Get new ID
                    let mut new_id = 0;
                    loop {
                        if !self.inventory.contains_key(&new_id) {
                            break;
                        } else {
                            new_id += 1
                        }
                    }

                    //Construct new path
                    let new_path = format!("#{}/{}", new_id, rest_of_path);
                    e_branch.id = new_id;
                    //e_branch.path = new_path;

                    //Merge it
                    self.inventory.insert(new_id, e_branch);
                    self.shortcuts.insert(name.to_string(), new_path);
                    
                },
                None => {
                    //Extract child branch from merging branch
                    let old_id = extract_id(path).unwrap();
                    let mut e_branch = branch.inventory.remove(&old_id).unwrap();

                    //Get new ID
                    let mut new_id = 0;
                    loop {
                        if !self.inventory.contains_key(&new_id) {
                            break;
                        } else {
                            new_id += 1
                        }
                    }

                    //Construct new path
                    let new_path = format!("#{}", new_id);
                    e_branch.id = new_id;
                    //e_branch.path = new_path;

                    //Merge it
                    self.inventory.insert(new_id, e_branch);
                    self.shortcuts.insert(name.to_string(), new_path);
                }
            }
        }

        Result::Ok(())
    }

    /// Returns the name from local cache on the same branch, is not guaranteed to be valid
    pub fn get_name(&self) -> &String {
        &self.name
    }

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


    // ===========================================================
    // === EXPOSED BRANCH DEBUG ===

    /// Generate overview of the inner tree in a stringified form
    pub fn generate_map(&self) -> String {
        let text = String::new();
        format!(
            "> {}{}",
            self.name.purple().bold().underline(),
            self.cascade_generate_map(text, 0)
        )
    }

    /// Generate debug view of the inner tree in a stringified form
    pub fn generate_map_debug(&self) -> String {
        let text = String::new();
        format!(
            "> {}{}",
            self.name.purple().bold().underline(),
            self.cascade_generate_map_debug(text, 0)
        )
    }
    
    /// Return a vector to iterate over containing all paths to all sub-branches
    pub fn collect_paths(&self) -> Vec<String> {
        let mut list = Vec::new();
        self.cascade_collect_paths(&mut list, "");
        list
    }


    // ===========================================================
    // === RECURSIVE SUB-FUNCTIONS ===

    /// Generate overview of the inner tree and write the mapped output to the given string with data formatted to a certain level depth
    pub(super) fn cascade_generate_map(&self, mut string: String, level: u32) -> String {
        for (name, path) in & self.shortcuts {
            match self.borrow_linked_checked(&path) {
                Ok(branch) => {
                    let mut text = String::from("\n  ");
                    for _ in 0..level { text += "|    " }
                    text += "|-> ";
                    string = format!("{}{}{}", string, text.black(), name.bold().yellow());
                    string = branch.cascade_generate_map(string, level + 1);
                }
                Err(_) => (),
            }
        }
        string
    }

    /// Generate debug view of the inner tree and write the mapped debug output to the given string with data formatted to a certain level depth
    pub(super) fn cascade_generate_map_debug(&self, mut string: String, level: u32) -> String {
        let mut done_branches: HashMap<String, bool> = HashMap::new();
        string = format!(
            "{}{}",
            string,
            format!(
                " - [{}-#{}] [{}/{}] | ({}/{})",
                self.name,
                self.id,
                self.level,
                self.get_depth(),
                self.visible,
                self.parent_visible
            )
            .black()
            .italic()
        );

        for (name, path) in self.shortcuts.iter() {
            match self.borrow_linked_checked(&path) {
                Ok(branch) => {
                    let mut text = String::from("\n  ");
                    for _ in 0..level {
                        text += "|    "
                    }
                    text += "|-> ";
                    string = format!(
                        "{}{}{} ({})",
                        string,
                        text.black(),
                        name.bold().yellow(),
                        path
                    );
                    string = branch.cascade_generate_map_debug(string, level + 1);
                    done_branches.insert(path.to_string(), true);
                }
                Err(_) => {
                    let mut text = String::from("\n  ");
                    for _ in 0..level {
                        text += "|    "
                    }
                    text += "|-> ";
                    string = format!(
                        "{}{}{}",
                        string,
                        text.black(),
                        format!("{} #[! Dangling pointer !]", name)
                            .bold()
                            .red()
                    );
                }
            }
        }
        for x in self.inventory.iter() {
            if done_branches.contains_key(&("#".to_string() + &x.0.to_string())) {
                continue;
            }

            let mut text = String::from("\n  ");
            for _ in 0..level {
                text += "|    "
            }
            text += "|-> ";
            string = format!(
                "{}{}{}",
                string,
                text.black(),
                format!("#{}", x.0).bold().truecolor(255, 165, 214)
            );

            string = x.1.cascade_generate_map_debug(string, level + 1);
        }
        string
    }

    /// Crawl through the subbranches and return all valid paths in the given directory
    pub(super) fn cascade_collect_paths(&self, list: &mut Vec<String>, directory: &str) {
        let mut done_branches: HashMap<String, bool> = HashMap::new();

        for (name, path) in self.shortcuts.iter() {
            match self.borrow_linked_checked(&path) {
                Ok(branch) => {
                    let dir = if directory.is_empty() {
                        String::from(name)
                    } else {
                        format!("{}/{}", directory, name)
                    };
                    list.push(dir.clone());
                    branch.cascade_collect_paths(list, &dir);

                    done_branches.insert(path.to_string(), true);
                }
                Err(_) => {}
            }
        }
        for x in self.inventory.iter() {
            if done_branches.contains_key(&("#".to_string() + &x.0.to_string())) {
                continue;
            }

            let dir = if directory.is_empty() {
                String::from(format!("#{}", x.0))
            } else {
                format!("{}/{}", directory, format!("#{}", x.0))
            };
            list.push(dir.clone());
            x.1.cascade_collect_paths(list, &dir);
        }
    }

    /// Computes the branches layout and recursively computes the sub-branches
    pub(super) fn cascade_compute_layout(&mut self, origin: Vec2, width: f32, height: f32) {
        self.container.calculate(origin, width, height);
        for x in &mut self.inventory {
            let pos = self.container.position_get();
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

    /// Create new branch and set name, id, path, level and visibility to cache
    pub(super) fn create_simple(&mut self, name: &str, position: LayoutPackage) -> String {
        let mut id = 0;
        loop {
            if !self.inventory.contains_key(&id) {
                break;
            } else {
                id += 1
            }
        }
        let path = if name.is_empty() { format!("{}/#{}", self.get_path(), id) } else { format!("{}/{}", self.get_path(), name) };

        let mut branch = UiBranch::new(
            name.to_string(),
            id,
            path,
            self.level + 1.0,
            self.is_visible(),
        );

        branch.container.layout_set(position);

        self.inventory.insert(id, branch);
        format!("#{}", id)
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

    /// Create new shortcut with name and path
    pub(super) fn shortcut_add(&mut self, name: String, path: String) -> Result<(), LunexError> {
        if self.shortcuts.contains_key(&name) {
            return Err(LunexError::NameInUse(name));
        }
        self.shortcuts.insert(name, path);
        Ok(())
    }

    /// Checks for shortcut ***NAME*** and returns path
    pub(super) fn translate_simple(&self, name: &str) -> Result<String, LunexError> {
        match self.shortcuts.get(name) {
            Some(path) => Ok(path.into()),
            None => Err(LunexError::NoShortcut(name.into())),
        }
    }


    // ===========================================================
    // === BRANCH BORROW ===

    /// Parses the ***NUMERICAL UID*** and returns borrow
    pub(super) fn borrow_simple(&self, uid: &str) -> Result<&UiBranch, LunexError> {
        match str::parse::<usize>(&uid[1..]) {
            Ok(id) => match self.inventory.get(&id) {
                Some(branch) => Ok(branch),
                None => Err(LunexError::NoBranch(id)),
            },
            Err(e) => Err(LunexError::InvalidId(e)),
        }
    }

    /// Checks for shortcut ***NAME*** and calls `borrow_simple` or enters recursion
    pub(super) fn borrow_simple_checked(&self, name: &str) -> Result<&UiBranch, LunexError> {
        if !name.is_empty() {
            if is_numerical_id(name) {
                self.borrow_simple(name)
            } else {
                match self.translate_simple(name) {
                    Ok(path) => self.borrow_linked_checked(&path),
                    Err(e) => Err(e),
                }
            }
        } else {
            Err(LunexError::InvalidPathSyntax)
        }
    }

    /// Checks the ***PATH*** until recursively locates the branch and calls `borrow_simple_checked`
    pub(super) fn borrow_linked_checked(&self, path: &str) -> Result<&UiBranch, LunexError> {
        match path.split_once('/') {
            None => self.borrow_simple_checked(path),
            Some((branch, remaining_path)) => match self.borrow_simple_checked(branch) {
                Ok(borrowed_branch) => borrowed_branch.borrow_linked_checked(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    /// Parses the ***NUMERICAL UID*** and returns mut borrow
    pub(super) fn borrow_simple_mut(&mut self, uid: &str) -> Result<&mut UiBranch, LunexError> {
        match str::parse::<usize>(&uid[1..]) {
            Ok(id) => match self.inventory.get_mut(&id) {
                Some(branch) => Ok(branch),
                None => Err(LunexError::NoBranch(id)),
            },
            Err(e) => Err(LunexError::InvalidId(e)),
        }
    }

    /// Checks for shortcut ***NAME*** and calls `borrow_simple_mut` or enters recursion
    pub(super) fn borrow_simple_checked_mut(&mut self, name: &str) -> Result<&mut UiBranch, LunexError> {
        if !name.is_empty() {
            if is_numerical_id(name) {
                self.borrow_simple_mut(name)
            } else {
                match self.translate_simple(name) {
                    Ok(path) => self.borrow_linked_checked_mut(&path),
                    Err(e) => Err(e),
                }
            }
        } else {
            Err(LunexError::InvalidPathSyntax)
        }
    }

    /// Checks the ***PATH*** until recursively locates the branch and calls `borrow_simple_checked_mut`
    pub(super) fn borrow_linked_checked_mut(&mut self, path: &str) -> Result<&mut UiBranch, LunexError> {
        match path.split_once('/') {
            None => self.borrow_simple_checked_mut(path),
            Some((branch, remaining_path)) => match self.borrow_simple_checked_mut(branch) {
                Ok(borrowed_branch) => borrowed_branch.borrow_linked_checked_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }


    // ===========================================================
    // === BRANCH REMOVAL ===

    /// Parses the ***NUMERICAL UID*** and drops the branch
    pub(super) fn drop_simple(&mut self, uid: &str) -> Result<(), LunexError> {
        match str::parse::<usize>(&uid[1..]) {
            Ok (id) => match self.inventory.remove(&id) {
                Some(_) => Ok(()),
                None => Err(LunexError::NoBranch(id)),
            },
            Err(e) => Err(LunexError::InvalidId(e)),
        }
    }

    /// Checks for shortcut ***NAME*** and calls `drop_simple` or enters recursion
    pub(super) fn drop_simple_checked(&mut self, name: &str) -> Result<(), LunexError> {
        if !name.is_empty() {
            if is_numerical_id(name) {
                self.drop_simple(name)
            } else {
                match self.translate_simple(name) {
                    Ok(path) => self.drop_linked_checked(&path),
                    Err(e) => Err(e),
                }
            }
        } else {
            Err(LunexError::InvalidPathSyntax)
        }
    }

    /// Checks the ***PATH*** until recursively locates the branch and calls `drop_simple_checked`
    pub(super) fn drop_linked_checked(&mut self, path: &str) -> Result<(), LunexError> {
        match path.split_once('/') {
            None => self.drop_simple_checked(path),
            Some((branch, remaining_path)) => match self.borrow_simple_checked_mut(branch) {
                Ok(borrowed_branch) => borrowed_branch.drop_linked_checked(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    /// Checks for shortcut ***NAME*** and removes it. Then drops the branch
    pub(super) fn remove_simple_checked(&mut self, name: &str) -> Result<(), LunexError> {
        if self.shortcuts.contains_key(name) {
            match self.drop_linked_checked(name) {
                Ok(_) => {
                    self.shortcuts.remove(name);
                    Ok(())
                },
                Err (e) => Err(e),
            }
        } else {
            Err(LunexError::NoShortcut(name.into()))
        }
    }

    /// Checks all shortcuts and removes them if they are invalid, returns number of removed shortcuts
    pub(super) fn remove_invalid(&mut self) -> usize {
        let mut marked: Vec<String> = Vec::new();
        for (shortcut, path) in &self.shortcuts {
            match self.borrow_linked_checked(path) {
                Ok (..) => {},
                Err (..) => {
                    marked.push(shortcut.to_string())
                },
            }
        }
        let n = marked.len();
        for shortcut in marked {
            self.shortcuts.remove(&shortcut);
        }
        n
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
