use bevy::prelude::*;

use bevy_lunex_core::UiTree;

use crate::cursor_update;
use crate::element_update;


// ===========================================================
// === DEBUGGING FUNCTIONALITY ===


/// # Lunex setup debug
pub fn lunex_drawlines_debug(
    mut query: Query<&UiTree>,
    mut gizmos: Gizmos,
) {
    for tree in &mut query {
        let vector = pathio::PathioHierarchy::crawl(tree);
        for bb in vector {
            gizmos.rect_2d(
                bb.file.as_ref().unwrap().point_1(),
                0.0,
                bb.file.as_ref().unwrap().size(),
                Color::ORANGE_RED,
            );
        }
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
        app.add_systems(Update, lunex_drawlines_debug.after(element_update))
            .add_systems(Update, lunex_camera_move_debug.before(cursor_update));
    }
}
