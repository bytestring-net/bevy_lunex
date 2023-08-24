use bevy::prelude::*;
use bevy_lunex_core::{UiTree, Widget};
use bevy_lunex_utility::Element;

/// # Tree Update
/// A system that transforms every [`UiTree`] into an immidiete mode UI framework.
/// 
/// It does that by pulling width and height from the first [`Window`] it queries (multiple windows not supported yet) and feeding it to the [`UiTree`].
/// Then it calls `.update()`.
/// 
/// This is repeated every frame.
pub fn tree_update(mut query: Query<&mut UiTree>, windows: Query<&Window>) {
    for window in windows.iter() {
        for mut system in &mut query {
            system.width = window.resolution.width();
            system.height = window.resolution.height();
            system.offset.x = -system.width/2.0;
            system.offset.y = system.height/2.0;
            system.update();
        }
        break;  //Currently support only 1 window.
    }
}

/// # Element Update
/// A system that repositions every [`Element`] to match the calculated layout.
pub fn element_update(systems: Query<&UiTree>, mut query: Query<(&Widget, &Element, &mut Transform)>) {
    for system in systems.iter() {
        for (widget, element, mut transform) in &mut query {
            match widget.fetch(&system) {
                Err(..) => {
                    transform.translation.x = -10000.0;
                    transform.translation.y = -10000.0;
                },
                Ok(branch) => {
                    if !branch.is_visible() {
                        transform.translation.x = -10000.0;
                        transform.translation.y = -10000.0;
                    } else {
    
                        transform.translation.z = branch.get_depth() + element.depth;
    
                        let pos = widget.fetch(&system).unwrap().container_get().position_get().invert_y();
                        let vec = pos.get_pos_y_inverted(element.relative);
                        transform.translation.x = vec.x + system.offset.x;
                        transform.translation.y = vec.y + system.offset.y;
    
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


/// # Lunex Ui Plugin
/// A main plugin adding Lunex UI functionality for 2D plane.
/// # Systems
/// * [`tree_update`]
/// * [`element_update`]
pub struct LunexUiPlugin;
impl Plugin for LunexUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tree_update, element_update));
    }
}