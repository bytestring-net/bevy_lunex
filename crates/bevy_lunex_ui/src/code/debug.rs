use bevy::prelude::*;
use colored::Colorize;

use bevy_lunex_core::{UiTree, Widget};
use bevy_lunex_utility::{ImageElementBundle, ImageParams};

use crate::cursor_update;
use crate::element_update;


// ===========================================================
// === DEBUGGING FUNCTIONALITY ===

/// ### Debug Image
/// A marker for ImageBundles spawned by debug functions, ***NOT INTENDED*** to be used by user!
#[derive(Component)]
pub struct DebugImage;

/// ### Lunex setup debug
/// A system that will create debug sprites for all valid widgets
pub fn lunex_setup_debug(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    systems: Query<&UiTree>,
) {
    for system in systems.iter() {
        for x in system.collect_paths() {
            let widget = Widget::new(&x);
            match widget.fetch(system) {
                Err(_) => {}
                Ok(..) => {
                    println!(
                        "{} {} {}",
                        "Debug".green().bold(),
                        "sprite created for:".black().italic(),
                        x.yellow().bold()
                    );
                    commands.spawn((
                        ImageElementBundle::new(widget, ImageParams::default().with_width(Some(100.0)).with_height(Some(100.0)), asset_server.load("debug.png"), Vec2::new(300.0,200.0)),
                        DebugImage
                    ));
                }
            }
        }
    }
}

/// ### Lunex setup debug
/// A system that will update debug sprites to have + 400 Z
pub fn lunex_update_debug(
    systems: Query<&UiTree>,
    mut query: Query<(&mut Widget, &mut Transform, &DebugImage)>,
) {
    let system = systems.get_single().unwrap();
    for (widget, mut transform, _) in &mut query {
        match widget.fetch(&system) {
            Err(_) => {
                transform.translation.x = -10000.0;
                transform.translation.y = -10000.0;
            }
            Ok(branch) => {
                transform.translation.z = branch.get_depth() + 400.0;
            }
        };
    }
}

/// ### Lunex setup debug
/// A system that will allow the camera to move out of view by WASD.
pub fn lunex_camera_move_debug(
    mut query: Query<(&Camera, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (_, mut transform) in &mut query {
        transform.translation.x += (keyboard_input.pressed(KeyCode::A) as i32) as f32 * -10.0;
        transform.translation.x += (keyboard_input.pressed(KeyCode::D) as i32) as f32 * 10.0;
        transform.translation.y += (keyboard_input.pressed(KeyCode::S) as i32) as f32 * -10.0;
        transform.translation.y += (keyboard_input.pressed(KeyCode::W) as i32) as f32 * 10.0;
    }
}

// ===========================================================
// === PLUGIN ===

/// ### Lunex Ui Debug Plugin
/// A plugin holding all systems used for debugging Bevy-Lunex.
/// ### Systems
/// * `lunex_setup_debug` = queries and initiates debug sprites for all valid widgets.
/// * `lunex_update_debug` = updates the debug sprites Z coordinate to be Z + 400.
/// * `lunex_camera_move_debug` = adds WASD movement to the camera so you can see widgets out of view.
pub struct LunexUiDebugPlugin;
impl Plugin for LunexUiDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, lunex_setup_debug)
            .add_systems(Update, lunex_update_debug.after(element_update))
            .add_systems(Update, lunex_camera_move_debug.before(cursor_update));
    }
}
