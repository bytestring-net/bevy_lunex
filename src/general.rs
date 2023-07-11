#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;
use bevy::sprite::Anchor;
use colored::Colorize;
use crate::prelude::*;

use crate::prelude::HashMap;

pub struct MString {}
impl MString {
    pub fn construct_from (template: &str, data: HashMap<String, String>) -> Result<String, String> {
        let mut level = 0;
        let mut name = String::new();
        let mut result = String::new();
        for character in template.chars() {
            if character == '}' {
                level -= 1;
                match data.get(&name){
                    None => return Err(String::from("Error while constructing MString - '") + &name + "' is not defined!"),
                    Some (value) => result += value,
                }
                name.clear();
            }
            if level == 1 {name.push(character);}
            if character == '{' {level += 1;}

            if level == 0 && character != '}'{
                result.push(character);
            }
        }
        if level != 0 {return Err(String::from("Error while constructing MString - wrong use of brackets!"));}
        Ok(result)
    }
    pub fn split_last (string: &str, delimiter: &str ) -> (String, String) {
        let str_list: Vec<&str> =  string.split(delimiter).collect();
        let mut output = String::new();
        let mut is_first = true;
        for x in str_list.iter().take(str_list.len() - 1){
            if !is_first {output += delimiter} else {is_first = false};
            output += x;
        }
        (output, String::from(str_list[str_list.len() - 1]))
    }
    pub fn subtract (string: &str, substring: &str) -> String {             // ABCDE - ABG = CDE
        let mut strip = string.chars();
        let mut substrip = substring.chars();
        for i in 0..strip.clone().count() {
            let char = strip.next();
            if char != substrip.next() {
                return String::from(char.unwrap_or('\0')) + strip.as_str();
            }
        }
        return String::from(strip.as_str());
    }
    pub fn subtract_void (string: &str, substring: &str) -> String {   // ABCDE - ABG = DE
        let mut strip = string.chars();
        let mut substrip = substring.chars();
        for i in 0..strip.clone().count() {
            if strip.next() != substrip.next() {
                return String::from(strip.as_str());
            }
        }
        return String::from(strip.as_str());
    }
}

pub fn vec_convert (vec2: &Vec2, offset: &Vec2) -> Vec2 {
    Vec2::new(vec2.x - offset.x, offset.y - vec2.y)
}



#[derive(Component)]
struct DebugImage ();
pub fn lunex_setup_debug (mut commands: Commands, asset_server: Res<AssetServer>, systems: Query<&Hierarchy>) {
    for system in systems.iter() {
        for x in system.collect_paths(){
            let widget = Widget {path: x.to_string()};
            match widget.fetch(system, ""){
                Result::Err(..) => {},
                Result::Ok(..) => {
                    println!("{} {} {}", "Debug".green().bold(), "sprite created for:".black().italic(), x.yellow().bold());
                    commands.spawn ((
                        widget,
                        DebugImage (),
                        SpriteBundle {
                            texture: asset_server.load("debug.png"),
                            transform: Transform { ..default() },
                            sprite: Sprite {
                                anchor: Anchor::TopLeft,
                                ..default()
                            },
                            ..default()
                        }
                    ));
                },
            }
        }
    }
}
pub fn lunex_update_debug( systems: Query<&Hierarchy>, mut query: Query<(&mut Widget, &mut Transform)>) {
    let system = systems.get_single().unwrap();
    for (widget, mut transform) in &mut query {
        match widget.fetch(&system, "") {
            Result::Err(..) => {
                transform.translation.x = -10000.0;
                transform.translation.y = -10000.0;
            },
            Result::Ok(branch) => {
                transform.translation.z = branch.get_depth() + 400.0;
            }
        };
    }
}
pub fn lunex_camera_move_debug (mut query: Query<(&Camera, &mut Transform)>, keyboard_input: Res<Input<KeyCode>>) {
    for (_, mut transform) in &mut query {
        transform.translation.x += (keyboard_input.pressed(KeyCode::A) as i32) as f32 * -10.0;
        transform.translation.x += (keyboard_input.pressed(KeyCode::D) as i32) as f32 * 10.0;
        transform.translation.y += (keyboard_input.pressed(KeyCode::S) as i32) as f32 * -10.0;
        transform.translation.y += (keyboard_input.pressed(KeyCode::W) as i32) as f32 * 10.0;
    }
}

pub struct LunexDebugPlugin;
impl Plugin for LunexDebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, lunex_setup_debug)
            .add_systems(Update, lunex_update_debug)
            .add_systems(Update, lunex_camera_move_debug.before(cursor_update));
    }
}