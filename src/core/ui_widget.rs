use super::export::*;
use bevy::prelude::*;
use super::ui_core::Branch;

// ===========================================================
// === MAIN WIDGET STRUCT ===

#[derive(Component, Default, Clone, Debug, PartialEq)]
pub struct Widget {
    path: String,
    name: String,
}
impl Widget {

    
    /*
    pub fn fetch_layout<'a> (&'a self, system: &'a  Hierarchy, key: &str) -> Result<&PositionLayout, String> {
        match self.fetch(system, key){
            Ok (branch) => Result::Ok(branch.container_get().position_layout_get()),
            Err (message) => Err(message),
        }
    }
    pub fn fetch_layout_mut<'a> (&'a self, system: &'a mut Hierarchy, key: &str) -> Result<&mut PositionLayout, String> {
        match self.fetch_mut(system, key){
            Ok (branch) => Result::Ok(branch.container_get_mut().position_layout_get_mut()),
            Err (message) => Err(message),
        }
    }
    pub fn fetch_data<'a> (&'a self, system: &'a  Hierarchy, key: &str) -> Result<&Option<Data>, String> {
        match self.fetch(system, key){
            Ok (branch) => Result::Ok(branch.data_get()),
            Err (message) => Err(message),
        }
    }
    pub fn fetch_data_mut<'a> (&'a self, system: &'a mut Hierarchy, key: &str) -> Result<&mut Option<Data>, String> {
        match self.fetch_mut(system, key){
            Ok (branch) => Result::Ok(branch.data_get_mut()),
            Err (message) => Err(message),
        }
    }
    pub fn fetch_position<'a> (&'a self, system: &'a Hierarchy, key: &str) -> Result<&Position, String> {
        match self.fetch(&system, key) {
            Ok (branch) => Result::Ok(&branch.container_get().position_get()),
            Err (message) => Result::Err(message),
        }
    }
    pub fn fetch_position_mut<'a> (&'a self, system: &'a mut Hierarchy, key: &str) -> Result<&mut Position, String> {
        match self.fetch_mut(system, key) {
            Ok (branch) => Result::Ok(branch.container_get_mut().position_get_mut()),
            Err (message) => Result::Err(message),
        }
    }
    */
    //add is cursor_within + depth

/* 
    pub fn destroy (&self, system: &mut Hierarchy, path : &str) -> Result<(), String> {
        match system.root_get_mut().borrow_linked_checked_mut(&self.path){
            Ok (reference) => {
                reference.destroy_chain_checked(path)
            },
            Err (message) => Result::Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn remove (&self, system: &mut Hierarchy, key : &str) -> Result<(), String> {
        match system.root_get_mut().borrow_linked_checked_mut(&self.path){
            Ok (reference) => {
                reference.remove_simple_checked(key)
            },
            Err (message) => Result::Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
*/


    //# POINTER RELATED

    /// ## Description
    /// This function by itself does NOTHING except creating a pointer from provided path.
    /// It does NOT SYNCHRONIZE with any hierarchy and doesn't change anything.
    /// 
    /// If you want to actually create new widget, use ``Widget::Create``
    /// 
    /// This is just a pointer to call on more advanced methods later.
    ///
    /// ## Examples
    /// ```
    /// let button = Widget::new("Button");
    /// let setting_button = Widget::new("Settings/Button");
    /// ```
    pub fn new (path: &str) -> Widget {
        Widget { 
            path: path.to_string(),
            name: split_last(path, "/").1,
        }
    }

    /// ## Description
    /// This function will try to return &[`Branch`], located inside [`Hierarchy`] based on the widgets path.
    /// 
    /// If you want to interact with the Hierarchy, you use this to get a borrow.
    ///
    /// ## Examples
    /// ```
    /// let system = Hierarchy::new();
    /// 
    /// //This is only a pointer
    /// let menu_pointer = Widget::create(&mut system, "Menu", PositionLayout::Default);
    /// let button_pointer = Widget::create(&mut system, &menu_pointer.end("Button"), PositionLayout::Default);
    /// 
    /// //This is the actual 'widget' that you can manipulate
    /// let menu: &Branch = menu_pointer.fetch(&system, "").unwrap();   //Leave blank for self
    /// let button: &Branch = menu_pointer.fetch(&system, "Button").unwrap(); //You can locate sub-widgets
    /// 
    /// ```
    pub fn fetch<'a> (&'a self, system: &'a  Hierarchy, path: &str) -> Result<&Branch, String> {
        let mut extra_path = String::from(&self.path);
        if !path.is_empty() { extra_path += "/";extra_path += path;}
        match system.root_get().borrow_linked_checked(&extra_path){
            Ok (branch) => Result::Ok(branch),
            Err (message) => Err(format!("Fetch failed, could not find '{}' because: {}", &extra_path, message)),
        }
    }
    /// ## Description
    /// This function will try to return &mut [`Branch`], located inside [`Hierarchy`] based on the widgets path.
    /// 
    /// If you want to interact with the Hierarchy, you use this to get a borrow.
    ///
    /// ## Examples
    /// ```
    /// let system = Hierarchy::new();
    /// 
    /// //This is only a pointer
    /// let menu_pointer = Widget::create(&mut system, "Menu", PositionLayout::Default);
    /// let button_pointer = Widget::create(&mut system, &menu_pointer.end("Button"), PositionLayout::Default);
    ///                                                 //      Menu/Button       //
    /// 
    /// //This is the actual 'widget' that you can manipulate
    /// let menu: &mut Branch = menu_pointer.fetch_mut(&mut system, "").unwrap();   //Leave blank for self
    /// let button: &mut Branch = menu_pointer.fetch_mut(&mut system, "Button").unwrap(); //You can locate sub-widgets
    /// 
    /// ```
    pub fn fetch_mut<'a> (&'a self, system: &'a mut Hierarchy, path: &str) -> Result<&mut Branch, String> {
        let mut extra_path = String::from(&self.path);
        if !path.is_empty() { extra_path += "/";extra_path += path;}
        match system.root_get_mut().borrow_linked_checked_mut(&extra_path){
            Ok (branch) => Result::Ok(branch),
            Err (message) => Err(format!("Fetch failed, could not find '{}' because: {}", &extra_path, message)),
        }
    }


    //# SIMPLE CREATION

    /// ## Description
    /// This function is the one you create new widgets with. It creates a [`Widget`] on the path specified inside the hierarchy.
    /// 
    /// Paths:
    /// * ``Menu`` -> Create widget ``Menu`` in ``#ROOT``
    /// * ``Menu/Category`` -> Create widget ``Category`` in ``Menu``
    /// * ``Menu/Category/Button`` -> Create widget ``Button`` in ``Category`` (Located at ``Menu/Category``)
    ///
    /// ## Example
    /// ```
    /// let menu_pointer = Widget::create(&mut system, "Menu", PositionLayout::Default);
    /// let button_pointer = Widget::create(&mut system, "Menu/Button", PositionLayout::Default); //Not recommended way of defining path
    /// ```
    /// 
    /// The string after the last '/' is the name of the [`Widget`] and the rest is path to the parent [`Widget`].
    /// Note that manually setting path is bad practice, it is recommended to use `.end()` method on [`Widget`]
    /// 
    /// ## Nameless widgets
    /// You are also able to create 'nameless' widgets. They are good for non-important widgets like
    /// one time use or used only for layout purposes. (Don't require interactivity).
    /// Leave an empty string for name to create nameless [`Widget`]. Their name will be generated.
    /// 
    /// ```
    /// let menu_pointer = Widget::create(&mut system, "", PositionLayout::Default);
    /// let button_pointer = Widget::create(&mut system, &menu_pointer.end(""), PositionLayout::Default);
    /// ```
    /// In this case the path of ``button_pointer`` is `` #0/#0 `` (The number stands for an order they were created in)
    /// 
    pub fn create (system: &mut Hierarchy, path: &str, position: Layout) -> Result <Widget, String> {

        let str_list: Vec<&str> =  path.split('/').collect();
        let str_list_len = str_list.len();

        let mut parent_path = String::new();
        let name = String::from(str_list[str_list_len-1]);
        
        let mut n = if str_list_len != 0 { str_list_len - 1} else {0};

        //# This will check for skippable paths (Menu/#0/#0/Display -> Menu/Display)
        let mut absolute = String::new(); // => #0/#0
        if !name.is_empty() && !is_absolute(&name) && str_list_len > 1 {
            let mut i = str_list_len - 2;
            while is_absolute(str_list[i]) && i > 0 {
                absolute = format!("{}/{}", str_list[i], absolute);
                i -= 1;
            }
            if absolute.contains("/") { absolute = absolute[..absolute.len() - 1].to_string()}
            n = i+1;
        }

        //# Collect the remaining iterator into path
        for ii in 0..n {
            if ii != 0 {parent_path += "/"}
            parent_path += str_list[ii];
        }
        


        //# Create branch in ROOT
        if parent_path.is_empty() {
            let parent_branch = system.root_get_mut();
            match parent_branch.create_linked(&name, position) {
                Result::Ok (absolute_key) => {
                    let widget = if name.is_empty() { Widget::new(&absolute_key) } else { Widget::new(path) };
                    widget.fetch_mut(system, "").unwrap().set_visibility(false);
                    Result::Ok (widget)
                },
                Result::Err (message) => Result::Err(message),
            }
        
        //# Create branch in branch
        } else {
            match Widget::new(&parent_path).fetch_mut(system, "") {
                Result::Ok (parent_branch) => {
                    if !absolute.is_empty() == true {

                        //println!("Name: {}, Path: {}, PPath: {}, obs: {}/###", name, path, parent_path, absolute);
                        
                        //# Create branch with skip
                        let absolute_key = match parent_branch.borrow_linked_checked_mut(&absolute){
                            Result::Ok (nameless_branch) => match nameless_branch.create_linked(&name, position) {
                                Result::Ok (absolute_key) => absolute_key,
                                Result::Err (message) => return Result::Err(message),
                            },
                            Result::Err (message) => return Result::Err(message),
                        };
                        match parent_branch.register_path(name, format!("{}/{}", absolute, absolute_key)) {
                            Result::Ok (..) => Result::Ok (Widget::new(&format!("{}/{}", parent_path, absolute_key))),
                            Result::Err(message) => Result::Err(message),
                        }

                    } else {

                        //# Create direct branch without skipping
                        match parent_branch.create_linked(&name, position) {
                            Result::Ok (absolute_key) => {
                                if name.is_empty() {
                                    Result::Ok (Widget::new(&format!("{}/{}", parent_path, absolute_key)))
                                } else {
                                    Result::Ok (Widget::new(path))
                                }
                            },
                            Result::Err (message) => Result::Err(message),
                        }

                    }
                },
                Result::Err (message) => Result::Err(message),
            }
        }
    }

    //# PATHING SYSTEM

    /// ## Description
    /// This method is used to create dynamic path to widgets.
    /// It is used in combination of ``.add_str()`` and ``.end()``.
    /// 
    /// ## Examples
    /// ```
    /// use bevy_lunex::prelude::*;
    /// 
    /// let mut system = Hierarchy::new();
    /// 
    /// let menu = Widget::create(&mut system, "Menu", PositionLayout::Default).unwrap();
    /// let category = Widget::create(&mut system, &menu.end("Category"), PositionLayout::Default).unwrap();
    /// 
    /// let path = menu.add(&category).end("Button");
    /// assert_eq!("Menu/Category/Button", path);
    /// 
    /// ```
    pub fn add (&self, w: &Widget) -> Widget {
        Widget::new(&format!("{}/{}", self.path, w.name))
    }
    pub fn add_str (&self, s: &str) -> Widget {
        Widget::new(&format!("{}/{}", self.path, s))
    }
    pub fn end (&self, s: &str) -> String {
        format!("{}/{}", self.path, s)
    }

    //# FUNCTIONALITY
    pub fn is_within (&self, system: &Hierarchy, path: &str, point: &Vec2) -> Result<bool, String> {
        match self.fetch(&system, path) {
            Ok (branch) => {
                let position = branch.container_get().position_get();
                Result::Ok((point.x > position.point_1.x && point.x < position.point_2.x) && (point.y > position.point_1.y && point.y < position.point_2.y))
            },
            Err (message) => Result::Err(format!("Point is_within failed because: {}", message)),
        }
    }

}