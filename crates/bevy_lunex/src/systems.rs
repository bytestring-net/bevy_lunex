use crate::*;
use bevy::{math::Vec3A, render::primitives::Aabb, sprite::Mesh2dHandle, text::TextLayoutInfo, window::PrimaryWindow};
use lunex_engine::*;


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
        #[cfg(feature = "verbose")]
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

                let mut color = Color::linear_rgb(0.0, 1.0, 0.0);

                if let Some(Layout::Solid(_)) = container.layout.get(&container.layout_index[0]) { color = Color::linear_rgb(1.0, 1.0, 0.0) }

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
    source: Query<(&Camera, Option<&OrthographicProjection>), (With<T>, Changed<Camera>)>,
    mut destination: Query<&mut Dimension, (With<UiTree<T, N>>, With<MovableByCamera>)>
) {
    // Undesired behaviour if source.len() > 1
    for (cam, o_projection) in &source {
        for mut dimension in &mut destination {
            // Extract camera size
            if let Some(size) = cam.physical_viewport_size() {
                #[cfg(feature = "verbose")]
                info!("{} {} - Fetched Dimension data from Camera", "->".blue(), "UiTree".purple().bold());
                dimension.size = Vec2::from((size.x as f32, size.y as f32)) * if let Some(p) = o_projection { p.scale } else { 1.0 };
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
    source: Query<(&Camera, Option<&OrthographicProjection>), (With<T>, Changed<Camera>)>,
    mut destination: Query<&mut Transform, (With<UiTree<T, N>>, With<MovableByCamera>)>,
    window: Query<&bevy::window::Window, With<PrimaryWindow>>,
) {
    // Undesired behaviour if source.len() > 1
    for (cam, o_projection) in &source {
        let scale = if let Ok(window) = window.get_single() { window.resolution.scale_factor() } else { 1.0 };
        for mut transform in &mut destination {
            // Extract camera size
            if let Some(size) = cam.physical_viewport_size() {
                #[cfg(feature = "verbose")]
                info!("{} {} - Fetched Transform data from Camera", "->".blue(), "UiTree".purple().bold());
                let s = if let Some(p) = o_projection { p.scale } else { 1.0 };
                transform.translation.x = (size.x as f32 /-2.0 / scale) * s;
                transform.translation.y = (size.y as f32 / 2.0 / scale) * s;
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
        #[cfg(feature = "verbose")]
        info!("{} {} - Touched all cameras", "<>".red(), "Camera".purple().bold());
        for mut camera in &mut camera {
            camera.as_mut();
        }
    }
}

// #========================#
// #=== PIPING FOR NODES ===#

/// This system takes [`UiLayout`] data and overwrites coresponding [`UiTree`] data. If node is not found, it creates new ones along the path.
pub fn send_layout_to_node<T:Component, N:Default + Component, S: UiState>(
    mut uis: Query<(&mut UiTree<T, N>, &Children)>,
    query: Query<(&UiLink<T>, &UiLayout<S>), (Changed<UiLayout<S>>, Without<UiTree<T, N>>)>,
) {
    for (mut ui, children) in &mut uis {
        for child in children {
            // If child matches
            if let Ok((link, layout)) = query.get(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_or_create_ui_node_mut(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data_mut() {
                        #[cfg(feature = "verbose")]
                        info!("{} {} - Received Layout data", "->".blue(), link.path.yellow().bold());
                        container.layout.insert(S::INDEX, layout.layout);
                    }
                }
            }
        }
    }
}

/// This system takes [`UiLayoutController`] data and overwrites coresponding [`UiTree`] data.
/// ## 📦 Types
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn send_layout_control_to_node<T:Component, N:Default + Component>(
    mut uis: Query<(&mut UiTree<T, N>, &Children)>,
    query: Query<(&UiLink<T>, &UiLayoutController), Changed<UiLayoutController>>,
) {
    for (mut ui, children) in &mut uis {
        for child in children {
            // If child matches
            if let Ok((link, control)) = query.get(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node_mut(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data_mut() {
                        #[cfg(feature = "verbose")]
                        info!("{} {} - Tweening between [{}] [{}] - {}", "->".blue(), link.path.yellow().bold(), control.index[0], control.index[1], control.tween);
                        container.layout_index = control.index;
                        container.layout_tween = (control.method)(control.tween);
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
                        #[cfg(feature = "verbose")]
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
                        #[cfg(feature = "verbose")]
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
                        #[cfg(feature = "verbose")]
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
                        #[cfg(feature = "verbose")]
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
                            #[cfg(feature = "verbose")]
                            info!("{} {} - Linked {} fetched Dimension data from node: {:?}", "<-".bright_green(), link.path.yellow().bold(), "ENTITY".blue(), container.rectangle.size);
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
                        #[cfg(feature = "verbose")]
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
        #[cfg(feature = "verbose")]
        info!("{} {} - Piped Dimension into sprite size", "--".yellow(), "ELEMENT".red());
        sprite.custom_size = Some(dimension.size)
    }
}

/// This system fetches [`Dimension`] data and overwrites querried [`Handle<Image>`] data to fit.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn element_image_size_from_dimension<T: Component>(
    query: Query<(&Handle<Image>, &Dimension), (With<UiLink<T>>, With<Element>, With<MovableByCamera>, Changed<Dimension>)>,
    mut images: ResMut<Assets<Image>>,
) {
    for (handle, dimension) in &query {
        #[cfg(feature = "verbose")]
        info!("{} {} - Resizing texture based on Dimension", "--".yellow(), "ELEMENT".red());
        if let Some(image) = images.get_mut(handle) {
            image.resize(bevy::render::render_resource::Extent3d { width: dimension.size.x as u32, height: dimension.size.y as u32, ..default() });
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
    mut query: Query<(&Dimension, Option<&mut Handle<Mesh>>, Option<&mut Mesh2dHandle>, Option<&mut Aabb>), (With<UiLink<T>>, With<Element>, Or<(Changed<Dimension>, Added<Mesh2dHandle>)>)>,
) {
    for (dimension, mut mesh_option, mut mesh2d_option, mut aabb_option) in &mut query {

        #[cfg(feature = "verbose")]
        info!("{} {} - Reconstructed mesh size", "--".yellow(), "ELEMENT".red());

        if let Some(aabb) = aabb_option.as_mut() {
            // Create new culling boundary
            **aabb = Aabb {
                center: Vec3A::ZERO,
                half_extents: Vec3A::new(dimension.size.x/2.0, dimension.size.y/2.0, 1.0),
            };
        }

        if let Some(mesh) = mesh_option.as_mut() {
            // Unload old mesh
            let _ = msh.remove(mesh.id());

            // Create new mesh
            **mesh = msh.add(Rectangle {half_size: dimension.size / 2.0});
        }

        if let Some(mesh2d) = mesh2d_option.as_mut() {
            // Unload old mesh
            let _ = msh.remove(mesh2d.0.id());

            // Create new mesh
            **mesh2d = Mesh2dHandle(msh.add(Rectangle {half_size: dimension.size / 2.0}));
        }
    }
}

/// This system takes [`TextLayoutInfo`] data and overwrites coresponding [`Layout`] solid data.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn element_text_size_to_layout<T: Component>(
    mut query: Query<(&mut UiLayout, &TextLayoutInfo, &Text, Option<&UiTextSize>), (With<UiLink<T>>, With<Element>, Changed<TextLayoutInfo>)>,
) {
    for (mut layout, text_info, text, optional_text_size) in &mut query {
        #[cfg(feature = "verbose")]
        info!("{} {} - Converted text size into Layout", "--".yellow(), "ELEMENT".red());
        match &mut layout.layout {
            Layout::Window(window) => {
                let font_size = text.sections[0].style.font_size;
                window.size = if let Some(text_size) = optional_text_size {
                    match text_size.size {
                        UiValueType::Ab(t) => Ab(text_info.logical_size/font_size * t.0).into(),
                        UiValueType::Rl(t) => Rl(text_info.logical_size/font_size * t.0).into(),
                        UiValueType::Rw(t) => Rw(text_info.logical_size/font_size * t.0).into(),
                        UiValueType::Rh(t) => Rh(text_info.logical_size/font_size * t.0).into(),
                        UiValueType::Em(t) => Em(text_info.logical_size/font_size * t.0).into(),
                        UiValueType::Sp(t) => Sp(text_info.logical_size/font_size * t.0).into(),
                        UiValueType::Vp(t) => Vp(text_info.logical_size/font_size * t.0).into(),
                        UiValueType::Vw(t) => Vw(text_info.logical_size/font_size * t.0).into(),
                        UiValueType::Vh(t) => Vh(text_info.logical_size/font_size * t.0).into(),
                    }
                } else { Rh(text_info.logical_size).into() };
            },
            Layout::Solid(solid) => {solid.size = Ab(text_info.logical_size).into()},
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
        #[cfg(feature = "verbose")]
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
        #[cfg(feature = "verbose")]
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
pub struct UiGenericPlugin <T:Component = MainUi, N:Default + Component = NoData>(PhantomData<T>, PhantomData<N>);
impl <T:Component, N:Default + Component> UiGenericPlugin<T, N> {
    pub fn new() -> Self {
        UiGenericPlugin::<T, N>(PhantomData, PhantomData)
    }
}
impl <T:Component, N:Default + Component> Plugin for UiGenericPlugin<T, N> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                element_text_size_to_layout::<T>,
                element_text_size_to_content::<T>,
                touch_camera_if_uitree_added::<T, N>,
                fetch_dimension_from_camera::<T, N>.after(touch_camera_if_uitree_added::<T, N>),
                fetch_transform_from_camera::<T, N>.after(touch_camera_if_uitree_added::<T, N>),
            ).in_set(UiSystems::Modify).before(UiSystems::Send))

            //.add_plugins(StatePlugin::<T, N, Base>::new())
            .add_plugins(StatePlugin::<T, N, Hover>::new())
            .add_plugins(StatePlugin::<T, N, Clicked>::new())
            .add_plugins(StatePlugin::<T, N, Selected>::new())
            .add_plugins(StatePlugin::<T, N, Intro>::new())
            .add_plugins(StatePlugin::<T, N, Outro>::new())

            .add_systems(Update, (
                send_layout_to_node::<T, N, Base>,
                send_content_size_to_node::<T, N>,
                send_stack_to_node::<T, N>,
                send_layout_control_to_node::<T, N>,
                send_depth_bias_to_node::<T, N>
            ).chain().in_set(UiSystems::Send).before(UiSystems::Compute))

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
                element_image_size_from_dimension::<T>,
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
pub struct UiDebugPlugin <T:Component = MainUi, N:Default + Component = NoData>(PhantomData<T>, PhantomData<N>);
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
