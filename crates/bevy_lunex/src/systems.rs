use std::marker::PhantomData;
use bevy::{math::Vec3A, prelude::*, render::primitives::Aabb};
use lunex_engine::*;

use crate::{Dimension, Element, MovableByCamera, UiContent, UiLink, UiStack};


// #===================#
// #=== CORE SYSTEM ===#

/// This system computes [`UiTree`] with data from querried [`Dimension`] component if there is a change.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn compute_ui<M:Default + Component, N:Default + Component, T: Component>(
    mut query: Query<(&Dimension, &mut UiTree<M, N>), (With<T>, Or<(Changed<Dimension>, Changed<UiTree<M, N>>)>)>
) {
    for (dimension, mut ui) in &mut query {
        // Compute the Ui
        //println!("Ui DIM: {}", dimension.size);
        ui.compute(Rectangle2D::new().with_size(dimension.size).into());
    }
}


// #===================#
// #=== DEBUG NODES ===#

/// This system draws the outlines of [`UiTree`] nodes as gizmos.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn debug_draw_gizmo<M:Default + Component, N:Default + Component, T: Component>(mut query: Query<(&UiTree<M, N>, &Transform), With<T>>, mut gizmos: Gizmos) {
    for (tree, transform) in &mut query {
        let list = tree.crawl();
        for node in list {
            if let Some(container) = node.obtain_data() {

                let mut color = Color::LIME_GREEN;

                if let Layout::Solid(_) = container.layout { color = Color::YELLOW }

                let mut pos = container.rectangle.pos.invert_y() + transform.translation;
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
pub fn debug_print_tree<M:Default + Component, N:Default + Component, T: Component>(
    uis: Query<&UiTree<M, N>, (With<T>, Changed<UiTree<M, N>>)>
) {
    for ui in &uis {
        info!("{}\n{}\n", "UiTree has been changed...", ui.tree("show-hidden"));
    }
}


// #=========================#
// #=== PIPING FOR UITREE ===#

/// This system takes [`Camera`] data and overwrites querried [`Dimension`] data.
/// It is mainly used to pipe [`Camera`] data into [`UiTree`] for root node computation.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// ## ⚠️ Warning
/// * Developer should ensure that source query returns only one camera.
///   Otherwise, it will lead to value overwriting. Just make sure only one camera
///   is marked with `(T)` component at the same time.
pub fn fetch_dimension_from_camera<M:Default + Component, N:Default + Component, T: Component>(
    source: Query<&Camera, (With<T>, Changed<Camera>)>,
    mut destination: Query<&mut Dimension, (With<T>, With<UiTree<M, N>>)>
) {
    // Undesired behaviour if source.len() > 1
    for cam in &source {
        for mut dimension in &mut destination {
            // Extract camera size
            if let Some(size) = cam.physical_viewport_size() {
                dimension.size = Vec2::from((size.x as f32, size.y as f32));
            }
        }
    }
}

/// This system takes [`Camera`] data and overwrites querried [`Transform`] + [`MovableByCamera`].
/// It is mainly used to pipe [`Camera`] data into [`UiTree`] for positioning.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// ## ⚠️ Warning
/// * Developer should ensure that source query returns only one camera.
///   Otherwise, it will lead to value overwriting. Just make sure only one camera
///   is marked with `(T)` component at the same time.
pub fn fetch_transform_from_camera<T: Component>(
    source: Query<&Camera, (With<T>, Changed<Camera>)>,
    mut destination: Query<&mut Transform, (With<T>, With<MovableByCamera>)>
) {
    // Undesired behaviour if source.len() > 1
    for cam in &source {
        for mut transform in &mut destination {
            // Extract camera size
            if let Some(size) = cam.physical_viewport_size() {
                transform.translation = Vec3::from((size.x as f32 /-2.0, size.y as f32 /2.0, 0.0));
            }
        }
    }
}


// #========================#
// #=== PIPING FOR NODES ===#

/// This system takes [`Layout`] data and overwrites coresponding [`UiTree`] data. If node is not found, it creates new ones along the path.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn send_layout_to_node<M:Default + Component, N:Default + Component, T: Component>(
    mut uis: Query<(&mut UiTree<M, N>, &Children), With<T>>,
    query: Query<(&UiLink, &Layout), (With<T>, Changed<Layout>)>,
) {
    for (mut ui, children) in &mut uis {
        for child in children {
            // If child matches
            if let Ok((link, layout)) = query.get(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_or_create_ui_node_mut(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data_mut() {
                        container.layout = *layout;
                    }
                }
            }
        }
    }
}

/// This system takes [`UiStack`] data and overwrites coresponding [`UiTree`] data.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn send_stack_to_node<M:Default + Component, N:Default + Component, T: Component>(
    mut uis: Query<(&mut UiTree<M, N>, &Children), With<T>>,
    query: Query<(&UiLink, &UiStack), (With<T>, Changed<Layout>)>,
) {
    for (mut ui, children) in &mut uis {
        for child in children {
            // If child matches
            if let Ok((link, stack)) = query.get(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node_mut(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data_mut() {
                        container.stack = *stack;
                    }
                }
            }
        }
    }
}

/// This system takes [`UiContent`] data and overwrites coresponding [`UiTree`] data.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn send_content_size_to_node<M:Default + Component, N:Default + Component, T: Component>(
    mut uis: Query<(&mut UiTree<M, N>, &Children), With<T>>,
    query: Query<(&UiLink, &UiContent), (With<T>, Changed<Layout>)>,
) {
    for (mut ui, children) in &mut uis {
        for child in children {
            // If child matches
            if let Ok((link, content)) = query.get(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node_mut(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data_mut() {
                        container.content_size = content.size;
                    }
                }
            }
        }
    }
}

/// This system fetches [`UiTree`] data and overwrites querried [`Transform`] data.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn fetch_transform_from_node<M:Default + Component, N:Default + Component, T: Component>(
    uis: Query<(&UiTree<M, N>, &Children), (With<T>, Changed<UiTree<M, N>>)>,
    mut query: Query<(&UiLink, &mut Transform), (With<T>, Without<Element>)>,
) {
    for (ui, children) in &uis {
        for child in children {
            // If child matches
            if let Ok((link, mut transform)) = query.get_mut(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data() {
                        transform.translation = container.rectangle.pos.invert_y();
                    }
                }
            }
        }
    }
}

/// This system fetches [`UiTree`] data and overwrites querried [`Dimension`] data.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn fetch_dimension_from_node<M:Default + Component, N:Default + Component, T: Component>(
    uis: Query<(&UiTree<M, N>, &Children), (With<T>, Changed<UiTree<M, N>>)>,
    mut query: Query<(&UiLink, &mut Dimension), With<T>>,
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
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn element_fetch_transform_from_node<M:Default + Component, N:Default + Component, T: Component>(
    uis: Query<(&UiTree<M, N>, &Children), (With<T>, Changed<UiTree<M, N>>)>,
    mut query: Query<(&UiLink, &mut Transform), (With<T>, With<Element>)>,
) {
    for (ui, children) in &uis {
        for child in children {
            // If child matches
            if let Ok((link, mut transform)) = query.get_mut(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data() {
                        transform.translation = container.rectangle.pos.invert_y();
                        transform.translation.x += container.rectangle.size.x /  2.0;
                        transform.translation.y += container.rectangle.size.y / -2.0;
                    }
                }
            }
        }
    }
}

/// This system fetches [`Dimension`] & [`Image`] data and overwrites querried [`Transform`] scale data to fit.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn element_sprite_scale_to_dimension<T: Component>(
    mut query: Query<(&mut Transform, &Dimension, &Handle<Image>), (With<T>, With<Element>, With<Sprite>, Or<(Changed<Dimension>, Changed<Handle<Image>>)>)>,
    assets: Res<Assets<Image>>,
) {
    for (mut transform, dimension, image) in &mut query {
        if let Some(img) = assets.get(image) {
            transform.scale.x = dimension.size.x / img.texture_descriptor.size.width as f32;
            transform.scale.y = dimension.size.y / img.texture_descriptor.size.height as f32;
        }
    }
}

/// This system reconstructs the mesh on [`UiTree`] change.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn element_reconstruct_mesh<T: Component>(
    mut msh: ResMut<Assets<Mesh>>,
    mut query: Query<(&Dimension, &mut Handle<Mesh>, &mut Aabb), (With<T>, With<Element>, Changed<Dimension>)>,
) {
    for (dimension, mut mesh, mut aabb) in &mut query {

        // Unload old mesh
        let _ = msh.remove(mesh.id());

        // Create new culling boundary
        *aabb = Aabb {
            center: Vec3A::ZERO,
            half_extents: Vec3A::new(dimension.size.x/2.0, dimension.size.y/2.0, 1.0),
        };

        // Create new mesh
        *mesh = msh.add(shape::Quad { size: dimension.size, flip: false });  //Into
    }
}


// #===============#
// #=== PLUGINS ===#

/// Plugin implementing all ui logic for the specified generic types.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// 
/// ## 🛠️ Example
/// *1. Define the types used*
/// ```
///  #[derive(Component, Default)]
///  struct MyMasterData { theme: String } // What data will each tree hold
/// 
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
///      .add_plugins(UiPlugin::<MyMasterData, MyNodeData, MyUiWidget>::new())
///      .run();
/// ```
/// *3. Use the [`UiTree`] freely*
/// ```
///#  fn setup(mut commands: Commands) {
///   commands.spawn((
///      MyUiWidget,
///      UiTree::<MyMasterData, MyNodeData>::new("MyWidget")
///   ));
///#  }
/// ```
#[derive(Debug, Default, Clone)]
pub struct UiPlugin <M:Default + Component, N:Default + Component, T: Component>(PhantomData<M>, PhantomData<N>, PhantomData<T>);
impl <M:Default + Component, N:Default + Component, T: Component> UiPlugin<M, N, T> {
    pub fn new() -> Self {
        UiPlugin::<M, N, T>(PhantomData, PhantomData, PhantomData)
    }
}
impl <M:Default + Component, N:Default + Component, T: Component> Plugin for UiPlugin<M, N, T> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, send_content_size_to_node::<M, N, T>.before(compute_ui::<M, N, T>))
            .add_systems(Update, send_stack_to_node::<M, N, T>.before(compute_ui::<M, N, T>))
            .add_systems(Update, send_layout_to_node::<M, N, T>.before(compute_ui::<M, N, T>))

            .add_systems(Update, fetch_transform_from_node::<M, N, T>)
            .add_systems(Update, (fetch_dimension_from_node::<M, N, T>, element_reconstruct_mesh::<T>).chain())
            .add_systems(Update, element_fetch_transform_from_node::<M, N, T>)
            .add_systems(Update, element_sprite_scale_to_dimension::<T>)

            .add_systems(Update, (fetch_dimension_from_camera::<M, N, T>, fetch_transform_from_camera::<T>).before(compute_ui::<M, N, T>))
            .add_systems(Update, compute_ui::<M, N, T>);
    }
}

/// Plugin implementing all debug ui logic for the specified generic types.
/// ## 📦 Types
/// * Generic `(M)` - Master data schema struct defining what can be stored in [`UiTree`]
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// 
/// ## 🛠️ Example
/// *1. Define the types used*
/// ```
///  #[derive(Component, Default)]
///  struct MyMasterData { theme: String } // What data will each tree hold
/// 
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
///      .add_plugins(UiPlugin::<MyMasterData, MyNodeData, MyUiWidget>::new())
///      .run();
/// ```
/// *3. Use the [`UiTree`] freely*
/// ```
///#  fn setup(mut commands: Commands) {
///   commands.spawn((
///      MyUiWidget,
///      UiTree::<MyMasterData, MyNodeData>::new("MyWidget")
///   ));
///#  }
/// ```
#[derive(Debug, Default, Clone)]
pub struct UiDebugPlugin <M:Default + Component, N:Default + Component, T: Component>(PhantomData<M>, PhantomData<N>, PhantomData<T>);
impl <M:Default + Component, N:Default + Component, T: Component> UiDebugPlugin<M, N, T> {
    pub fn new() -> Self {
        UiDebugPlugin::<M, N, T>(PhantomData, PhantomData, PhantomData)
    }
}
impl <M:Default + Component, N:Default + Component, T: Component> Plugin for UiDebugPlugin<M, N, T> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, debug_draw_gizmo::<M, N, T>)
            .add_systems(Update, debug_print_tree::<M, N, T>);
    }
}