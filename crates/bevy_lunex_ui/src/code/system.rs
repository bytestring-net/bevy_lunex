use bevy::prelude::*;
use bevy_lunex_core::{UiTree, Widget, UiT, UiD};
use bevy_lunex_utility::Element;

use crate::cursor_update;

#[derive(Component, Default)]
pub struct Rectangle {
    width: f32,
    height: f32,
}

// ===========================================================
// === SYSTEMS ===

/// # Tree Pull Window
/// A system that pulls [`Window`] dimensions into UiTree's [`Rectangle`] component.
/// 
/// This is repeated every frame.
pub fn tree_pull_window(mut query: Query<(&UiTree, &mut Rectangle, &mut Transform, &Window)>) {
    for (_, mut rectangle, mut transform, window) in &mut query {
        rectangle.width = window.resolution.width();
        rectangle.height = window.resolution.height();
        transform.translation.x = -rectangle.width/2.0;
        transform.translation.y = -rectangle.height/2.0;
    }
}

// FUTURE ADD TREE_PULL_CAMERA 

/// # Tree Compute
/// A system that calls `.compute()` with data from UiTree's [`Rectangle`] component.
/// 
/// This is repeated every frame.
pub fn tree_compute(mut query: Query<(&mut UiTree, &Rectangle, &Transform)>) {
    for (mut tree, rectangle, transform) in &mut query {
        tree.compute(transform.translation.truncate(), rectangle.width, rectangle.height);
    }
}

/// # Element Update
/// A system that re-positions and re-scales every [`Element`] to match the calculated layout.
/// 
/// Requires that entity has [`Element`] + [`Widget`] + [`Transform`] components.
/// * [`Element`] contains the data how to position the entity relative to the widget.
/// * [`Widget`] constains the path link.
/// * [`Transform`] fields will be overwritten by this system.
/// 
/// [`Widget`] needs to have valid path, otherwise the entity will be **`despawned`**.
/// When [`Widget`] visibility is set to `false`, X and Y transform will be set to `-10 000`.
pub fn element_update(systems: Query<(&UiTree, &Transform)>, mut query: Query<(&Widget, &Element, &mut Transform, &mut Visibility, Without<UiTree>)>) {
    for (tree, tree_transform) in systems.iter() {
        for (widget, element, mut transform, mut visibility, _) in &mut query {
            match widget.fetch(&tree) {
                Err(_) => {
                    // DESPAWN
                    *visibility = Visibility::Hidden;
                },
                Ok(branch) => {
                    if !branch.is_visible() {
                        *visibility = Visibility::Hidden;
                    } else {
                        *visibility = Visibility::Inherited;
    
                        transform.translation.z = branch.get_depth() + element.depth + tree_transform.translation.z;
    
                        //let pos = widget.fetch(&tree).unwrap().get_container().get_position().clone().invert_y();
                        let pos = branch.get_container().get_position().clone().invert_y();
                        let vec = pos.get_pos_y_inverted(element.relative);
                        transform.translation.x = vec.x;
                        transform.translation.y = vec.y;
    
                        match element.width {
                            Some (w) => {
                                match element.height {
                                    Some (h) => {
                                        transform.scale.x = (pos.width/element.boundary.x)*(w/100.0) * element.scale/100.0;
                                        transform.scale.y = (pos.height/element.boundary.y)*(h/100.0) * element.scale/100.0;
                                    },
                                    None => {
                                        let scale = (pos.width/element.boundary.x)*(w/100.0) * element.scale/100.0;
                                        transform.scale.x = scale;
                                        transform.scale.y = scale;
                                    },
                                }
                            },
                            None => {
                                match element.height {
                                    Some (h) => {
                                        let scale = (pos.height/element.boundary.y)*(h/100.0) * element.scale/100.0;
                                        transform.scale.x = scale;
                                        transform.scale.y = scale;
                                    },
                                    None => {
                                        let scale = f32::min(pos.width/element.boundary.x, pos.height/element.boundary.y) * element.scale/100.0;
                                        transform.scale.x = scale;
                                        transform.scale.y = scale;
                                    },
                                }
                            },
                        }
                    }
                }
            };
        }
    }
}


// ===========================================================
// === PLUGIN ===

/// # Lunex Ui Plugin
/// A main plugin adding Lunex UI functionality for 2D plane.
/// # Systems
/// * [`tree_update`]
/// * [`element_update`]
pub struct LunexUiPlugin;
impl Plugin for LunexUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tree_pull_window).before(tree_compute))
           .add_systems(Update, (tree_compute, element_update).chain())
           .add_systems(Update, cursor_update);
    }
}