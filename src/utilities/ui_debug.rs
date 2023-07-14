use bevy::{prelude::*, sprite::Anchor};
use colored::Colorize;
use crate::prelude::*;

// ===========================================================
// === DEBUGGING FUNCTIONALITY ===

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