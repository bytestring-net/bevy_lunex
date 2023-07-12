#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::*;
use bevy::prelude::*;

use super::ui_container::{Position, PositionLayout};
use super::ui_core::Branch;

//===========================================================================

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Widget {
    path: String,
    key: String,
}
impl Widget {

    pub fn fetch<'a> (&'a self, system: &'a  Hierarchy, key: &str) -> Result<&Branch, String> {
        let mut extra_path = String::from(&self.path);
        if !key.is_empty() { extra_path += "/";extra_path += key;}
        match system.root_get().borrow_chain_checked(&extra_path){
            Ok (branch) => Result::Ok(branch),
            Err (message) => Err(format!("Fetch failed, could not find '{}' #REASON: {}", &extra_path, message).to_string()),
        }
    }
    pub fn fetch_mut<'a> (&'a self, system: &'a mut Hierarchy, key: &str) -> Result<&mut Branch, String> {
        let mut extra_path = String::from(&self.path);
        if !key.is_empty() { extra_path += "/";extra_path += key;}
        match system.root_get_mut().borrow_chain_checked_mut(&extra_path){
            Ok (branch) => Result::Ok(branch),
            Err (message) => Err(format!("Fetch failed, could not find '{}' #REASON: {}", &extra_path, message).to_string()),
        }
    }
    
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
    
    pub fn is_within (&self, system: &Hierarchy, key: &str, point: &Vec2) -> Result<bool, String> {
        match self.fetch_position(&system, key) {
            Ok (position) => Result::Ok((point.x > position.point_1.x && point.x < position.point_2.x) && (point.y > position.point_1.y && point.y < position.point_2.y)),
            Err (message) => Result::Err(message),
        }
    }
    //add is cursor_within + depth

    pub fn get_path (&self) -> &String {
        &self.path
    }
    pub fn from_path (path: &str) -> Widget {
        Widget { 
            path: path.to_string(),
            key: MString::split_last(path, "/").1,
        }
    }
    /*
    pub fn new(system: &mut Hierarchy, key: &str, position: PositionLayout) -> Result<Widget, String> {
        match system.root_get_mut().create_simple_checked(key, position) {
            Ok (new_key) => {
                let widget = Widget::from_path(&new_key);
                widget.fetch_mut(system, "").unwrap().set_visibility(false);
                Result::Ok(widget)
            },
            Err (message) => Err(String::from("UNABLE TO CREATE WIDGET! #Error: ") + &message),
        }
    }
    pub fn new_in(system: &mut Hierarchy, widget: &Widget, key: &str, position: PositionLayout) -> Result <Widget, String> {
        match key.split_once('/') {
            None => {
                match system.root_get_mut().borrow_chain_checked_mut(&widget.path){
                    Ok (reference) => match reference.create_simple_checked(key, position) {
                        Ok (new_key) => Result::Ok(Widget::from_path(&(String::new() + &widget.path + "/"+ &new_key))),
                        Err (message) => Result::Err(message),
                    },
                    Err (message) => Err(String::from("WIDGET '") + &widget.path + "' NOT FOUND! #Error: "+ &message),
                }
            },
            Some (tuple) => {
                let mut path = String::new();
                let tuple1 = MString::split_last(key,"/");       // xxx/xxx/xxx - yyy
                let is_rooted:bool = match tuple1.0.split_once('/'){             // xxx - xxx/xxx
                    Some (tuple2) => if tuple2.0 == "#ROOT" {path += tuple2.1;true} else {path += &tuple1.0;false},
                    None =>  if tuple1.0 == "#ROOT" {true} else {path += &tuple1.0;false},
                };

                if is_rooted {
                    if path.is_empty() {
                        Result::Err(String::from("THIS KEY IS ILLEGAL!"))
                    } else {
                        let source = match system.root_get_mut().translate_chain_checked(&path){
                            Ok (source) => source,
                            Err (message) => return Result::Err(message),
                        };
                        let substring = match system.root_get_mut().translate_chain_checked(&widget.path){
                            Ok (substring) => substring,
                            Err (message) => return Result::Err(message),
                        };

                        let _path = MString::subtract_void(&source, &substring);

                        let new_key: String = match system.root_get_mut().borrow_chain_mut(&source) {
                            Ok (set_widget) => {
                                set_widget.create_simple(true, position, &tuple1.1)
                            },
                            Err (message) => return Result::Err(message),
                        };
                        match system.root_get_mut().borrow_chain_mut(&substring) {
                            Ok (register_widget) => {
                                register_widget.register_path(String::from(&tuple1.1), _path + "/" + &new_key)
                            },
                            Err (message) => return Result::Err(message),
                        };

                        Result::Ok(Widget::from_path(&(String::new() + &widget.path + "/" + &tuple1.1)))
                    }
                } else {
                    let mut new_local_path = String::from(&path) + "/";
                    match system.root_get_mut().borrow_chain_checked_mut(&(String::from(&widget.path)+ "/" + &path)) {
                        Ok (set_widget) => {
                            new_local_path += &set_widget.create_simple(true, position, &tuple1.1);
                        },
                        Err (message) => return Err(String::from("CRASHED ON MAKING NEW BRANCH! #Error: ") + &message),
                    };
                    match system.root_get_mut().borrow_chain_checked_mut(&widget.path) {
                        Ok (register_widget) => {
                            register_widget.register_path(String::from(&tuple1.1), new_local_path.clone())
                        },
                        Err (message) => return Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
                    };
                    Result::Ok(Widget::from_path(&(String::new() + &widget.path + "/" + &tuple1.1)))
                }
            },
        }
    }
    */

    pub fn chain_str (&self, str: &str) -> String {
        format!("{}/{}", self.path, str)
    }

    pub fn destroy (&self, system: &mut Hierarchy, path : &str) -> Result<(), String> {
        match system.root_get_mut().borrow_chain_checked_mut(&self.path){
            Ok (reference) => {
                reference.destroy_chain_checked(path)
            },
            Err (message) => Result::Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn remove (&self, system: &mut Hierarchy, key : &str) -> Result<(), String> {
        match system.root_get_mut().borrow_chain_checked_mut(&self.path){
            Ok (reference) => {
                reference.remove_simple_checked(key)
            },
            Err (message) => Result::Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }



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
    
    //FIX DISABLING GAP ISSUES
    pub fn generate_grid_in_solid (system: &mut Hierarchy, path: &str, grid: &Vec<Vec<&str>>, anchor: Vec2, style: &WidgetListStyle) -> Result<Widget, String>{
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
        

        let widget = match Widget::create(system, path, Layout::Solid {
            horizontal_anchor: anchor.x,
            vertical_anchor: anchor.y,
            width: (container_width*10.0) as u32,
            height: (container_height*10.0) as u32,
            scaling: Scale::Fit,
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



    //REFACTORED
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
            match parent_branch.create_simple_checked(&name, position) {
                Result::Ok (absolute_key) => {
                    let widget = if name.is_empty() { Widget::from_path(&absolute_key) } else { Widget::from_path(path) };
                    widget.fetch_mut(system, "").unwrap().set_visibility(false);
                    Result::Ok (widget)
                },
                Result::Err (message) => Result::Err(message),
            }
        
        //# Create branch in branch
        } else {
            match Widget::from_path(&parent_path).fetch_mut(system, "") {
                Result::Ok (parent_branch) => {
                    if !absolute.is_empty() == true {

                        //println!("Name: {}, Path: {}, PPath: {}, obs: {}/###", name, path, parent_path, absolute);
                        
                        //# Create branch with skip
                        let absolute_key = match parent_branch.borrow_chain_checked_mut(&absolute){
                            Result::Ok (nameless_branch) => match nameless_branch.create_simple_checked(&name, position) {
                                Result::Ok (absolute_key) => absolute_key,
                                Result::Err (message) => return Result::Err(message),
                            },
                            Result::Err (message) => return Result::Err(message),
                        };
                        match parent_branch.register_path(name, format!("{}/{}", absolute, absolute_key)) {
                            Result::Ok (..) => Result::Ok (Widget::from_path(&format!("{}/{}", parent_path, absolute_key))),
                            Result::Err(message) => Result::Err(message),
                        }

                    } else {

                        //# Create direct branch without skipping
                        match parent_branch.create_simple_checked(&name, position) {
                            Result::Ok (absolute_key) => {
                                if name.is_empty() {
                                    Result::Ok (Widget::from_path(&format!("{}/{}", parent_path, absolute_key)))
                                } else {
                                    Result::Ok (Widget::from_path(path))
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

    pub fn add (&self, w: &Widget) -> Widget {
        Widget::from_path(&format!("{}/{}", self.path, w.key))
    }
    pub fn add_str (&self, s: &str) -> Widget {
        Widget::from_path(&format!("{}/{}", self.path, s))
    }
    pub fn end (&self, s: &str) -> String {
        format!("{}/{}", self.path, s)
    }


}

fn is_absolute (str: &str) -> bool {
    match str.chars().nth(0) {
        Some (value) => {
            value == '#'
        },
        None => false,
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