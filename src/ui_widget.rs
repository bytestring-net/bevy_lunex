#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::*;
use bevy::prelude::*;

use super::ui_container::PositionLayout;
use super::ui_core::Branch;

//===========================================================================

#[derive(Component, Default, Debug, Clone, PartialEq)]
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
    /// If you want to actually create new widget, use ```Widget::Create```
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
    ///                                                 //      Menu/Button       //
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
    /// This function is the one you create new widgets with
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
    /// ```
    pub fn create (system: &mut Hierarchy, path: &str, position: PositionLayout) -> Result <Widget, String> {

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
    pub fn add (&self, w: &Widget) -> Widget {
        Widget::new(&format!("{}/{}", self.path, w.name))
    }
    pub fn add_str (&self, s: &str) -> Widget {
        Widget::new(&format!("{}/{}", self.path, s))
    }
    pub fn end (&self, s: &str) -> String {
        format!("{}/{}", self.path, s)
    }

    //# ADVANCED CREATION
    pub fn generate_grid (system: &mut Hierarchy, path: &String, grid: &Vec<Vec<&str>>, relative: Vec2, style: &WidgetListStyle) -> Result<Widget, String>{
        let xx = grid.len();
        let yy = grid[0].len();
    
        let total_width = style.width_relative * xx as f32;
        let total_height = style.height_relative * yy as f32;
    
        let xi = if style.width_padding_gap == true {1.0} else {-1.0};
        let yi = if style.height_padding_gap == true {1.0} else {-1.0};
    
        let total_wgap = style.gap_relative.x * (xx as f32 + xi);
        let total_hgap = style.gap_relative.y * (yy as f32 + yi);
    
        let container_width = total_width + total_wgap;
        let container_height = total_height + total_hgap;
    
        let widget = match Widget::create(system, path, Layout::Window {
            relative,
            width_relative: container_width,
            height_relative: container_height,
            ..Default::default()
        }.wrap()) {
            Result::Ok (widget) => widget,
            Result::Err(message) => return Result::Err(message),
        };
    
        let width = (100.0 * total_width/container_width)/xx as f32;
        let height = (100.0 * total_height/container_height)/yy as f32;
    
        let wgap = (100.0 * total_wgap/container_width)/(xx as f32 + xi);
        let hgap = (100.0 * total_hgap/container_height)/(yy as f32 + xi);
    
        for x in 0..xx {
            for y in 0..yy {
                match Widget::create(system, &widget.end(grid[x][y]), Layout::Window {
                    relative: Vec2::new(
                        width*x as f32 + wgap*x as f32 + if style.width_padding_gap == true {wgap} else {0.0},
                        height*y as f32 + hgap*y as f32 + if style.height_padding_gap == true {hgap} else {0.0},
                    ),
                    width_relative: width,
                    height_relative: height,
                    ..Default::default()
                }.wrap()) {
                        Result::Ok (..) => (),
                        Result::Err(message) => return Result::Err(message),
                };
            }
        }
        Result::Ok(widget)
    }
    pub fn generate_grid_inside (system: &mut Hierarchy, widget: &Widget, grid: &Vec<Vec<&str>>, style: &WidgetListStyle) -> Result<(), String>{
        let xx = grid.len();
        let yy = grid[0].len();
    
        let total_width = style.width_relative * xx as f32;
        let total_height = style.height_relative * yy as f32;
    
        let xi = if style.width_padding_gap == true {1.0} else {-1.0};
        let yi = if style.height_padding_gap == true {1.0} else {-1.0};
    
        let total_wgap = style.gap_relative.x * (xx as f32 + xi);
        let total_hgap = style.gap_relative.y * (yy as f32 + yi);
    
        let container_width = total_width + total_wgap;
        let container_height = total_height + total_hgap;
    
        let width = (100.0 * total_width/container_width)/xx as f32;
        let height = (100.0 * total_height/container_height)/yy as f32;
    
        let wgap = (100.0 * total_wgap/container_width)/(xx as f32 + xi);
        let hgap = (100.0 * total_hgap/container_height)/(yy as f32 + xi);
    
        for x in 0..xx {
            for y in 0..yy{
                match Widget::create(system, &widget.end(grid[x][y]), Layout::Window {
                    relative: Vec2::new(
                        width*x as f32 + wgap*x as f32 + if style.width_padding_gap == true {wgap} else {0.0},
                        height*y as f32 + hgap*y as f32 + if style.height_padding_gap == true {hgap} else {0.0},
                    ),
                    width_relative: width,
                    height_relative: height,
                    ..Default::default()
                }.wrap()) {
                        Result::Ok (..) => (),
                        Result::Err(message) => return Result::Err(message),
                };
            }
        }
        Result::Ok(())
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



#[derive(Default)]
pub struct WidgetListStyle {
    pub gap_relative: Vec2,
    pub width_relative: f32,
    pub height_relative: f32,
    pub width_padding_gap: bool,
    pub height_padding_gap: bool,
}