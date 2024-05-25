use std::marker::PhantomData;
use bevy::{math::Vec3A, prelude::*, render::primitives::Aabb, text::TextLayoutInfo, window::PrimaryWindow};
#[cfg(feature = "debug")]
use colored::Colorize;
use lunex_engine::*;

use crate::{Dimension, Element, MovableByCamera, UiContent, UiDepthBias, UiLink};


// #===================#
// #=== CORE SYSTEM ===#

/// This system computes [`UiTree`] with data from querried [`Dimension`] component if there is a change.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn compute_ui<T:Component, N:Default + Component>(
    mut query: Query<(&Dimension, &mut UiTree<T, N>, Option<&MovableByCamera>), (With<UiLink<T>>, Or<(Changed<UiTree<T, N>>, Changed<Dimension>)>)>,
    window: Query<&bevy::window::Window, With<PrimaryWindow>>,
) {
    let scale = if let Ok(window) = window.get_single() { window.resolution.scale_factor() } else { 1.0 };
    for (dimension, mut ui, is_camera_sourced) in &mut query {
        #[cfg(feature = "debug")]
        info!("{} {} - {}", "<>".red(), "UiTree".purple().bold(), "Recomputed".underline().bold());
        let scale = if is_camera_sourced.is_none() { 1.0 } else { scale };
        ui.compute(Rectangle2D::new().with_size(dimension.size / scale).into());
    }
}


// #===================#
// #=== DEBUG NODES ===#

/// This system draws the outlines of [`UiTree`] nodes as gizmos.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn debug_draw_gizmo<T:Component, N:Default + Component>(
    mut query: Query<(&UiTree<T, N>, &GlobalTransform)>,
    mut gizmos: Gizmos
) {
    for (tree, transform) in &mut query {
        let list = tree.crawl();
        for node in list {
            if let Some(container) = node.obtain_data() {

                let mut color = Color::LIME_GREEN;

                if let UiLayout::Solid(_) = container.layout { color = Color::YELLOW }

                let mut pos = container.rectangle.pos.invert_y() + transform.translation();
                pos.x += container.rectangle.size.x / 2.0;
                pos.y += container.rectangle.size.y / -2.0;

                gizmos.rect(
                    pos,
                    Quat::from_rotation_y(0.0),
                    container.rectangle.size,
                    color,
                );
            }
        }
    }
}

/// This system prints [`UiTree`] if there is a change.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn debug_print_tree<T:Component, N:Default + Component>(
    uis: Query<&UiTree<T, N>, Changed<UiTree<T, N>>>
) {
    for ui in &uis {
        info!("{}\n{}\n", "Change detected...", ui.tree("show-hidden"));
    }
}


// #=========================#
// #=== PIPING FOR UITREE ===#

/// This system takes [`Camera`] data and overwrites querried [`Dimension`] data.
/// It is mainly used to pipe [`Camera`] data into [`UiTree`] for root node computation.
/// ## 📦 Types
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// ## ⚠️ Warning
/// * Developer should ensure that source query returns only one camera.
///   Otherwise, it will lead to value overwriting. Just make sure only one camera
///   is marked with `(T)` component at the same time.
pub fn fetch_dimension_from_camera<T:Component, N:Default + Component>(
    source: Query<&Camera, (With<T>, Changed<Camera>)>,
    mut destination: Query<&mut Dimension, (With<UiTree<T, N>>, With<MovableByCamera>)>
) {
    // Undesired behaviour if source.len() > 1
    for cam in &source {
        for mut dimension in &mut destination {
            // Extract camera size
            if let Some(size) = cam.physical_viewport_size() {
                #[cfg(feature = "debug")]
                info!("{} {} - Fetched Dimension data from Camera", "->".blue(), "UiTree".purple().bold());
                dimension.size = Vec2::from((size.x as f32, size.y as f32));
            }
        }
    }
}

/// This system takes [`Camera`] data and overwrites querried [`Transform`] + [`MovableByCamera`].
/// It is mainly used to pipe [`Camera`] data into [`UiTree`] for positioning.
/// ## 📦 Types
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// ## ⚠️ Warning
/// * Developer should ensure that source query returns only one camera.
///   Otherwise, it will lead to value overwriting. Just make sure only one camera
///   is marked with `(T)` component at the same time.
pub fn fetch_transform_from_camera<T:Component, N:Default + Component>(
    source: Query<&Camera, (With<T>, Changed<Camera>)>,
    mut destination: Query<&mut Transform, (With<UiTree<T, N>>, With<MovableByCamera>)>,
    window: Query<&bevy::window::Window, With<PrimaryWindow>>,
) {
    // Undesired behaviour if source.len() > 1
    for cam in &source {
        let scale = if let Ok(window) = window.get_single() { window.resolution.scale_factor() } else { 1.0 };
        for mut transform in &mut destination {
            // Extract camera size
            if let Some(size) = cam.physical_viewport_size() {
                #[cfg(feature = "debug")]
                info!("{} {} - Fetched Transform data from Camera", "->".blue(), "UiTree".purple().bold());
                transform.translation = Vec3::from((size.x as f32 /-2.0 / scale, size.y as f32 / 2.0 / scale, 0.0));
            }
        }
    }
}

/// This system listens for added [`UiTree`] component and if it finds one, mutable accesses all [`Camera`] without changing them.
/// This way UiTrees that are spawned independently get the correct size piped into them.
/// ## 📦 Types
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// ## ⚠️ Warning
/// * Developer should ensure that source query returns only one camera.
///   Otherwise, it will lead to value overwriting. Just make sure only one camera
///   is marked with `(T)` component at the same time.
pub fn touch_camera_if_uitree_added<T:Component, N:Default + Component>(
    query: Query<Entity, (Added<UiTree<T, N>>, With<MovableByCamera>)>,
    mut camera: Query<&mut Camera, With<T>>,
){
    if !query.is_empty() {
        #[cfg(feature = "debug")]
        info!("{} {} - Touched all cameras", "<>".red(), "Camera".purple().bold());
        for mut camera in &mut camera {
            camera.as_mut();
        }
    }
}

// #========================#
// #=== PIPING FOR NODES ===#

/// This system takes [`UiLayout`] data and overwrites coresponding [`UiTree`] data. If node is not found, it creates new ones along the path.
/// ## 📦 Types
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn send_layout_to_node<T:Component, N:Default + Component>(
    mut uis: Query<(&mut UiTree<T, N>, &Children)>,
    query: Query<(&UiLink<T>, &UiLayout), (Changed<UiLayout>, Without<UiTree<T, N>>)>,
) {
    for (mut ui, children) in &mut uis {
        for child in children {
            // If child matches
            if let Ok((link, layout)) = query.get(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_or_create_ui_node_mut(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data_mut() {
                        #[cfg(feature = "debug")]
                        info!("{} {} - Received Layout data", "->".blue(), link.path.yellow().bold());
                        container.layout = *layout;
                    }
                }
            }
        }
    }
}

/// This system takes [`UiStack`] data and overwrites coresponding [`UiTree`] data.
/// ## 📦 Types
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn send_stack_to_node<T:Component, N:Default + Component>(
    mut uis: Query<(&mut UiTree<T, N>, &Children)>,
    query: Query<(&UiLink<T>, &UiStack), Changed<UiStack>>,
) {
    for (mut ui, children) in &mut uis {
        for child in children {
            // If child matches
            if let Ok((link, stack)) = query.get(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node_mut(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data_mut() {
                        #[cfg(feature = "debug")]
                        info!("{} {} - Received Stack data", "->".blue(), link.path.yellow().bold());
                        container.stack = *stack;
                    }
                }
            }
        }
    }
}

/// This system takes [`UiDepthBias`] data and overwrites coresponding [`UiTree`] data.
/// ## 📦 Types
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn send_depth_bias_to_node<T:Component, N:Default + Component>(
    mut uis: Query<(&mut UiTree<T, N>, &Children)>,
    query: Query<(&UiLink<T>, &UiDepthBias), Changed<UiDepthBias>>,
) {
    for (mut ui, children) in &mut uis {
        for child in children {
            // If child matches
            if let Ok((link, bias)) = query.get(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node_mut(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data_mut() {
                        #[cfg(feature = "debug")]
                        info!("{} {} - Received Depth bias data", "->".blue(), link.path.yellow().bold());
                        container.depth_bias = bias.0;
                    }
                }
            }
        }
    }
}

/// This system takes [`UiContent`] data and overwrites coresponding [`UiTree`] data.
/// ## 📦 Types
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn send_content_size_to_node<T:Component, N:Default + Component>(
    mut uis: Query<(&mut UiTree<T, N>, &Children)>,
    query: Query<(&UiLink<T>, &UiContent), Changed<UiContent>>,
) {
    for (mut ui, children) in &mut uis {
        for child in children {
            // If child matches
            if let Ok((link, content)) = query.get(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node_mut(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data_mut() {
                        #[cfg(feature = "debug")]
                        info!("{} {} - Received Content size data", "->".blue(), link.path.yellow().bold());
                        container.content_size = content.size;
                    }
                }
            }
        }
    }
}

/// This system fetches [`UiTree`] data and overwrites querried [`Transform`] data.
/// ## 📦 Types
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn fetch_transform_from_node<T:Component, N:Default + Component>(
    uis: Query<(&UiTree<T, N>, &Children), Changed<UiTree<T, N>>>,
    mut query: Query<(&UiLink<T>, &mut Transform), Without<Element>>,
) {
    for (ui, children) in &uis {
        for child in children {
            // If child matches
            if let Ok((link, mut transform)) = query.get_mut(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data() {
                        #[cfg(feature = "debug")]
                        info!("{} {} - Linked {} fetched Transform data from node", "<-".bright_green(), link.path.yellow().bold(), "ENTITY".blue());
                        transform.translation = container.rectangle.pos.invert_y();
                    }
                }
            }
        }
    }
}

/// This system fetches [`UiTree`] data and overwrites querried [`Dimension`] data.
/// ## 📦 Types
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn fetch_dimension_from_node<T:Component, N:Default + Component>(
    uis: Query<(&UiTree<T, N>, &Children), Changed<UiTree<T, N>>>,
    mut query: Query<(&UiLink<T>, &mut Dimension)>,
) {
    for (ui, children) in &uis {
        for child in children {
            // If child matches
            if let Ok((link, mut dimension)) = query.get_mut(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data() {
                        if dimension.as_ref().size != container.rectangle.size {
                            #[cfg(feature = "debug")]
                            info!("{} {} - Linked {} fetched Dimension data from node", "<-".bright_green(), link.path.yellow().bold(), "ENTITY".blue());
                            dimension.size = container.rectangle.size;
                        }
                    }
                }
            }
        }
    }
}

/// This system fetches [`UiTree`] data and overwrites querried [`Transform`] + [`Element`] data in specific way.
/// ## 📦 Types
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn element_fetch_transform_from_node<T:Component, N:Default + Component>(
    uis: Query<(&UiTree<T, N>, &Children), Changed<UiTree<T, N>>>,
    mut query: Query<(&UiLink<T>, &mut Transform), With<Element>>,
) {
    for (ui, children) in &uis {
        for child in children {
            // If child matches
            if let Ok((link, mut transform)) = query.get_mut(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data() {
                        #[cfg(feature = "debug")]
                        info!("{} {} - Linked {} fetched Transform data", "<-".bright_green(), link.path.yellow().bold(), "ELEMENT".red());
                        transform.translation = container.rectangle.pos.invert_y();
                        transform.translation.x += container.rectangle.size.x /  2.0;
                        transform.translation.y += container.rectangle.size.y / -2.0;
                    }
                }
            }
        }
    }
}

/// This system fetches [`Dimension`] data and overwrites querried [`Sprite`] data to fit.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn element_sprite_size_from_dimension<T: Component>(
    mut query: Query<(&mut Sprite, &Dimension), (With<UiLink<T>>, With<Element>, Changed<Dimension>)>,
) {
    for (mut sprite, dimension) in &mut query {
        #[cfg(feature = "debug")]
        info!("{} {} - Piped Dimension into sprite size", "--".yellow(), "ELEMENT".red());
        sprite.custom_size = Some(dimension.size)
    }
}

/// This system reconstructs the mesh on [`UiTree`] change.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn element_reconstruct_mesh<T: Component>(
    mut msh: ResMut<Assets<Mesh>>,
    mut query: Query<(&Dimension, &mut Handle<Mesh>, &mut Aabb), (With<UiLink<T>>, With<Element>, Changed<Dimension>)>,
) {
    for (dimension, mut mesh, mut aabb) in &mut query {

        #[cfg(feature = "debug")]
        info!("{} {} - Reconstructed mesh size", "--".yellow(), "ELEMENT".red());

        // Unload old mesh
        let _ = msh.remove(mesh.id());

        // Create new culling boundary
        *aabb = Aabb {
            center: Vec3A::ZERO,
            half_extents: Vec3A::new(dimension.size.x/2.0, dimension.size.y/2.0, 1.0),
        };

        // Create new mesh
        *mesh = msh.add(Rectangle {half_size: dimension.size / 2.0})
    }
}

/// This system takes [`TextLayoutInfo`] data and overwrites coresponding [`Layout`] solid data.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn element_text_size_to_layout<T: Component>(
    mut query: Query<(&mut UiLayout, &TextLayoutInfo), (With<UiLink<T>>, With<Element>, Changed<TextLayoutInfo>)>,
) {
    for (mut layout, text_info) in &mut query {
        #[cfg(feature = "debug")]
        info!("{} {} - Converted text size into Layout", "--".yellow(), "ELEMENT".red());
        match layout.as_mut() {
            UiLayout::Window(window) => {window.size = Rh(text_info.logical_size).into()},
            UiLayout::Solid(solid) => {solid.size = Ab(text_info.logical_size).into()},
            _ => {},
        }
    }
}

/// This system takes [`TextLayoutInfo`] data and overwrites coresponding [`UiContent`] data.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn element_text_size_to_content<T: Component>(
    mut query: Query<(&mut UiContent, &TextLayoutInfo), (With<UiLink<T>>, With<Element>, Changed<TextLayoutInfo>)>,
) {
    for (mut content, text_info) in &mut query {
        #[cfg(feature = "debug")]
        info!("{} {} - Piped text size into content", "--".yellow(), "ELEMENT".red());
        content.size = text_info.logical_size;
    }
}

/// This system takes [`TextLayoutInfo`] data and overwrites coresponding [`Transform`] scale data for text to fit inside [`Dimension`].
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn element_text_size_scale_fit_to_dimension<T: Component>(
    mut query: Query<(&mut Transform, &Dimension, &TextLayoutInfo), (With<UiLink<T>>, With<Element>, Changed<Dimension>)>,
) {
    for (mut transform, dimension, text_info) in &mut query {
        #[cfg(feature = "debug")]
        info!("{} {} - Scaled Transform for text size to fit into Dimension", "--".yellow(), "ELEMENT".red());
        let scale = dimension.size / text_info.logical_size;
        transform.scale.x = scale.x;
        transform.scale.y = scale.y;
    }
}


// #===============#
// #=== PLUGINS ===#

/// System set for [`UiPlugin`]
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UiSystems {
    /// Systems where we modify data pre-computation
    Modify,
    /// Systems that send component data to UiTree
    Send,
    /// The computation
    Compute,
    /// Systems that fetch component data from UiTree
    Fetch,
    /// Systems that process new data from UiTree
    Process,
}

/// Plugin implementing all ui logic for the specified generic types.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// 
/// ## 🛠️ Example
/// *1. Define the types used*
/// ```
///  #[derive(Component, Default)]
///  struct MyNodeData { value: i32 } // What data will each node contain
/// 
///  #[derive(Component)]
///  struct MyUiWidget; // Empty marker, used for selecting entities
/// ```
/// *2. Add the plugin to your app*
/// ```
///  App::new()
///      .add_plugins(DefaultPlugins)
///      .add_plugins(UiPlugin::<MyUiWidget, MyNodeData>::new())
///      .run();
/// ```
/// *3. Use the [`UiTree`] freely*
/// ```
///#  fn setup(mut commands: Commands) {
///   commands.spawn((
///      UiTree::<MyUiWidget, MyNodeData>::new("MyWidget")
///   ));
///#  }
/// ```
#[derive(Debug, Default, Clone)]
pub struct UiPlugin <T:Component, N:Default + Component = NoData>(PhantomData<T>, PhantomData<N>);
impl <T:Component, N:Default + Component> UiPlugin<T, N> {
    pub fn new() -> Self {
        UiPlugin::<T, N>(PhantomData, PhantomData)
    }
}
impl <T:Component, N:Default + Component> Plugin for UiPlugin<T, N> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                element_text_size_to_layout::<T>,
                element_text_size_to_content::<T>,
                touch_camera_if_uitree_added::<T, N>,
                fetch_dimension_from_camera::<T, N>.after(touch_camera_if_uitree_added::<T, N>),
                fetch_transform_from_camera::<T, N>.after(touch_camera_if_uitree_added::<T, N>),
            ).in_set(UiSystems::Modify).before(UiSystems::Send))

            .add_systems(Update, (
                send_content_size_to_node::<T, N>,
                send_stack_to_node::<T, N>,
                send_layout_to_node::<T, N>,
                send_depth_bias_to_node::<T, N>,
            ).in_set(UiSystems::Send).before(UiSystems::Compute))

            .add_systems(Update, (
                compute_ui::<T, N>.in_set(UiSystems::Compute)
            ).in_set(UiSystems::Compute))

            .add_systems(Update, (
                fetch_transform_from_node::<T, N>,
                fetch_dimension_from_node::<T, N>,
                element_fetch_transform_from_node::<T, N>,
            ).in_set(UiSystems::Fetch).after(UiSystems::Compute))

            .add_systems(Update, (
                element_sprite_size_from_dimension::<T>,
                element_text_size_scale_fit_to_dimension::<T>,
                element_reconstruct_mesh::<T>,
            ).in_set(UiSystems::Process).after(UiSystems::Fetch))
            ;
    }
}

/// Plugin implementing all debug ui logic for the specified generic types.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// 
/// ## 🛠️ Example
/// *1. Define the types used*
/// ```
///  #[derive(Component, Default)]
///  struct MyNodeData { value: i32 } // What data will each node contain
/// 
///  #[derive(Component)]
///  struct MyUiWidget; // Empty marker, used for selecting entities
/// ```
/// *2. Add the plugin to your app*
/// ```
///  App::new()
///      .add_plugins(DefaultPlugins)
///      .add_plugins(UiPlugin::<MyUiWidget, MyNodeData>::new())
///      .run();
/// ```
/// *3. Use the [`UiTree`] freely*
/// ```
///#  fn setup(mut commands: Commands) {
///   commands.spawn((
///      UiTree::<MyUiWidget, MyNodeData>::new("MyWidget")
///   ));
///#  }
/// ```
#[derive(Debug, Default, Clone)]
pub struct UiDebugPlugin <T:Component, N:Default + Component = NoData>(PhantomData<T>, PhantomData<N>);
impl <T:Component, N:Default + Component> UiDebugPlugin<T, N> {
    pub fn new() -> Self {
        UiDebugPlugin::<T, N>(PhantomData, PhantomData)
    }
}
impl <T:Component, N:Default + Component> Plugin for UiDebugPlugin<T, N> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, debug_draw_gizmo::<T, N>)
            .add_systems(Update, debug_print_tree::<T, N>.after(UiSystems::Compute));
    }
}
