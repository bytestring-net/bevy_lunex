use std::num::ParseIntError;

use super::export::*;
use super::ui_container::{Container, LayoutPackage};
use bevy::prelude::*;
use bevy::utils::thiserror::Error;
use colored::Colorize;

const ROOT_STARTING_DEPTH: f32 = 100.0;
const LEVEL_DEPTH_DIFFERENCE: f32 = 10.0;
const HIGHLIGHT_DEPTH_ADDED: f32 = 5.0;

// ===========================================================
// === UITREE STRUCT ===

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
    branch: Branch,
}

impl UiTree {
    pub fn new() -> UiTree {
        //let mut branch = Branch::new(0.0, true, "ROOT", "".to_string());
        let mut branch = Branch::new("ROOT".to_string(), 0, "".to_string(), 0.0, true);
        branch.container.layout_set(
            layout::Relative {
                relative_1: Vec2 { x: 0.0, y: 0.0 },
                relative_2: Vec2 { x: 100.0, y: 100.0 },
                ..Default::default()
            }
            .pack(),
        );

        UiTree {
            width: 0.0,
            height: 0.0,
            branch,
        }
    }

    pub fn update(&mut self) {
        self.branch
            .cascade_update_self(Vec2::default(), self.width, self.height);
    }

    pub fn get_map(&self) -> String {
        let text = String::new();
        format!(
            "{}{}",
            "#ROOT".purple().bold().underline(),
            self.branch.cascade_map(text, 0)
        )
    }

    pub fn get_map_debug(&self) -> String {
        let text = String::new();
        format!(
            "{}{}",
            "#ROOT".purple().bold().underline(),
            self.branch.cascade_map_debug(text, 0)
        )
    }

    pub fn collect_paths(&self) -> Vec<String> {
        self.branch.collect_paths()
    }

    pub fn merge(&mut self, tree: UiTree) -> Result<(), BranchError> {
        self.branch.merge(tree.branch)
    }

    pub(super) fn root_get(&self) -> &Branch {
        &self.branch
    }

    pub(super) fn root_get_mut(&mut self) -> &mut Branch {
        &mut self.branch
    }
}

pub fn hierarchy_update(mut query: Query<&mut UiTree>, mut windows: Query<&mut Window>) {
    let window = windows.get_single_mut().unwrap();
    for mut system in &mut query {
        system.width = window.resolution.width();
        system.height = window.resolution.height();

        system.update();
    }
}

// ===========================================================
// === BRANCH STRUCT ===

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Branch {
    //# CACHING =======
    name: String,
    id: usize,
    path: String,

    //# RENDERING =======
    /// How deep the branch is in UITree
    level: f32,
    /// Z index calculated from branch depth
    depth: f32,
    /// If widget is activated, can be used to check for interactivity
    active: bool,
    /// If widget has visibility enabled
    visible: bool,
    /// If widget is currently highligted
    in_focus: bool,
    /// If the parenting container is visible
    parent_visible: bool,

    //# MOUNTED DATA =======
    container: Container,
    data: Option<Data>,

    //# RECURSION =======
    inventory: HashMap<usize, Branch>,
    shortcuts: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum BranchError {
    #[error("duplicate name '{0:}'")]
    DuplicateName(String),
    #[error("name '{0:}' already in use")]
    NameInUse(String),
    #[error("branch already contains name '{0:}'")]
    AlreadyContainsPath(String),
    #[error("no shortcut '{0:}'")]
    NoShortcut(String),
    #[error("branch with ID #{0:} doesn't exist")]
    NoBranch(usize),
    #[error("invalid branch ID: {0:}")]
    InvalidId(ParseIntError),
    #[error("cannot borrow branch with no name")]
    BorrowNoName,
}

impl Branch {
    //#USER EXPOSED CONTROL

    //Borrows
    pub fn data_get(&self) -> &Option<Data> {
        &self.data
    }
    pub fn data_get_mut(&mut self) -> &mut Option<Data> {
        &mut self.data
    }

    pub fn layout_get(&self) -> &LayoutPackage {
        self.container.layout_get()
    }
    pub fn layout_get_mut(&mut self) -> &mut LayoutPackage {
        self.container.layout_get_mut()
    }

    pub fn container_get(&self) -> &Container {
        &self.container
    }
    pub fn container_get_mut(&mut self) -> &mut Container {
        &mut self.container
    }

    //Fn calls
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_depth(&self) -> f32 {
        if self.in_focus {
            self.level * LEVEL_DEPTH_DIFFERENCE + self.depth + HIGHLIGHT_DEPTH_ADDED
        } else {
            self.level * LEVEL_DEPTH_DIFFERENCE + self.depth
        }
    }

    pub fn set_depth(&mut self, depth: f32) {
        self.cascade_set_depth_self(depth);
    }

    pub fn set_depth_self_only(&mut self, depth: f32) {
        self.cascade_set_depth(depth);
    }

    pub fn get_path(&self) -> String {
        if self.level == 0.0 {
            "".to_string()
        } else if !self.path.is_empty() {
            format!("{}/{}", self.path, self.name)
        } else {
            String::from(&self.name)
        }
    }

    pub fn get_focus(&self) -> bool {
        self.in_focus
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.in_focus = focus;
    }

    pub fn is_visible(&self) -> bool {
        self.visible == true && self.parent_visible == true
    }

    pub fn get_visibility(&self) -> bool {
        self.visible
    }

    pub fn set_visibility(&mut self, visible: bool) {
        let old = self.is_visible();
        self.visible = visible;
        let new = self.is_visible();
        if new != old {
            self.cascade_set_visibility()
        }
    }

    pub fn get_map(&self) -> String {
        let text = String::new();
        format!(
            "{}{}",
            self.name.purple().bold().underline(),
            self.cascade_map(text, 0)
        )
    }

    pub fn get_map_debug(&self) -> String {
        let text = String::new();
        format!(
            "{}{}",
            self.name.purple().bold().underline(),
            self.cascade_map_debug(text, 0)
        )
    }

    pub fn collect_paths(&self) -> Vec<String> {
        let mut list = Vec::new();
        self.cascade_collect_paths(&mut list, "".to_string());
        list
    }

    /// # Merge
    /// This method will merge another branch into this branch. As long as there are no name collision, the merge will be succesfull.
    ///
    /// ## Important!
    /// It is worth noting that internal IDs of the merged branches WILL change. That means if there are unnamed branches in the root of
    /// the merged branch, their paths will become invalid if the preserved branch is not empty.
    ///
    /// To work around this, all branches located in the root of the merging MUST be named and accessed through their names!
    /// ```
    /// let mut existing_tree = UITree::new(); //Let's say it contains other widgets...
    ///
    /// let mut merged_tree = UITree::new();    //This is blank new tree, so it's empty...
    /// let background = Widget::create(&mut merged_tree, "background", Layout::Solid::default().pack())?;  //It's first so ID is '0'
    /// let image = Widget::create(&mut merged_tree, &background.end(""), Layout::Solid::default().pack())?;  //unnamed widgets not in the root are fine
    ///
    /// existing_tree.merge(merged_tree)?;     //The `background` after merge is no longer ID '0', but is offset by widgets that already existed there.
    /// ```
    /// ## Bad practice! Avoid!
    /// ```
    /// let mut existing_tree = UITree::new(); //Let's say it contains other widgets...
    ///
    /// let mut merged_tree = UITree::new();    //This is blank new tree, so it's empty...
    /// let background = Widget::create(&mut merged_tree, "", Layout::Solid::default().pack())?;  //No name but ID is '0'
    ///
    /// existing_tree.merge(merged_tree)?;     //ID changed so we have no way of accessing the widget!!!
    /// ```
    ///
    pub fn merge(&mut self, branch: Branch) -> Result<(), BranchError> {
        // Check if there is a name collision
        for name in branch.shortcuts.keys() {
            if self.shortcuts.contains_key(name) {
                return Err(BranchError::DuplicateName(name.into()));
            }
        }

        // commented out because we shouldn't be printing random debug stuff
        // // 1. Check if all paths to be merged are free to use
        // for id in branch.inventory.keys() {
        //     println!("Id: {}", id);
        // }

        // for (name, path) in branch.shortcuts.iter() {
        //     println!("name: {} = path: {}", name, path);
        // }

        Ok(())
        // 2. Merge them
    }

    //#LIBRARY RECURSION CALLS
    pub(super) fn cascade_map(&self, mut string: String, level: u32) -> String {
        for (name, path) in self.shortcuts.iter() {
            match self.borrow_linked_checked(&path) {
                Ok(widget) => {
                    let mut text = String::from("\n  ");
                    for _ in 0..level {
                        text += "|    "
                    }
                    text += "|-> ";
                    string = format!("{}{}{}", string, text.black(), name.bold().yellow());

                    string = widget.cascade_map(string, level + 1);
                }
                Err(..) => (),
            }
        }
        string
    }

    pub(super) fn cascade_map_debug(&self, mut string: String, level: u32) -> String {
        let mut done_widgets: HashMap<String, bool> = HashMap::new();
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
                Ok(widget) => {
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

                    string = widget.cascade_map_debug(string, level + 1);
                    done_widgets.insert(path.to_string(), true);
                }
                Err(..) => {
                    let mut text = String::from("\n  ");
                    for _ in 0..level {
                        text += "|    "
                    }
                    text += "|-> ";
                    string = format!(
                        "{}{}{}",
                        string,
                        text.black(),
                        format!("{} #[! Dangling register pointer !]", name)
                            .bold()
                            .red()
                    );
                }
            }
        }
        for x in self.inventory.iter() {
            if done_widgets.contains_key(&("#".to_string() + &x.0.to_string())) {
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

            string = x.1.cascade_map_debug(string, level + 1);
        }
        string
    }

    pub(super) fn cascade_collect_paths(&self, list: &mut Vec<String>, directory: String) {
        let mut done_widgets: HashMap<String, bool> = HashMap::new();

        for (name, path) in self.shortcuts.iter() {
            match self.borrow_linked_checked(&path) {
                Ok(widget) => {
                    let dir = if directory.is_empty() {
                        String::from(name)
                    } else {
                        format!("{}/{}", directory, name)
                    };
                    list.push(dir.clone());
                    widget.cascade_collect_paths(list, dir);

                    done_widgets.insert(path.to_string(), true);
                }
                Err(..) => {}
            }
        }
        for x in self.inventory.iter() {
            if done_widgets.contains_key(&("#".to_string() + &x.0.to_string())) {
                continue;
            }

            let dir = if directory.is_empty() {
                String::from(format!("#{}", x.0))
            } else {
                format!("{}/{}", directory, format!("#{}", x.0))
            };
            list.push(dir.clone());
            x.1.cascade_collect_paths(list, dir);
        }
    }

    pub(super) fn cascade_update_self(&mut self, point: Vec2, width: f32, height: f32) {
        //This will cascade update all branches
        self.container.calculate(point, width, height);
        for x in self.inventory.iter_mut() {
            let pos = self.container.position_get();
            x.1.cascade_update_self(pos.point_1, pos.width, pos.height);
        }
    }

    pub(super) fn cascade_set_visibility(&mut self) {
        //This will cascade set parent visible all branches
        let visibility = self.is_visible();
        for x in self.inventory.iter_mut() {
            x.1.cascade_set_visibility_self(visibility);
        }
    }

    pub(super) fn cascade_set_visibility_self(&mut self, visible: bool) {
        //This will cascade set parent visible all branches
        self.parent_visible = visible;
        self.cascade_set_visibility()
    }

    pub(super) fn cascade_set_depth(&mut self, depth: f32) {
        //This will cascade set parent visible all branches
        for x in self.inventory.iter_mut() {
            x.1.cascade_set_depth_self(depth);
        }
    }

    pub(super) fn cascade_set_depth_self(&mut self, depth: f32) {
        //This will cascade set parent visible all branches
        self.depth = depth;
        self.cascade_set_depth(depth);
    }

    //#LIBRARY MECHANISMS
    fn new(name: String, id: usize, path: String, level: f32, parent_visible: bool) -> Branch {
        Branch {
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
            data: Option::None,

            inventory: HashMap::new(),
            shortcuts: HashMap::new(),
        }
    }

    //
    pub(super) fn append(&mut self, branch: Branch) -> usize {
        let mut id = 0;
        loop {
            if !self.inventory.contains_key(&id) {
                break;
            } else {
                id += 1
            }
        }

        self.inventory.insert(id, branch);
        id
    }

    pub(super) fn create_simple(&mut self, name: &str, position: LayoutPackage) -> String {
        let mut id = 0;
        loop {
            if !self.inventory.contains_key(&id) {
                break;
            } else {
                id += 1
            }
        }

        let path = if name.is_empty() {
            format!("{}/#{}", self.get_path(), id)
        } else {
            format!("{}/{}", self.get_path(), name)
        };
        let mut branch = Branch::new(
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

    pub(super) fn create_linked(
        &mut self,
        name: &str,
        position: LayoutPackage,
    ) -> Result<String, BranchError> {
        if name.is_empty() {
            Ok(self.create_simple("", position))
        } else {
            if !self.shortcuts.contains_key(name) {
                let path = self.create_simple(name, position);
                self.shortcuts.insert(name.to_string(), path);
                Ok(name.into())
            } else {
                Err(BranchError::NameInUse(name.into()))
            }
        }
    }

    pub(super) fn register_path(&mut self, name: String, path: String) -> Result<(), BranchError> {
        //This registers ABSOLUTE PATH for a key
        if self.shortcuts.contains_key(&name) {
            return Err(BranchError::AlreadyContainsPath(name));
        }
        self.shortcuts.insert(name, path);
        Ok(())
    }

    pub(super) fn translate_simple(&self, name: &str) -> Result<String, BranchError> {
        //This can take ONLY RELATIVE and return ABSOLUTE
        match self.shortcuts.get(name) {
            Some(absolute) => Result::Ok(absolute.to_string()),
            None => Err(BranchError::NoShortcut(name.into())),
        }
    }

    pub(super) fn borrow_simple(&self, path: &str) -> Result<&Branch, BranchError> {
        //This can take ONLY ABSOLUTE and return reference
        match str::parse::<usize>(&path[1..]) {
            Ok(id) => match self.inventory.get(&id) {
                Some(branch) => Result::Ok(branch),
                None => Err(BranchError::NoBranch(id)),
            },
            Err(e) => Err(BranchError::InvalidId(e)),
        }
    }

    pub(super) fn borrow_simple_checked(&self, name: &str) -> Result<&Branch, BranchError> {
        //This can take RELATIVE/ABSOLUTE and return reference
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
            Err(BranchError::BorrowNoName)
        }
    }

    pub(super) fn borrow_linked_checked(&self, path: &str) -> Result<&Branch, BranchError> {
        //This can take chained ABSOLUTE/RELATIVE path and return reference
        match path.split_once('/') {
            None => self.borrow_simple_checked(path),
            Some((branch, remaining_path)) => match self.borrow_simple_checked(branch) {
                Ok(borrowed_widget) => borrowed_widget.borrow_linked_checked(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    pub(super) fn borrow_simple_mut(&mut self, path: &str) -> Result<&mut Branch, BranchError> {
        //This can take ONLY ABSOLUTE and return reference
        match str::parse::<usize>(&path[1..]) {
            Ok(id) => match self.inventory.get_mut(&id) {
                Some(branch) => Ok(branch),
                None => Err(BranchError::NoBranch(id)),
            },
            Err(e) => Err(BranchError::InvalidId(e)),
        }
    }

    pub(super) fn borrow_simple_checked_mut(&mut self, name: &str) -> Result<&mut Branch, BranchError> {
        //This can take RELATIVE/ABSOLUTE and return reference
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
            Err(BranchError::BorrowNoName)
        }
    }

    pub(super) fn borrow_linked_checked_mut(&mut self, path: &str) -> Result<&mut Branch, BranchError> {
        //This can take chained ABSOLUTE/RELATIVE path and return reference
        match path.split_once('/') {
            None => self.borrow_simple_checked_mut(path),
            Some((branch, remaining_path)) => match self.borrow_simple_checked_mut(branch) {
                Ok(borrowed_widget) => borrowed_widget.borrow_linked_checked_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    /*
    pub (in crate) fn destroy_simple (&mut self, path: &str) -> Result<(), String> {                                                       //This can take ONLY ABSOLUTE and return Option if the destruction succeded
        match path.chars().nth(1) {
            Some (value) => {
                match value {
                    'p' => Result::Err(String::from("Widgets with no name are supposed to be permanent and cannot be destroyed directly!")),
                    'r' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                if !self.removable.contains_key(&index) {
                                    return Result::Err(format!("Removable branch with key '{}' does not exist!", &index).to_string());
                                }
                                self.removable.remove(&index);
                                Result::Ok(())
                            },
                            Err (..) => Result::Err(format!("The path '{}' is not a valid number!", &path).to_string()),
                        }
                    },
                    _ => Result::Err(format!("The second character '{}' in '{}' needs to be either 'r' or 'p' (Stands for storage stack)!", &value, &path).to_string()),
                }
            },
            None => Result::Err(format!("Path '{}' is missing information (Example: #r12)!", &path).to_string()),
        }
    }
    pub (in crate) fn destroy_simple_checked (&mut self, key: &str) -> Result<(), String> {                                                    //This can take RELATIVE/ABSOLUTE and return Option if the destruction succeded
        match key.chars().next() {
            Some (_char) => match _char {
                '#' => self.destroy_simple(key),
                _ => match self.translate_simple(key){
                    Result::Ok (new_key) => self.destroy_chain(&new_key),
                    Result::Err (message) => Result::Err(message),
                },
            }
            None => Result::Err(String::from("There is no key!")),
        }
    }
    pub (in crate) fn destroy_chain (&mut self, path: &str) -> Result<(), String> {                                                            //This can take chained ABSOLUTE path and return Option if the destruction succeded
        match path.split_once('/') {
            None => {
                self.destroy_simple(path)
            },
            Some (tuple) => match self.borrow_simple_mut(tuple.0) {
                Result::Ok (borrowed_widget) => borrowed_widget.destroy_chain(tuple.1),
                Result::Err (message) => Result::Err(message),
            },
        }
    }
    pub (in crate) fn destroy_chain_checked (&mut self, keypath: &str) -> Result<(), String> {                                                 //This can take chained ABSOLUTE/RELATIVE path and return Option if the destruction succeded
        match keypath.split_once('/') {
            None => {
                self.destroy_simple_checked(keypath)
            },
            Some (tuple) => match self.borrow_simple_checked_mut(tuple.0) {
                Result::Ok (borrowed_widget) => borrowed_widget.destroy_simple_checked(tuple.1),
                Result::Err (message) => Result::Err(message),
            },
        }
    }

    pub (in crate) fn remove_simple_checked (&mut self, key: &str) -> Result<(), String> {                                                     //This can take ONLY RELATIVE and return Option if the widget was destroyed and removed from register
        if self.register.contains_key(key) {
            match self.destroy_chain_checked(key) {
                Result::Ok(_) => {
                    self.register.remove(key);
                    Result::Ok(())
                },
                Result::Err (message) => Result::Err(message),
            }
        } else {
            Result::Err(format!("Widget registered as '{}' does not exist!", &key).to_string())
        }
    }
    */
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
