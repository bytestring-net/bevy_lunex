use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_vector_shapes::prelude::*;


// ===========================================================
// === PLUGIN ===

/// # Lunex Ui Prefab Plugin
/// A plugin adding all logic that prefabs from Bevy-Lunex require.
/// Without this plugin, none of the prefabs will work.
/// 
/// * Includes the `Shape2DPlugin` from `bevy_vector_shapes`
/// * Includes the `LunexUiPlugin` from `bevy_lunex_ui`
pub struct LunexUiPrefabPlugin;
impl Plugin for LunexUiPrefabPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(LunexUiPlugin)

        .add_systems(Update, (
            crate::code::boxes::vector_rectangle_update,
        ).after(element_update));
    }
}