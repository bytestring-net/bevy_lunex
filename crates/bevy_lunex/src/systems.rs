use crate::*;
use bevy::{math::Vec3A, render::primitives::Aabb, sprite::Mesh2dHandle, text::TextLayoutInfo, window::PrimaryWindow};
use lunex_engine::*;


// #===================#
// #=== CORE SYSTEM ===#

/// This system computes [`UiTree`] with data from querried [`Dimension`] component if there is a change.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
pub fn compute_ui<T:Component, N:Default + Component>(
    mut query: Query<(&Dimension, &mut UiTree<T, N>, Option<&SourceFromCamera>), (With<UiLink<T>>, Or<(Changed<UiTree<T, N>>, Changed<Dimension>)>)>,
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
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(G)` - [`GizmoConfigGroup`] that will be used to draw the outlines
pub fn debug_draw_gizmo<T:Component, N:Default + Component, G:GizmoConfigGroup>(
    mut query: Query<(&UiTree<T, N>, &GlobalTransform)>,
    mut gizmos: Gizmos<G>
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
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
pub fn debug_print_tree<T:Component, N:Default + Component>(
    uis: Query<&UiTree<T, N>, Changed<UiTree<T, N>>>
) {
    for ui in &uis {
        info!("{}\n{}\n", "Change detected...", ui.tree("show-hidden"));
    }
}


// #=========================#
// #=== PIPING FOR UITREE ===#

/// This system takes [`Camera`] data and overwrites querried [`Dimension`] + [`SourceFromCamera`].
/// It is mainly used to pipe [`Camera`] data into [`UiTree`] for root node computation.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
pub fn fetch_dimension_from_camera<T:Component, N:Default + Component>(
    source: Query<(&Camera, Option<&OrthographicProjection>), (With<T>, Changed<Camera>)>,
    mut destination: Query<&mut Dimension, (With<UiTree<T, N>>, With<SourceFromCamera>)>
) {
    if source.is_empty() { return; }
    let Ok((cam, o_projection)) = source.get_single() else {
        warn!("Multiple D cameras with UI marker component. Only a single camera can be used as source!");
        return;
    };

    for mut dimension in &mut destination {
        // Extract camera size
        if let Some(size) = cam.physical_viewport_size() {
            #[cfg(feature = "verbose")]
            info!("{} {} - Fetched Dimension data from Camera", "->".blue(), "UiTree".purple().bold());
            dimension.size = Vec2::from((size.x as f32, size.y as f32)) * if let Some(p) = o_projection { p.scale } else { 1.0 };
        }
    }
}

/// This system takes [`Camera`] data and overwrites querried [`Transform`] + [`SourceFromCamera`].
/// It is mainly used to pipe [`Camera`] data into [`UiTree`] for root node computation.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
pub fn fetch_transform_from_camera<T:Component, N:Default + Component>(
    source: Query<(&Camera, Option<&OrthographicProjection>), (With<T>, Changed<Camera>)>,
    mut destination: Query<&mut Transform, (With<UiTree<T, N>>, With<SourceFromCamera>)>,
    window: Query<&bevy::window::Window, With<PrimaryWindow>>,
) {
    if source.is_empty() { return; }
    let Ok((cam, o_projection)) = source.get_single() else {
        warn!("Multiple cameras with UI marker component. Only a single camera can be used as source!");
        return;
    };

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

/// This system listens for added [`UiTree`] components and if it finds one, mutable accesses all [`Camera`]s without changing them.
/// This way [`UiTree`]s that are spawned independently get the correct size immidietly piped into them.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
pub fn touch_camera_if_uitree_added<T:Component, N:Default + Component>(
    query: Query<Entity, (Added<UiTree<T, N>>, With<SourceFromCamera>)>,
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
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
/// * Generic `(S)` - A state generic for the given layout, as entities can have multiple layouts
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
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
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

/// # WORK IN PROGRESS!!! DOES NOTHING CURRENTLY.
/// This system takes [`UiStack`] data and overwrites coresponding [`UiTree`] data.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
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
                        container.stack = stack.clone();
                    }
                }
            }
        }
    }
}

/// This system takes [`UiDepthBias`] data and overwrites coresponding [`UiTree`] data.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
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

/// # WORK IN PROGRESS!!! DOES NOTHING CURRENTLY.
/// This system takes [`UiContent`] data and overwrites coresponding [`UiTree`] data.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
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

/// This system fetches computed [`UiTree`] data and overwrites querried [`Transform`] data.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
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

/// This system fetches computed [`UiTree`] data and overwrites querried [`Dimension`] data.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
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

/// This system takes computed [`UiTree`] data and overwrites querried [`Transform`] + [`Element`] data in specific way.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
/// * Generic `(N)` - Node data schema struct defining what can be stored in [`UiNode`]
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

/// This system takes updated [`Dimension`] data and overwrites querried [`Sprite`] data to fit.
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

/// This system takes updated [`Dimension`] data and overwrites querried [`Handle<Image>`] data to fit.
/// This is used to resize manually created render targets for secondary cameras, not textures.
/// ## 📦 Types
/// * Generic `(T)` - Marker component grouping entities into one widget type
pub fn element_image_size_from_dimension<T: Component>(
    query: Query<(&Handle<Image>, &Dimension), (With<UiLink<T>>, With<Element>, With<SourceFromCamera>, Changed<Dimension>)>,
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

/// This system takes updated [`Dimension`] data and reconstructs the mesh.
/// ## 📦 Types
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

/// This system takes updated [`TextLayoutInfo`] data and overwrites coresponding [`Layout`] data to match the text size.
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

/// # WORK IN PROGRESS!!! DOES NOTHING CURRENTLY.
/// This system takes updated [`TextLayoutInfo`] data and overwrites coresponding [`UiContent`] data to match the text size.
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

/// This system takes updated [`TextLayoutInfo`] data and overwrites coresponding [`Transform`] scale data for text to fit inside [`Dimension`].
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
///  #[derive(Component)]
///  struct MyUiWidget; // Empty marker, used for selecting entities
/// 
///  #[derive(Component, Default)]
///  struct MyNodeData { value: i32 } // What data will each node contain
/// ```
/// *2. Add the plugin to your app*
/// ```
///  App::new()
///      .add_plugins(DefaultPlugins)
///      .add_plugins(UiCorePlugin::<MyUiWidget, MyNodeData>::new())
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
pub struct UiCorePlugin <T:Component = MainUi, N:Default + Component = NoData>(PhantomData<T>, PhantomData<N>);
impl <T:Component, N:Default + Component> UiCorePlugin<T, N> {
    pub fn new() -> Self {
        UiCorePlugin::<T, N>(PhantomData, PhantomData)
    }
}
impl <T:Component, N:Default + Component> Plugin for UiCorePlugin<T, N> {
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
/// * Generic `(G)` - [`GizmoConfigGroup`] that will be used to draw the outlines
/// 
/// ## 🛠️ Example
/// *1. Define the types used*
/// ```
///  #[derive(Component)]
///  struct MyUiWidget; // Empty marker, used for selecting entities
/// 
///  #[derive(Component, Default)]
///  struct MyNodeData { value: i32 } // What data will each node contain
/// ```
/// *2. Add the plugin to your app*
/// ```
///  App::new()
///      .add_plugins(DefaultPlugins)
///      .add_plugins(UiCorePlugin::<MyUiWidget, MyNodeData>::new())
///      .add_plugins(UiDebugPlugin::<MyUiWidget, MyNodeData>::new())
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
pub struct UiDebugPlugin <T:Component = MainUi, N:Default + Component = NoData, G:GizmoConfigGroup = DefaultGizmoConfigGroup>(PhantomData<T>, PhantomData<N>, PhantomData<G>);
impl <T:Component, N:Default + Component, G:GizmoConfigGroup> UiDebugPlugin<T, N, G> {
    pub fn new() -> Self {
        UiDebugPlugin::<T, N, G>(PhantomData, PhantomData, PhantomData)
    }
}
impl <T:Component, N:Default + Component, G:GizmoConfigGroup> Plugin for UiDebugPlugin<T, N, G> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, debug_draw_gizmo::<T, N, G>)
            .add_systems(Update, debug_print_tree::<T, N>.after(UiSystems::Compute));
    }
}
