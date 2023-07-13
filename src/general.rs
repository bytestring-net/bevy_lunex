#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;
use bevy::sprite::Anchor;
use colored::Colorize;
use crate::prelude::*;

//===========================================================================

#[derive(Component)]
struct DebugImage ();
pub fn lunex_setup_debug (mut commands: Commands, asset_server: Res<AssetServer>, systems: Query<&Hierarchy>) {
    for system in systems.iter() {
        for x in system.collect_paths(){
            let widget = Widget::new(&x);
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



//# GENERAL USE


/// # Description
/// This function is used for translating Vec2 from Bevy 2D coordinate system to Lunex coordinate system.
/// It is necessary to go through this step if you want entities to be able to interact with Lunex.
/// 
/// Example of this is the cursor entity which has an unmodified [`Transform`] component.
/// Due to the nature of Bevy 2D, the y+ direction is upwards instead of downwards.
/// 
/// * This function will invert the Y component.
/// * In addition it will offset the values because Lunex always starts at 0.
///
/// # Examples
/// ```
/// let system = Hierarchy::new();
/// let offset = Vec2::new( -window.size.x / 2.0, window.size.y / 2.0 );
/// let cursor_position = Vec2::new(40.0, 20.0);    //Extracted from transform.translation.x, y
///
/// //Returns bool
/// widget.is_within(&system, "", &vec_convert(cursor_position, &offset)).unwrap();
/// ```
///
pub fn vec_convert (vec2: &Vec2, offset: &Vec2) -> Vec2 {
    Vec2::new(vec2.x - offset.x, offset.y - vec2.y)
}

pub fn tween (value_1: f32, value_2: f32, slide: f32) -> f32 {
    let diff = value_2 - value_1;
    value_1 + diff * slide
}


//# CRATE USE ONLY
pub fn is_absolute (str: &str) -> bool {
    match str.chars().nth(0) {
        Some (value) => {
            value == '#'
        },
        None => false,
    }
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