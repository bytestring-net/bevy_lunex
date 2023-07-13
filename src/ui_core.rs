#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;
use colored::Colorize;

use crate::prelude::*;

use super::ui_container::{Container, PositionLayout};


//===========================================================================


#[derive(Component, Default)]
pub struct Hierarchy {
    pub width: f32,
    pub height: f32,
    branch: Branch,
}
impl Hierarchy {
    pub fn new () -> Hierarchy {
        //let mut branch = Branch::new(0.0, true, "ROOT", "".to_string());
        let mut branch = Branch::new("ROOT".to_string(), 0, "".to_string(), 0.0, true);
        branch.container.position_layout_set(Layout::Relative {
            relative_1: Vec2 { x: 0.0, y: 0.0 },
            relative_2: Vec2 { x: 100.0, y: 100.0 },
            ..Default::default()
        }.wrap());

        Hierarchy {
            width: 0.0,
            height: 0.0,
            branch,
        }
    }
    pub fn update (&mut self) {
        self.branch.cascade_update_self(Vec2::default(), self.width, self.height);
    }
    pub fn get_map (&self) -> String {
        let text = String::new();
        format!("{}{}", "#ROOT".purple().bold().underline(), self.branch.cascade_map(text, 0))
    }
    pub fn get_map_debug (&self) -> String {
        let text = String::new();
        format!("{}{}", "#ROOT".purple().bold().underline(), self.branch.cascade_map_debug(text, 0))
    }

    pub fn collect_paths (&self) -> Vec<String> {
        self.branch.collect_paths()
    }
    
    pub (in crate) fn root_get (&self) -> & Branch {
        &self.branch
    }
    pub (in crate) fn root_get_mut (&mut self) -> &mut Branch {
        &mut self.branch
    }
    
}

pub fn hierarchy_update(mut query: Query<&mut Hierarchy>, mut windows: Query<&mut Window>) {
    let window = windows.get_single_mut().unwrap();
    for mut system in &mut query {
        system.width = window.resolution.width();
        system.height = window.resolution.height();

        system.update();
    }
}


//===========================================================================


#[derive(Default)]
pub struct Branch {

    //# CACHING =======
    name: String,
    id: usize,
    path: String,

    //# RENDERING =======
    level: f32,
    depth: f32,
    active: bool,
    visible: bool,
    in_focus: bool,
    parent_visible: bool,

    //# MOUNTED DATA =======
    container: Container,
    data: Option<Data>,

    //# RECURSION =======
    inventory: HashMap<usize, Branch>,
    shortcuts: HashMap<String, String>,

}
impl Branch {
    //#USER EXPOSED CONTROL

    //Borrows
    pub fn data_get (&self) -> &Option<Data> {                                                                
        &self.data
    }
    pub fn data_get_mut (&mut self) -> &mut Option<Data> {                                                                
        &mut self.data
    }
    
    pub fn layout_get (&self) -> &PositionLayout {                                                                
        self.container.position_layout_get()
    }
    pub fn layout_get_mut (&mut self) -> &mut PositionLayout {                                                                
        self.container.position_layout_get_mut()
    }
    
    pub fn container_get (&self) -> &Container {                                                                
        &self.container
    }
    pub fn container_get_mut (&mut self) -> &mut Container {                                                                
        &mut self.container
    }

    //Fn calls
    pub fn get_name (&self) -> &String {
        &self.name
    }

    pub fn get_depth (&self) -> f32 {
        if self.in_focus {self.level + self.depth + 0.5} else {self.level + self.depth}
    }
    pub fn set_depth (&mut self, depth: f32) {
        self.cascade_set_depth_self(depth);
    }
    pub fn set_depth_self_only (&mut self, depth: f32) {
        self.cascade_set_depth(depth);
    }
    
    pub fn get_path (&self) -> String {
        if self.level == 0.0 {
            "".to_string()
        } else if !self.path.is_empty(){
            format!("{}/{}", self.path, self.name)
        } else {
            String::from(&self.name)
        }
    }
    
    pub fn get_focus (&self) -> bool {
        self.in_focus
    }
    pub fn set_focus (&mut self, focus: bool) {
        self.in_focus = focus;
    }

    pub fn is_visible (&self) -> bool {
        self.visible == true && self.parent_visible == true
    }
    pub fn get_visibility (&self) -> bool {
        self.visible
    }
    pub fn set_visibility (&mut self, visible: bool) {
        let old = self.is_visible();
        self.visible = visible;
        let new = self.is_visible();
        if new != old {
            self.cascade_set_visibility()
        }
    }

    pub fn get_map (&self) -> String {
        let text = String::new();
        format!("{}{}", self.name.purple().bold().underline(), self.cascade_map(text, 0))
    }
    pub fn get_map_debug (&self) -> String {
        let text = String::new();
        format!("{}{}", self.name.purple().bold().underline(), self.cascade_map_debug(text, 0))
    }

    pub fn collect_paths (&self) -> Vec<String> {
        let mut list = Vec::new();
        self.cascade_collect_paths(&mut list, "".to_string());
        list
    }

    //#LIBRARY RECURSION CALLS
    pub (in crate) fn cascade_map (&self, mut string: String, level: u32) -> String {                                                
        for (name, path) in self.shortcuts.iter(){
            match self.borrow_linked_checked(&path){
                Ok (widget) => {

                    let mut text = String::from("\n  ");
                    for _ in 0..level {text += "|    "}
                    text += "|-> ";
                    string = format!("{}{}{}", string, text.black(), name.bold().yellow());

                    string = widget.cascade_map(string, level + 1);
                },
                Err(..) => (),
            }
        }
        string
    }
    pub (in crate) fn cascade_map_debug (&self, mut string: String, level: u32) -> String {                                              
        let mut done_widgets: HashMap<String, bool> = HashMap::new();
        string = format!("{}{}", string, format!(" - [{}-#{}] [{}/{}] | ({}/{})", self.name, self.id, self.level, self.get_depth(), self.visible, self.parent_visible).black().italic());
        
        for (name, path) in self.shortcuts.iter(){
            match self.borrow_linked_checked(&path){
                Ok (widget) => {

                    let mut text = String::from("\n  ");
                    for _ in 0..level {text += "|    "}
                    text += "|-> ";
                    string = format!("{}{}{} ({})", string, text.black(), name.bold().yellow(), path);

                    string = widget.cascade_map_debug(string, level + 1);
                    done_widgets.insert(path.to_string(), true);
                },
                Err(..) => {
                    let mut text = String::from("\n  ");
                    for _ in 0..level {text += "|    "}
                    text += "|-> ";
                    string = format!("{}{}{}", string, text.black(), format!("{} #[! Dangling register pointer !]", name).bold().red());
                },
            }
        }
        for x in self.inventory.iter(){
            if done_widgets.contains_key( &("#".to_string() + &x.0.to_string())) {
                continue;
            }
            
            let mut text = String::from("\n  ");
            for _ in 0..level {text += "|    "}
            text += "|-> ";
            string = format!("{}{}{}", string, text.black(), format!("#{}", x.0).bold().truecolor(255, 165, 214));

            string = x.1.cascade_map_debug(string, level + 1);
        }
        string
    }

    pub (in crate) fn cascade_collect_paths (&self, list: &mut Vec<String>, directory: String) {                                              
        let mut done_widgets: HashMap<String, bool> = HashMap::new();
        
        for (name, path) in self.shortcuts.iter(){
            match self.borrow_linked_checked(&path){
                Ok (widget) => {

                    let dir = if directory.is_empty() {
                        String::from(name)
                    } else {
                        format!("{}/{}", directory, name)
                    };
                    list.push(dir.clone());
                    widget.cascade_collect_paths(list, dir);

                    done_widgets.insert(path.to_string(), true);
                },
                Err(..) => {},
            }
        }
        for x in self.inventory.iter(){
            if done_widgets.contains_key( &("#".to_string() + &x.0.to_string())) {
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

    pub (in crate) fn cascade_update_self (&mut self, point: Vec2, width: f32, height: f32) {                                       //This will cascade update all branches
        self.container.update(point, width, height);
        for x in self.inventory.iter_mut(){
            let pos = self.container.position_get();
            x.1.cascade_update_self(pos.point_1, pos.width, pos.height);
        }
    }

    pub (in crate) fn cascade_set_visibility (&mut self) {                                                                              //This will cascade set parent visible all branches
        let visibility = self.is_visible();
        for x in self.inventory.iter_mut(){
            let pos = self.container.position_get();
            x.1.cascade_set_visibility_self(visibility);
        }
    }
    pub (in crate) fn cascade_set_visibility_self (&mut self, visible: bool) {                                                          //This will cascade set parent visible all branches
        self.parent_visible = visible;
        self.cascade_set_visibility()
    }
    
    pub (in crate) fn cascade_set_depth (&mut self, depth: f32) {                                                                       //This will cascade set parent visible all branches
        for x in self.inventory.iter_mut(){
            let pos = self.container.position_get();
            x.1.cascade_set_depth_self(depth);
        }
    }
    pub (in crate) fn cascade_set_depth_self (&mut self, depth: f32) {                                                                  //This will cascade set parent visible all branches
        self.depth = depth;
        self.cascade_set_depth(depth);
    }

    //#LIBRARY MECHANISMS
    fn new (name: String, id: usize, path: String, level: f32, parent_visible: bool) -> Branch {
        Branch {
            name,
            id,
            path,

            level,
            depth: 100.0,
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

    pub (in crate) fn create_simple (&mut self, name: &str, position: PositionLayout) -> String {
        
        let mut id = 0;
        loop {if !self.inventory.contains_key(&id) {break} else {id += 1}}

        let path = if name.is_empty() {format!("{}/#{}", self.get_path(), id)} else {format!("{}/{}", self.get_path(), name)};
        let mut branch = Branch::new(name.to_string(), id, path, self.level + 1.0, self.is_visible());

        branch.container.position_layout_set(position);

        self.inventory.insert(id, branch);
        format!("#{}", id)

    }
    pub (in crate) fn create_linked (&mut self, name: &str, position: PositionLayout) -> Result<String, String> {
        if name.is_empty() {
            Result::Ok(self.create_simple("", position))
        } else {
            if !self.shortcuts.contains_key(name) {

                let path = self.create_simple(name, position);
                self.shortcuts.insert(name.to_string(), path);
                Result::Ok(name.to_string())

            } else {
                Result::Err(format!("The name '{}' is already in use!", name))
            }
        }
    }

    pub (in crate) fn register_path (&mut self, name: String, path: String) -> Result<(), String> {                                                         //This registers ABSOLUTE PATH for a key
        if self.shortcuts.contains_key(&name) {return Result::Err(format!("Branch already contains a path for name {}", &name));}
        self.shortcuts.insert(name, path);
        Result::Ok(())
    }
    pub (in crate) fn translate_simple (&self, name: &str) -> Result<String, String> {                                               //This can take ONLY RELATIVE and return ABSOLUTE
        match self.shortcuts.get(name) {
            Some (absolute) => Result::Ok(absolute.to_string()),
            None => Result::Err(format!("There is no shortcut for '{}'!", &name)),
        }
    }

    pub (in crate) fn borrow_simple (&self, path: &str) -> Result<&Branch, String> {                                                //This can take ONLY ABSOLUTE and return reference
        match str::parse::<usize>(&path[1..]) {
            Result::Ok (id) => {
                match self.inventory.get(&id) {
                    Option::Some (branch) => {
                        Result::Ok(branch)
                    },
                    Option::None => Result::Err(format!("Branch with id '#{}' doesn't exist!", &id)),
                }
            },
            Result::Err (..) => Result::Err(format!("Invalid syntax in path '{}'!", path)),
        }
    }
    pub (in crate) fn borrow_simple_checked (&self, name: &str) -> Result<&Branch, String> {                                         //This can take RELATIVE/ABSOLUTE and return reference
        if !name.is_empty() {
            if is_absolute(name){
                self.borrow_simple(name)
            } else {
                match self.translate_simple(name){
                    Ok (path) => self.borrow_linked_checked(&path),
                    Err (message) => Result::Err(message),
                }
            }
        } else {
            Result::Err("Cannot borrow branch with no name!".to_string())
        }
    }
    pub (in crate) fn borrow_linked_checked (&self, path: &str) -> Result<&Branch, String> {                                      //This can take chained ABSOLUTE/RELATIVE path and return reference
        match path.split_once('/') {
            None => self.borrow_simple_checked(path),
            Some ((branch, remaining_path)) => match self.borrow_simple_checked(branch) {
                Ok (borrowed_widget) => borrowed_widget.borrow_linked_checked(remaining_path),
                Err (message) => Result::Err(message),
            },
        }
    }

    pub (in crate) fn borrow_simple_mut (&mut self, path: &str) -> Result<&mut Branch, String> {                                                //This can take ONLY ABSOLUTE and return reference
        match str::parse::<usize>(&path[1..]) {
            Result::Ok (id) => {
                match self.inventory.get_mut(&id) {
                    Option::Some (branch) => {
                        Result::Ok(branch)
                    },
                    Option::None => Result::Err(format!("Branch with id '#{}' doesn't exist!", &id)),
                }
            },
            Result::Err (..) => Result::Err(format!("Invalid syntax in path '{}'!", path)),
        }
    }
    pub (in crate) fn borrow_simple_checked_mut (&mut self, name: &str) -> Result<&mut Branch, String> {                                         //This can take RELATIVE/ABSOLUTE and return reference
        if !name.is_empty() {
            if is_absolute(name){
                self.borrow_simple_mut(name)
            } else {
                match self.translate_simple(name){
                    Ok (path) => self.borrow_linked_checked_mut(&path),
                    Err (message) => Result::Err(message),
                }
            }
        } else {
            Result::Err("Cannot borrow branch with no name!".to_string())
        }
    }
    pub (in crate) fn borrow_linked_checked_mut (&mut self, path: &str) -> Result<&mut Branch, String> {                                      //This can take chained ABSOLUTE/RELATIVE path and return reference
        match path.split_once('/') {
            None => self.borrow_simple_checked_mut(path),
            Some ((branch, remaining_path)) => match self.borrow_simple_checked_mut(branch) {
                Ok (borrowed_widget) => borrowed_widget.borrow_linked_checked_mut(remaining_path),
                Err (message) => Result::Err(message),
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


//===========================================================================


pub struct Data {
    pub f32s: HashMap<String, f32>,
    pub vec2s: HashMap<String, Vec2>,
    pub vec3s: HashMap<String, Vec3>,
    pub vec4s: HashMap<String, Vec4>,
    pub bools: HashMap<String, bool>,
    pub strings: HashMap<String, String>,
}
impl Data {
    pub fn new () -> Data {
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