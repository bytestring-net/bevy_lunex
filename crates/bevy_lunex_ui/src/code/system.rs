use bevy::prelude::*;
use bevy_lunex_core::{UiTree, Widget, UiT, UiD};
use bevy_lunex_utility::Element;

use crate::{cursor_update, InvertY};

#[derive(Component, Default)]
pub struct Size {
    width: f32,
    height: f32,
}

// ===========================================================
// === SYSTEMS ===

/// # Tree Pull Window
/// A system that pulls [`Window`] dimensions into UiTree's [`Size`] and [`Transform`] component.
/// 
/// This is repeated every frame.
pub fn tree_pull_window<T:Component + Default>(mut query: Query<(&mut Size, &mut Transform, &Window), With<UiTree<T>>>) {
    for (mut size, mut transform, window) in &mut query {
        size.width = window.resolution.width();
        size.height = window.resolution.height();
        transform.translation.x = -size.width/2.0;
        transform.translation.y = -size.height/2.0;
    }
}

// FUTURE ADD TREE_PULL_CAMERA 

/// # Tree Compute
/// A system that calls `.compute()` with data from UiTree's [`Size`] and [`Transform`] component.
/// 
/// This is repeated every frame.
pub fn tree_compute<T:Component + Default>(mut query: Query<(&mut UiTree<T>, &Size, &Transform)>) {
    for (mut tree, size, transform) in &mut query {
        tree.compute(transform.translation.truncate(), size.width, size.height);
    }
}

/// # Element Update
/// A system that re-positions and re-scales every [`Element`] to match the calculated layout.
/// 
/// Requires that entity has [`Element`] + [`Widget`] + [`Transform`] + [`Visibility`] components.
/// * [`Element`] contains the data how to position the entity relative to the widget.
/// * [`Widget`] constains the path link.
/// * [`Transform`] fields will be overwritten by this system.
/// * [`Visibility`] enum will be changed by this system.
/// 
/// [`Widget`] needs to have valid path, otherwise the entity will be **`despawned`**
pub fn element_update<T:Component + Default>(mut commands: Commands, systems: Query<(&UiTree<T>, &Transform)>, mut query: Query<(&Widget, &Element, &mut Transform, &mut Visibility, Entity), Without<UiTree<T>>>) {
    for (tree, tree_transform) in systems.iter() {
        for (widget, element, mut transform, mut visibility, entity) in &mut query {
            match widget.fetch(&tree) {
                Err(_) => {
                    commands.entity(entity).despawn();
                },
                Ok(branch) => {
                    if !branch.is_visible() {
                        *visibility = Visibility::Hidden;
                    } else {
                        *visibility = Visibility::Inherited;
    
                        transform.translation.z = branch.get_depth() + element.depth + tree_transform.translation.z;
    
                        let pos = branch.get_container().get_position().clone();
                        let vec = pos.get_pos(element.relative).invert_y();
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
            }
        }
    }
}


// ===========================================================
// === PLUGIN ===

/// # Lunex Ui Plugin 2D
/// A main plugin adding Lunex UI functionality for a 2D plane.
/// ## Systems
/// * [`tree_pull_window`]
/// * [`tree_compute`]
/// * [`element_update`]
/// * [`cursor_update`]
pub struct LunexUiPlugin2D<T:Component + Default>(pub std::marker::PhantomData<T>);
impl <T: Component + Default> Plugin for LunexUiPlugin2D<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tree_pull_window::<T>).before(tree_compute::<T>))
           .add_systems(Update, (tree_compute::<T>, element_update::<T>).chain())
           .add_systems(Update, cursor_update);
    }
}