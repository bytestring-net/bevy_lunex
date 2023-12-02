use bevy::prelude::*;
use bevy_lunex_core::UiTree;

use crate::cursor_update;
use crate::LunexUiSystemSet2D;
use crate::InvertY;

// ===========================================================
// === DEBUGGING FUNCTIONALITY ===

/// # Lunex Draw Lines Debug 2D
/// A system that uses 2D gizmos to draw `LIME_GREEN` rectangles over location of every widget.
pub fn lunex_draw_lines_debug_2d<T:Component + Default>(
    mut query: Query<&UiTree<T>>,
    mut gizmos: Gizmos,
) {
    for tree in &mut query {
        let vector = pathio::PathioHierarchy::crawl(tree);
        for bb in vector {
            let container = bb.file.as_ref().unwrap().container();
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

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub struct LunexUiDebugSystemSet2D;

/// # Lunex Ui Debug Plugin 2D
/// A plugin holding all plugins required for debugging Bevy-Lunex in 2D plane.
/// 
/// Implements logic for [`UiTree`]<`T`> for the generic `T`. If you use more generics for UiTree
/// add the plugins separetly, [`LunexUiDebugPlugin2DShared`] once and [`LunexUiDebugPlugin2DGeneric`] for every generic.
/// ## Plugins
/// * [`LunexUiDebugPlugin2DShared`]
/// * [`LunexUiDebugPlugin2DGeneric`] for `T`
#[derive(Debug, Default, Clone)]
pub struct LunexUiDebugPlugin2D<T:Component + Default>(pub std::marker::PhantomData<T>);
impl <T:Component + Default>LunexUiDebugPlugin2D<T> {
    pub fn new() -> Self {
        LunexUiDebugPlugin2D::<T>(std::marker::PhantomData)
    }
}
impl <T: Component + Default> Plugin for LunexUiDebugPlugin2D<T> {
    fn build(&self, app: &mut App) {
        app.add_plugins(LunexUiDebugPlugin2DShared)
           .add_plugins(LunexUiDebugPlugin2DGeneric::<T>::new());
    }
}


/// # Lunex Ui Debug Plugin 2D Shared
/// A plugin holding all **SHARED** systems used for debugging Bevy-Lunex in 2D plane.
/// Contains logic which is undesired for 3D applications.
/// 
/// Should be added only once per app. Has no generic.
/// ## Systems
/// * [`lunex_camera_move_debug_2d`]
#[derive(Debug, Default, Clone)]
pub struct LunexUiDebugPlugin2DShared;
impl LunexUiDebugPlugin2DShared {
    pub fn new() -> Self {
        LunexUiDebugPlugin2DShared
    }
}
impl Plugin for LunexUiDebugPlugin2DShared {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lunex_camera_move_debug_2d.before(cursor_update));
    }
}


/// # Lunex Ui Debug Plugin 2D Generic 
/// A plugin holding all **GENERIC** systems used for debugging Bevy-Lunex in 2D plane.
/// Contains logic which is undesired for 3D applications.
/// 
/// 
/// Add this plugin for every `T` that you use.
/// ## Systems
/// * [`lunex_draw_lines_debug_2d`]
#[derive(Debug, Default, Clone)]
pub struct LunexUiDebugPlugin2DGeneric<T:Component + Default>(pub std::marker::PhantomData<T>);
impl <T:Component + Default>LunexUiDebugPlugin2DGeneric<T> {
    pub fn new() -> Self {
        LunexUiDebugPlugin2DGeneric::<T>(std::marker::PhantomData)
    }
}
impl <T: Component + Default> Plugin for LunexUiDebugPlugin2DGeneric<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lunex_draw_lines_debug_2d::<T>.after(LunexUiSystemSet2D).in_set(LunexUiDebugSystemSet2D));
    }
}
