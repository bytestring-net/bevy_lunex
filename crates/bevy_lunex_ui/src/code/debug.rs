use bevy::prelude::*;
use bevy_lunex_core::UiTree;

use crate::cursor_update;
use crate::element_update;
use crate::InvertY;

// ===========================================================
// === DEBUGGING FUNCTIONALITY ===

/// # Lunex Draw Lines Debug 2D
/// A system that uses 2D gizmos to draw `LIME_GREEN` rectangles over location of every widget.
pub fn lunex_draw_lines_debug_2d(
    mut query: Query<&UiTree>,
    mut gizmos: Gizmos,
) {
    for tree in &mut query {
        let vector = pathio::PathioHierarchy::crawl(tree);
        for bb in vector {
            let container = bb.file.as_ref().unwrap();
            gizmos.rect_2d(
                (container.point_1() + container.size() / 2.0).invert_y(),
                0.0,
                container.size(),
                Color::LIME_GREEN,
            );
        }
    }
}

/// # Lunex Camera Move Debug 2D
/// A system that will allow the camera to move out of view by WASD on 2D plane.
pub fn lunex_camera_move_debug_2d(
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

/// # Lunex Ui Debug Plugin 2D
/// A plugin holding all systems used for debugging Bevy-Lunex in 2D plane.
/// Contains logic which is undesired for 3D applications.
/// ## Systems
/// * [`lunex_draw_lines_debug_2d`]
/// * [`lunex_camera_move_debug_2d`]
pub struct LunexUiDebugPlugin2D;
impl Plugin for LunexUiDebugPlugin2D {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lunex_draw_lines_debug_2d.after(element_update))
            .add_systems(Update, lunex_camera_move_debug_2d.before(cursor_update));
    }
}
