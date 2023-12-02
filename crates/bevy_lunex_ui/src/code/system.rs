use bevy::prelude::*;
use bevy_lunex_core::{UiTree, Widget, UiT, UiD, Size, Modifier};
use bevy_lunex_utility::Element;

use crate::{cursor_update, cursor_update_texture, cursor_preupdate, InvertY};


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
/// A system that re-positions and re-scales every [`Element`] for widgets in changed [`UiTree`]s to match the calculated
/// layout.
/// 
/// Requires that entity has [`Element`] + [`Widget`] + [`Transform`] + [`Visibility`] components.
/// * [`Element`] contains the data how to position the entity relative to the widget.
/// * [`Widget`] constains the path link.
/// * [`Transform`] fields will be overwritten by this system.
/// * [`Visibility`] enum will be changed by this system.
/// 
/// [`Widget`] needs to have valid path, otherwise the entity will be **`despawned`**.
pub fn update_changed_trees<T:Component + Default>(
    mut commands: Commands,
    changed_trees: Query<(&UiTree<T>, &Transform), Or<(Changed<UiTree<T>>, Changed<Transform>)>>,
    mut elements: Query<(Entity, &Widget, &Element, &mut Transform, &mut Visibility), Without<UiTree<T>>>,
) -> bool {
    for (tree, tree_transform) in changed_trees.iter() {
        for (entity, widget, element, mut transform, mut visibility) in &mut elements {
            element_update_impl(
                &mut commands,
                tree,
                &tree_transform.translation,
                entity,
                widget,
                element,
                &mut transform,
                &mut visibility
            );
        }
    }

    !changed_trees.is_empty()
}

/// # Element Update
/// See [`update_changed_trees()`]. Only updates widget elements that have changed.
pub fn update_changed_elements<T:Component + Default>(
    In(trees_changed): In<bool>,
    mut commands: Commands,
    trees: Query<(&UiTree<T>, &Transform)>,
    mut changed_elements: Query<
        (Entity, &Widget, &Element, &mut Transform, &mut Visibility),
        (Without<UiTree<T>>, Or<(Changed<Widget>, Changed<Element>)>)
    >
) {
    if trees_changed {
        return;
    }

    for (tree, tree_transform) in trees.iter() {
        for (entity, widget, element, mut transform, mut visibility) in &mut changed_elements {
            element_update_impl(
                &mut commands,
                tree,
                &tree_transform.translation,
                entity,
                widget,
                element,
                &mut transform,
                &mut visibility
            );
        }
    }
}

fn element_update_impl<T:Component + Default>(
    commands: &mut Commands,
    tree: &UiTree<T>,
    tree_translation: &Vec3,
    entity: Entity,
    widget: &Widget,
    element: &Element,
    transform: &mut Transform,
    visibility: &mut Visibility
) {
    match widget.fetch(&tree) {
        Err(_) => {
            commands.entity(entity).despawn_recursive();
        },
        Ok(branch) => {
            if !branch.is_visible() {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Inherited;

                let container = branch.get_container();
                match container.get_render_depth() {
                    Modifier::Add(v) => transform.translation.z = v + branch.get_depth() * bevy_lunex_core::LEVEL_RENDER_DEPTH_DIFFERENCE + element.depth + tree_translation.z,
                    Modifier::Set(v) => transform.translation.z = v + element.depth + tree_translation.z,
                }

                let pos = container.get_position().clone();
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

fn widget_garbage_collection<T:Component + Default>(mut commands: Commands, trees: Query<&UiTree<T>>, widgets: Query<(Entity, &Widget)>) {
    for tree in &trees {
        for (entity, widget) in &widgets {
            if let Err(_) = widget.fetch(tree) {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

// ===========================================================
// === PLUGIN ===

/// # Lunex Ui System Set 2D
/// The system set where Lunex internal systems run.
#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub struct LunexUiSystemSet2D;

/// # Lunex Ui Plugin 2D
/// A plugin holding all plugins required by Bevy-Lunex to work in 2D plane.
/// 
/// Implements logic for [`UiTree`]<`T`> for the generic `T`. If you use more generics for UiTree
/// add the plugins separetly, [`LunexUiPlugin2DShared`] once and [`LunexUiPlugin2DGeneric`] for every generic.
/// ## Plugins
/// * [`LunexUiPlugin2DShared`]
/// * [`LunexUiPlugin2DGeneric`] for `T`
#[derive(Debug, Default, Clone)]
pub struct LunexUiPlugin2D<T:Component + Default>(pub std::marker::PhantomData<T>);
impl <T:Component + Default>LunexUiPlugin2D<T> {
    pub fn new() -> Self {
        LunexUiPlugin2D::<T>(std::marker::PhantomData)
    }
}
impl <T: Component + Default> Plugin for LunexUiPlugin2D<T> {
    fn build(&self, app: &mut App) {
        app.add_plugins(LunexUiPlugin2DShared)
           .add_plugins(LunexUiPlugin2DGeneric::<T>::new());
    }
}


/// # Lunex Ui Plugin 2D Shared
/// A plugin holding all **SHARED** systems required by Bevy-Lunex to work in 2D plane.
/// Contains logic which is undesired for 3D applications.
/// 
/// Should be added only once per app. Has no generic.
/// ## Systems
/// * [`cursor_update`]
#[derive(Debug, Default, Clone)]
pub struct LunexUiPlugin2DShared;
impl LunexUiPlugin2DShared {
    pub fn new() -> Self {
        LunexUiPlugin2DShared
    }
}
impl Plugin for LunexUiPlugin2DShared {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, cursor_preupdate.in_set(LunexUiSystemSet2D))
           .add_systems(Update, cursor_update.in_set(LunexUiSystemSet2D))
           .add_systems(PostUpdate, cursor_update_texture.in_set(LunexUiSystemSet2D));
    }
}


/// # Lunex Ui Plugin 2D Generic 
/// A plugin holding all **GENERIC** systems required by Bevy-Lunex to work in 2D plane.
/// Contains logic which is undesired for 3D applications.
/// 
/// 
/// Add this plugin for every `T` that you use.
/// ## Systems
/// * [`tree_pull_window`]
/// * [`tree_compute`]
/// * [`element_update`]
#[derive(Debug, Default, Clone)]
pub struct LunexUiPlugin2DGeneric<T:Component + Default>(pub std::marker::PhantomData<T>);
impl <T:Component + Default>LunexUiPlugin2DGeneric<T> {
    pub fn new() -> Self {
        LunexUiPlugin2DGeneric::<T>(std::marker::PhantomData)
    }
}
impl <T: Component + Default> Plugin for LunexUiPlugin2DGeneric<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            (
                tree_pull_window::<T>,
                tree_compute::<T>,
                update_changed_trees::<T>.pipe(update_changed_elements::<T>)
            ).chain()
                .in_set(LunexUiSystemSet2D)
                .before(cursor_update))
           .add_systems(PostUpdate, widget_garbage_collection::<T>);
    }
}