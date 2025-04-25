#![doc = include_str!("../README.md")]
#![allow(clippy::type_complexity)]

// Imports for this crate
pub(crate) use std::any::TypeId;
pub(crate) use bevy::prelude::*;
pub(crate) use bevy::app::PluginGroupBuilder;
use bevy::render::view::{self, VisibilityClass};
pub(crate) use bevy::sprite::Anchor;
pub(crate) use bevy::text::TextLayoutInfo;
pub(crate) use bevy::platform::collections::HashMap;
pub(crate) use bevy::render::view::RenderLayers;
pub(crate) use colored::Colorize;
#[cfg(feature = "text3d")]
pub(crate) use bevy_rich_text3d::*;
pub(crate) use bevy::render::{primitives::Aabb, mesh::MeshAabb};

// Imports from this crate
pub mod prelude {
    // Default plugins and system sets
    pub use crate::{UiLunexPlugins, UiSystems};
    // Debug plugins
    pub use crate::UiLunexDebugPlugin;

    // All standard components
    pub use crate::{
        Dimension,

        UiFetchFromCamera,
        UiSourceCamera,

        UiEmbedding,
        UiMeshPlane2d,
        UiMeshPlane3d,

        UiRoot3d,

        UiLayoutRoot,
        UiLayout,
        UiDepth,
        UiColor,

        UiImageSize,
        UiTextSize,

        UiBase,
    };

    // Import other file preludes
    pub use crate::cursor::prelude::*;
    pub use crate::layouts::prelude::*;
    pub use crate::states::prelude::*;
    pub use crate::units::*;

    // Export stuff from other crates
    pub use bevy::sprite::Anchor;
    #[cfg(feature = "text3d")]
    pub use bevy_rich_text3d::*;
    #[cfg(feature = "text3d")]
    pub use bevy::text::cosmic_text::Weight;
}

// Link files
mod cursor;
pub use cursor::*;
mod layouts;
pub use layouts::*;
mod picking;
pub use picking::*;
mod states;
pub use states::*;
mod units;
pub use units::*;


// #===============================#
// #=== MULTIPURPOSE COMPONENTS ===#

/// **Dimension** - This component holds width and height used for different Ui components
#[derive(Component, Reflect, Deref, DerefMut, Default, Clone, PartialEq, Debug)]
pub struct Dimension(pub Vec2);
/// Conversion implementations
impl <T: Into<Vec2>> From<T> for Dimension {
    fn from(value: T) -> Self {
        Dimension(value.into())
    }
}


// #=========================#
// #=== TEXTURE EMBEDDING ===#

/// **Ui Embedding** - Use this component to mark entities whose texture handles are embeddings instead of regular assets.
/// This means Lunex will resize the actual texture source when [`Dimension`] has changed.
#[derive(Component, Reflect, Clone, PartialEq, Debug)]
pub struct UiEmbedding;

/// This system takes [`Dimension`] data and pipes them into querried [`Handle<Image>`] data to fit.
/// This will resize the original image texture.
pub fn system_embedd_resize(
    query: Query<(&Sprite, &Dimension), (With<UiEmbedding>, Changed<Dimension>)>,
    mut images: ResMut<Assets<Image>>,
) {
    for (sprite, dimension) in &query {
        if let Some(image) = images.get_mut(&sprite.image) {
            if **dimension != Vec2::ZERO {
                image.resize(bevy::render::render_resource::Extent3d { width: dimension.x as u32, height: dimension.y as u32, ..default() });
            }
        }
    }
}

/// Provides utility constructor methods for [`Image`]
pub trait ImageTextureConstructor {
    /// Just a utility constructor hiding the necessary texture initialization
    fn clear_render_texture() -> Image {
        use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
        use bevy::asset::RenderAssetUsages;

        let mut image = Image::new_fill(
            Extent3d {
                width: 512,
                height: 512,
                ..default()
            },
            TextureDimension::D2,
            &[0, 0, 0, 0],
            TextureFormat::Bgra8UnormSrgb,
            RenderAssetUsages::default(),
        );
        image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
        image
    }
}
impl ImageTextureConstructor for Image {}

/// Provides utility costructor methods for [`Camera`]
pub trait CameraTextureRenderConstructor {
    /// Just a utility constructor for camera that renders to a transparent texture
    fn clear_render_to(handle: Handle<Image>) -> Camera {
        use bevy::render::camera::RenderTarget;
        Camera {
            target: RenderTarget::Image(handle.into()),
            clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            ..default()
        }
    }
    /// Modify the camera render order
    fn with_order(self, order: isize) -> Self;
}
impl CameraTextureRenderConstructor for Camera {
    fn with_order(mut self, order: isize) -> Self {
        self.order = order;
        self
    }
}


// #===========================#
// #=== LAYOUT ROOT CONTROL ===#

/// **Ui Layout Root** - This component marks the start of a worldspace Ui-Tree. Spawn this standalone for worldspace 3D UI
/// or spawn this as a child of camera for a HUD. For 2D UI, if your camera does not move you can spawn it standalone too.
///
/// Important components:
/// - [`Transform`] - Set the position of the Ui-Tree
/// - [`Dimension`] - Set the size of the Ui-Tree
/// ## 🛠️ Example
/// ```
/// # use bevy::prelude::*;
/// # use bevy_lunex::*;
/// # fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
///     commands.spawn((
///         UiLayoutRoot::new_2d(),
///         UiFetchFromCamera::<0>, // Pipe the size from Camera
///     )).with_children(|ui| {
///         // ... spawn your Ui Here
///     });
/// # }
/// ```
#[derive(Component, Reflect, Clone, PartialEq, Debug)]
#[require(Visibility, Transform, Dimension, VisibilityClass)]
#[component(on_add = view::add_visibility_class::<UiLayoutRoot>)]
pub struct UiLayoutRoot {
    abs_scale: f32,
}
impl UiLayoutRoot {
    pub fn new_2d() -> Self {
        Self { abs_scale: 1.0 }
    }
    pub fn new_3d() -> Self {
        Self { abs_scale: 0.001 }
    }
}


/// **Ui Root 3d** - This is a marker component for all entities which fall under a 3D UI. You can check through this component
/// if a specific node is 2D or 3D without looking for its root.
#[derive(Component, Reflect, Clone, PartialEq, Debug)]
pub struct UiRoot3d;

/// This system traverses the hierarchy and adds [`UiRoot3d`] component to children.
pub fn system_mark_3d(
    root_query: Query<(Has<UiRoot3d>, &Children), (With<UiLayoutRoot>, Without<UiLayout>, Changed<UiLayoutRoot>)>,
    node_query: Query<(Entity, Has<UiRoot3d>, Option<&Children>), (With<UiLayout>, Without<UiLayoutRoot>)>,
    mut commands: Commands,
) {
    for (is_root_3d, root_children) in &root_query {

        // Stack-based traversal
        let mut stack: Vec<(Entity, usize)> = root_children.iter().map(|child| (child, 1)).rev().collect();

        // Loop over the stack
        while let Some((current_entity, depth)) = stack.pop() {
            if let Ok((node, is_node_3d, node_children_option)) = node_query.get(current_entity) {


                if is_root_3d != is_node_3d {
                    if is_root_3d {
                        commands.entity(node).insert(UiRoot3d);
                    } else {
                        commands.entity(node).remove::<UiRoot3d>();
                    }
                }


                // Push children to the stack
                if let Some(node_children) = node_children_option {
                    for child in node_children.iter().rev() {
                        stack.push((child, depth + 1));
                    }
                }
            }
        }
    }
}



/// Trigger this event to recompute all [`UiLayoutRoot`] entities.
#[derive(Event)]
pub struct RecomputeUiLayout;

/// This observer will mutably touch [`UiLayoutRoot`] which will trigger [`system_layout_compute`].
pub fn observer_touch_layout_root(
    _trigger: Trigger<RecomputeUiLayout>,
    mut query: Query<&mut UiLayoutRoot>,
){
    for mut root in &mut query {
        root.as_mut();
    }
}

/// This system draws the outlines of [`UiLayout`] and [`UiLayoutRoot`] as gizmos.
pub fn system_debug_draw_gizmo_2d(
    query: Query<(&GlobalTransform, &Dimension), (Or<(With<UiLayout>, With<UiLayoutRoot>)>, Without<UiRoot3d>)>,
    mut gizmos: Gizmos<LunexGizmoGroup2d>
) {
    for (transform, dimension) in &query {
        // Draw the gizmo outline
        gizmos.rect(
            Isometry3d::new(transform.translation(), transform.rotation()),
            **dimension,
            Color::linear_rgb(0.0, 1.0, 0.0),
        );
    }
}

/// This system draws the outlines of [`UiLayout`] and [`UiLayoutRoot`] as gizmos.
pub fn system_debug_draw_gizmo_3d(
    query: Query<(&GlobalTransform, &Dimension), (Or<(With<UiLayout>, With<UiLayoutRoot>)>, With<UiRoot3d>)>,
    mut gizmos: Gizmos<LunexGizmoGroup3d>
) {
    for (transform, dimension) in &query {
        // Draw the gizmo outline
        gizmos.rect(
            Isometry3d::new(transform.translation(), transform.rotation()),
            **dimension,
            Color::linear_rgb(0.0, 1.0, 0.0),
        );
    }
}

/// This system traverses the hierarchy and prints the debug information.
pub fn system_debug_print_data(
    root_query: Query<(&UiLayoutRoot, NameOrEntity, &Dimension, &Children), (Without<UiLayout>, Or<(Changed<UiLayoutRoot>, Changed<Dimension>)>)>,
    node_query: Query<(&UiLayout, &UiState, NameOrEntity, &Dimension, &Transform, Option<&Children>), Without<UiLayoutRoot>>,
) {
    for (_, root_name, root_dimension, root_children) in &root_query {
        // Create output string
        let mut output_string = format!("▶ {}", format!("{root_name}").bold().underline().magenta());

        output_string += " ⇒ ";
        output_string += &format!("[w: {}, h: {}]", format!("{:.02}", root_dimension.x).green(), format!("{:.02}", root_dimension.y).green());

        output_string += "\n";

        // Stack-based traversal
        let mut stack: Vec<(Entity, usize, bool)> = root_children
            .iter()
            .enumerate()
            .map(|(i, child)| (child, 1, i == root_children.len() - 1)) // Track last-child flag
            .rev()
            .collect();

        // Tracks whether previous levels had last children (for vertical bars)
        let mut last_child_levels: Vec<bool> = Vec::new();

        while let Some((current_entity, depth, is_last)) = stack.pop() {
            if let Ok((node_layout, _node_state, node_name, node_dimension, node_transform, node_children_option)) = node_query.get(current_entity) {

                // Adjust last_child_levels size
                if last_child_levels.len() < depth {
                    last_child_levels.push(is_last);
                } else {
                    last_child_levels[depth - 1] = is_last;
                }

                // Create the tab level offset
                for &last in &last_child_levels[..depth - 1] {
                    output_string += &if last { format!("{}", "  ┆".black()) } else { "  │".to_string() };
                }

                // Add the name
                output_string += if is_last { "  └" } else { "  ├" };
                if node_name.name.is_some() {
                    output_string += &format!("─ {}", format!("{node_name}").bold().yellow());
                } else {
                    output_string += &format!("─ {}", format!("{node_name}").yellow());
                }

                output_string += " ⇒ ";

                output_string += &format!("[w: {}, h: {}, d: {}]",
                    format!("{:.02}", node_dimension.x).green(),
                    format!("{:.02}", node_dimension.y).green(),
                    format!("{:.00}", node_transform.translation.z).green(),
                );

                match node_layout.layouts.get(&UiBase::id()).unwrap() {
                    UiLayoutType::Boundary(boundary) => {
                        output_string += &format!(" ➜ {} {} p1: {}, p2: {} {}",
                            "Boundary".bold(),
                            "{",
                            boundary.pos1.to_nicestr(),
                            boundary.pos2.to_nicestr(),
                            "}",
                        );
                    },
                    UiLayoutType::Window(window) => {
                        output_string += &format!(" ➜ {} {} p: {}, s: {}, a: {} {}",
                            "Window".bold(),
                            "{",
                            window.pos.to_nicestr(),
                            window.size.to_nicestr(),
                            window.anchor.to_nicestr(),
                            "}",
                        );
                    },
                    UiLayoutType::Solid(solid) => {
                        output_string += &format!(" ➜ {} {} s: {}, ax: {}, ay: {}, scl: {} {}",
                            "Solid".bold(),
                            "{",
                            solid.size.to_nicestr(),
                            format!("{:.02}", solid.align_x.0).green(),
                            format!("{:.02}", solid.align_y.0).green(),
                            format!("{:?}", solid.scaling).green(),
                            "}",
                        );
                    },
                }

                output_string += "\n";

                if let Some(node_children) = node_children_option {
                    let child_count = node_children.len();
                    for (i, child) in node_children.iter().enumerate().rev() {
                        stack.push((child, depth + 1, i == child_count - 1));
                    }
                }
            }
        }

        // Print to console
        info!("UiLayout change detected:\n{}", output_string);
    }
}


// #======================#
// #=== LAYOUT CONTROL ===#

/// **Ui Layout** - This component specifies the layout of a Ui-Node, which must be spawned as a child
/// of either [`UiLayoutRoot`] or [`UiLayout`] to work. Based on the provided layout other attached
/// components on this entity are overwritten to match the computed structure.
///
/// Direct output components:
/// - [`Transform`] - The computed position of the Ui-Node _(Read-only)_
/// - [`Dimension`] - The computed size of the Ui-Node _(Read-only)_
///
/// ## 🛠️ Example
/// ```
/// # use bevy::prelude::*;
/// # use bevy_lunex::*;
/// # fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
/// # commands.spawn((
/// #     UiLayoutRoot::new_2d(),
/// # )).with_children(|ui| {
///       // Must be spawned as a child
///       ui.spawn((
///           // Use 1 of the 3 available layout types
///           UiLayout::solid().size((1920.0, 1080.0)).scaling(Scaling::Fill).pack(),
///           // Attach image to the node
///           Sprite::from_image(asset_server.load("images/ui/background.png")),
///       ));
/// # });
/// # }
/// ```
#[derive(Component, Reflect, Clone, PartialEq, Debug)]
#[require(Visibility, Transform, Dimension, VisibilityClass, UiState, UiDepth)]
#[component(on_add = view::add_visibility_class::<UiLayout>)]
pub struct UiLayout {
    /// Stored layout per state
    pub layouts: HashMap<TypeId, UiLayoutType>
}
/// Constructors
impl UiLayout {
    /// **Boundary** - Declarative layout type that is defined by its top-left corner and bottom-right corner.
    /// Nodes with this layout are not included in the ui flow.
    /// ## 🛠️ Example
    /// ```
    /// # use bevy_lunex::{UiLayout, Rl};
    /// let layout: UiLayout = UiLayout::boundary().pos1(Rl(20.0)).pos2(Rl(80.0)).pack();
    /// ```
    pub fn boundary() -> UiLayoutTypeBoundary {
        UiLayoutTypeBoundary::new()
    }
    /// **Window** - Declarative layout type that is defined by its size and position.
    /// Nodes with this layout are not included in the ui flow.
    /// ## 🛠️ Example
    /// ```
    /// # use bevy_lunex::{UiLayout, Ab, Rl};
    /// let layout: UiLayout = UiLayout::window().pos(Ab(100.0)).size(Rl(50.0)).pack();
    /// ```
    pub fn window() -> UiLayoutTypeWindow {
        UiLayoutTypeWindow::new()
    }
    /// **Solid** - Declarative layout type that is defined by its width and height ratio.
    /// Scales in a way to fit itself inside parent container. It never deforms.
    /// Nodes with this layout are not included in the ui flow.
    /// ## 🛠️ Example
    /// ```
    /// # use bevy_lunex::UiLayout;
    /// let layout: UiLayout = UiLayout::solid().size((4.0, 3.0)).align_x(-0.8).pack();
    /// ```
    pub fn solid() -> UiLayoutTypeSolid {
        UiLayoutTypeSolid::new()
    }
    /// Create multiple layouts for a different states at once.
    pub fn new(value: Vec<(TypeId, impl Into<UiLayoutType>)>) -> Self {
        let mut map = HashMap::new();
        for (state, layout) in value {
            map.insert(state, layout.into());
        }
        Self { layouts: map }
    }
}
/// Conversion implementations
impl From<UiLayoutType> for UiLayout {
    fn from(value: UiLayoutType) -> Self {
        let mut map = HashMap::new();
        map.insert(UiBase::id(), value);
        Self {
            layouts: map,
        }
    }
}
impl From<UiLayoutTypeBoundary> for UiLayout {
    fn from(value: UiLayoutTypeBoundary) -> Self {
        let value: UiLayoutType = value.into();
        UiLayout::from(value)
    }
}
impl From<UiLayoutTypeWindow> for UiLayout {
    fn from(value: UiLayoutTypeWindow) -> Self {
        let value: UiLayoutType = value.into();
        UiLayout::from(value)
    }
}
impl From<UiLayoutTypeSolid> for UiLayout {
    fn from(value: UiLayoutTypeSolid) -> Self {
        let value: UiLayoutType = value.into();
        UiLayout::from(value)
    }
}

pub fn system_recompute_on_change <C: Component>(query: Query<Entity, Changed<C>>, mut commands: Commands){
    if !query.is_empty() { commands.trigger(RecomputeUiLayout); }
}

/// **Ui Depth** - This component overrides the default Z axis (depth) stacking order.
/// This is useful when fixing Z order flickering. Another use can be offseting an background
/// image for example.
#[derive(Component, Reflect, Clone, PartialEq, Debug)]
pub enum UiDepth {
    /// Add to existing depth
    Add(f32),
    /// Override existing depth
    Set(f32),
}
impl Default for UiDepth {
    fn default() -> Self {
        UiDepth::Add(1.0)
    }
}


/// This system traverses the hierarchy and computes all nodes.
pub fn system_layout_compute(
    root_query: Query<(&UiLayoutRoot, &Transform, &Dimension, &Children), (Without<UiLayout>, Or<(Changed<UiLayoutRoot>, Changed<Dimension>)>)>,
    mut node_query: Query<(&UiLayout, &UiDepth, &UiState, &mut Transform, &mut Dimension, Option<&Children>), Without<UiLayoutRoot>>,
) {
    for (root, root_transform, root_dimension, root_children) in &root_query {
        // Size of the viewport
        let root_rectangle = Rectangle2D {
            pos: root_transform.translation.xy(),
            size: **root_dimension,
        };

        // Stack-based traversal
        let mut stack: Vec<(Entity, Rectangle2D, f32)> = root_children.iter().map(|child| (child, root_rectangle, 0.0)).rev().collect();

        while let Some((current_entity, parent_rectangle, depth)) = stack.pop() {
            if let Ok((node_layout, node_depth, node_state, mut node_transform, mut node_dimension, node_children_option)) = node_query.get_mut(current_entity) {
                // Compute all layouts for the node
                let mut computed_rectangles = Vec::with_capacity(node_layout.layouts.len());
                for (state, layout) in &node_layout.layouts {
                    computed_rectangles.push((state, layout.compute(&parent_rectangle, root.abs_scale, root_rectangle.size, 16.0)));
                }

                // Normalize the active state weights
                let mut total_weight = 0.0;
                for (state, _) in &node_layout.layouts {
                    if let Some(weight) = node_state.states.get(state) {
                        total_weight += weight;
                    }
                }

                // Combine the state rectangles into one normalized
                let mut node_rectangle = Rectangle2D::EMPTY;

                // Use base if no active state
                if total_weight == 0.0 {
                    node_rectangle.pos += computed_rectangles[0].1.pos;
                    node_rectangle.size += computed_rectangles[0].1.size;

                // Combine the active states into one rectangle
                } else {
                    for (state, rectangle) in computed_rectangles {
                        if let Some(weight) = node_state.states.get(state) {
                            node_rectangle.pos += rectangle.pos * (weight / total_weight);
                            node_rectangle.size += rectangle.size * (weight / total_weight);
                        }
                    }
                }

                // Save the computed layout
                node_transform.translation.x = node_rectangle.pos.x;
                node_transform.translation.y = -node_rectangle.pos.y;
                let depth = match node_depth {
                    UiDepth::Add(v) => {depth + v},
                    UiDepth::Set(v) => {*v},
                };
                node_transform.translation.z = depth * root.abs_scale;
                **node_dimension = node_rectangle.size;

                if let Some(node_children) = node_children_option {
                    // Add children to the stack
                    stack.extend(node_children.iter().map(|child| (child, node_rectangle, depth)));
                }
            }
        }
    }
}


// #=====================#
// #=== STATE CONTROL ===#

/// **Ui State** - This component aggrages state transition values for later reference
/// by other components. You don't directly control or spawn this component, but use an abstraction
/// instead. You can use the prebuilt state components or create a custom ones with a completely
/// unique transition logic. You just have to provide transition value to this component later.
/// - [`UiBase`] _(Type only, not a component)_
/// - [`UiHover`]
/// - [`UiSelected`]
/// - [`UiClicked`]
/// - [`UiIntro`]
/// - [`UiOutro`]
///
/// Dependant components:
/// - [`UiLayout`]
/// - [`UiColor`]
///
/// ## 🛠️ Example
/// ```
/// # use bevy::prelude::*;
/// # use bevy::color::palettes::basic::*;
/// # use bevy_lunex::*;
/// # fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
/// # commands.spawn((
/// #     UiLayoutRoot::new_2d(),
/// # )).with_children(|ui| {
///       ui.spawn((
///           // Like this you can enable a state
///           UiHover::new().forward_speed(20.0).backward_speed(4.0),
///           // You can define layouts per state
///           UiLayout::new(vec![
///               (UiBase::id(), UiLayout::window().full()),
///               (UiHover::id(), UiLayout::window().x(Rl(10.0)).full())
///           ]),
///           // You can define colors per state
///           UiColor::new(vec![
///               (UiBase::id(), Color::Srgba(RED).with_alpha(0.8)),
///               (UiHover::id(), Color::Srgba(YELLOW).with_alpha(1.2))
///           ]),
///           // ... Sprite, Text, etc.
///
///       // Add observers that enable/disable the hover state component
///       )).observe(hover_set::<Pointer<Over>, true>)
///         .observe(hover_set::<Pointer<Out>, false>);
/// # });
/// # }
/// ```
#[derive(Component, Reflect, Clone, PartialEq, Debug)]
pub struct UiState {
    /// Stored transition per state
    states: HashMap<TypeId, f32>,
}
/// Default constructor
impl Default for UiState {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(UiBase::id(), 1.0);
        Self {
            states: map,
        }
    }
}

/// This system controls the [`UiBase`] state. This state is decreased based on total sum of all other active states.
pub fn system_state_base_balancer(
    mut query: Query<&mut UiState, Changed<UiState>>,
) {
    for mut manager in &mut query {
        // Normalize the active nobase state weights
        let mut total_nonbase_weight = 0.0;
        for (state, value) in &manager.states {
            if *state == UiBase::id() { continue; }
            total_nonbase_weight += value;
        }

        // Decrease base transition based on other states
        if let Some(value) = manager.states.get_mut(&UiBase::id()) {
            *value = (1.0 - total_nonbase_weight).clamp(0.0, 1.0);
        }
    }
}
/// This system pipes the attached state component data to the [`UiState`] component.
pub fn system_state_pipe_into_manager<S: UiStateTrait + Component>(
    mut commads: Commands,
    mut query: Query<(&mut UiState, &S), Changed<S>>,
) {
    for (mut manager, state) in &mut query {
        // Send the value to the manager
        if let Some(value) = manager.states.get_mut(&S::id()) {
            *value = state.value();

        // Insert the value if it does not exist
        } else {
            manager.states.insert(S::id(), state.value());
        }
        // Recompute layout
        commads.trigger(RecomputeUiLayout);
    }
}

/// Trait that all states must implement before being integrated into the state machine.
pub trait UiStateTrait: Send + Sync + 'static {
    /// This is used as a key to identify a Ui-Node state.
    fn id() -> TypeId {
        TypeId::of::<Self>()
    }
    /// This must return a value between `0.0 - 1.0`. It is used as transition value
    /// for a state, with `0.0` being off and `1.0` being on. Any smoothing should happen
    /// inside this function.
    fn value(&self) -> f32;
}

/// **Ui Base** - The default state for a Ui-Node, used only for the [`UiBase::id`] key. It is not a component that you can control.
#[derive(Clone, PartialEq, Debug)]
pub struct UiBase;
impl UiStateTrait for UiBase {
    fn value(&self) -> f32 {
        1.0
    }
}


// #=====================#
// #=== IMAGE CONTROL ===#

/// **Ui Image Size** - This component makes image size the authority instead.
#[derive(Component, Reflect, Deref, DerefMut, Default, Clone, PartialEq, Debug)]
pub struct UiImageSize (pub UiValue<Vec2>);
/// Constructors
impl <T: Into<UiValue<Vec2>>> From<T> for UiImageSize {
    fn from(value: T) -> Self {
        UiImageSize(value.into())
    }
}

/// This system takes [`Dimension`] data and pipes them into querried [`Sprite`].
pub fn system_pipe_sprite_size_from_dimension(
    mut query: Query<(&mut Sprite, &Dimension), Changed<Dimension>>,
) {
    for (mut sprite, dimension) in &mut query {
        sprite.custom_size = Some(**dimension)
    }
}

/// This system takes updated [`Handle<Image>`] data and overwrites coresponding [`UiLayout`] data to match the text size.
pub fn system_image_size_to_layout(
    images: Res<Assets<Image>>,
    mut query: Query<(&mut UiLayout, &Sprite, &UiImageSize)>,
) {
    for (mut layout, sprite, image_size) in &mut query {
        if let Some(image) = images.get(&sprite.image) {
            let x = image_size.get_x() * image.width() as f32;
            let y = image_size.get_y() * image.height() as f32;

            if match layout.layouts.get(&UiBase::id()).unwrap() {
                UiLayoutType::Window(window) => window.size.get_x() != x || window.size.get_y() != y,
                UiLayoutType::Solid(solid) => solid.size.get_x() != x || solid.size.get_y() != y,
                _ => false,
            } {
                match layout.layouts.get_mut(&UiBase::id()).unwrap() {
                    UiLayoutType::Window(window) => { window.set_width(x); window.set_height(y); },
                    UiLayoutType::Solid(solid) => { solid.set_width(x); solid.set_height(y); },
                    _ => {},
                }
            }
        }
    }
}


// #====================#
// #=== TEXT CONTROL ===#

/// **Ui Text Size** - This component is used to control the size of a text compared
/// to other Ui-Nodes. It works by overwritting the attached [`UiLayout`] window
/// size parameter to match the text bounds. The value provided is used as a _scale_
/// to adjust this size, specificaly it's height. It is recommended to use `non-relative`
/// units such as [`Ab`], [`Rw`], [`Rh`], [`Vh`], [`Vw`] and [`Em`] for even values.
///
/// Affected components:
/// - [`UiLayout`] - **MUST BE WINDOW TYPE** for this to work
///
/// ## 🛠️ Example
/// ```
/// # use bevy::prelude::*;
/// # use bevy_lunex::*;
/// # fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
/// # commands.spawn((
/// #     UiLayoutRoot::new_2d(),
/// # )).with_children(|ui| {
///       ui.spawn((
///           // Position the text using the window layout's position and anchor
///           UiLayout::window().pos((Rh(40.0), Rl(50.0))).anchor(Anchor::CenterLeft).pack(),
///           // This controls the height of the text, so 60% of the parent's node height
///           UiTextSize::from(Rh(60.0)),
///           // You can attach text like this
///           Text2d::new("Button"),
///           // Font size now works as "text resolution"
///           TextFont {
///               font: asset_server.load("fonts/Rajdhani.ttf"),
///               font_size: 64.0,
///               ..default()
///           },
///       ));
/// # });
/// # }
/// ```
#[derive(Component, Reflect, Deref, DerefMut, Default, Clone, PartialEq, Debug)]
pub struct UiTextSize (pub UiValue<f32>);
/// Constructors
impl <T: Into<UiValue<f32>>> From<T> for UiTextSize {
    fn from(value: T) -> Self {
        UiTextSize(value.into())
    }
}

/// This system takes [`TextLayoutInfo`] data and pipes them into querried [`Transform`] scale.
pub fn system_text_size_from_dimension(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Dimension, &TextLayoutInfo), Changed<Dimension>>,
) {
    for (mut transform, dimension, text_info) in &mut query {
        // Wait for text to render
        if text_info.size.y == 0.0 {
            commands.trigger(RecomputeUiLayout);
        }

        // Scale the text
        let scale = **dimension / text_info.size;
        transform.scale.x = scale.x;
        transform.scale.y = scale.x;
    }
}

/// This system takes updated [`TextLayoutInfo`] data and overwrites coresponding [`UiLayout`] data to match the text size.
pub fn system_text_size_to_layout(
    mut commands: Commands,
    mut query: Query<(&mut UiLayout, &Text2d, &TextLayoutInfo, &UiTextSize), Changed<TextLayoutInfo>>,
) {
    for (mut layout, text, text_info, text_size) in &mut query {
        // Wait for text to render
        if text_info.size.y == 0.0 {
            commands.trigger(RecomputeUiLayout);
        }

        // Create the text layout
        match layout.layouts.get_mut(&UiBase::id()).expect("UiBase state not found for Text") {
            UiLayoutType::Window(window) => {
                let lines = 1 + text.trim().matches('\n').count();
                window.set_height(**text_size * (lines as f32));
                window.set_width(**text_size * (lines as f32) * (text_info.size.x / text_info.size.y));
            },
            UiLayoutType::Solid(solid) => {
                solid.set_size(Ab(text_info.size));
            },
            _ => {},
        }
    }
}

// # TEXT 3D

/// This system takes updated [`Text3dDimensionOut`] data and overwrites coresponding [`UiLayout`] data to match the text size.
#[cfg(feature = "text3d")]
pub fn system_text_3d_size_to_layout(
    mut commands: Commands,
    mut query: Query<(&mut UiLayout, &Text3d, &Text3dDimensionOut, &UiTextSize), Changed<Text3dDimensionOut>>,
) {
    for (mut layout, text, text_info, text_size) in &mut query {
        // Wait for text to render
        if text_info.dimension.y == 0.0 {
            commands.trigger(RecomputeUiLayout);
            continue;
        }

        // Create the text layout
        match layout.layouts.get_mut(&UiBase::id()).expect("UiBase state not found for Text") {
            UiLayoutType::Window(window) => {
                let lines = 1 + text.get_single()
                    .expect("Multisegment 3D text not supported, make a PR to Lunex if you need it")
                    .trim().matches('\n').count();
                window.set_height(**text_size * (lines as f32));
                window.set_width(**text_size * (lines as f32) * (text_info.dimension.x / text_info.dimension.y));
            },
            UiLayoutType::Solid(solid) => {
                solid.set_size(Ab(text_info.dimension));
            },
            _ => {},
        }
    }
}

/// This system takes [`Text3dDimensionOut`] data and pipes them into querried [`Transform`] scale.
#[cfg(feature = "text3d")]
pub fn system_text_3d_size_from_dimension(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Dimension, &Text3dDimensionOut), Changed<Dimension>>,
) {
    for (mut transform, dimension, text_info) in &mut query {
        // Wait for text to render
        if text_info.dimension.y == 0.0 {
            commands.trigger(RecomputeUiLayout);
            continue;
        }

        // Scale the text
        let scale = dimension.x / text_info.dimension.x;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}


// #=====================#
// #=== STATE CONTROL ===#

/// **Ui Mesh Plane 3d** - This component is used to mark mesh entities that can be freely replaced with quad mesh on demand.
#[derive(Component, Reflect, Default, Clone, PartialEq, Debug)]
#[require(Mesh3d)]
pub struct UiMeshPlane3d;

/// **Ui Mesh Plane 2d** - This component is used to mark mesh entities that can be freely replaced with quad mesh on demand.
#[derive(Component, Reflect, Default, Clone, PartialEq, Debug)]
#[require(Mesh2d)]
pub struct UiMeshPlane2d;

/// This system takes [`Dimension`] data and constructs a plane mesh.
pub fn system_mesh_3d_reconstruct_from_dimension(
    mut query: Query<(&Dimension, &mut Mesh3d, Option<&mut Aabb>), (With<UiMeshPlane3d>, Changed<Dimension>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (dimension, mut mesh, aabb_option) in &mut query {
        let plane_mesh = Mesh::from(Rectangle::new(dimension.x, dimension.y));
        if let Some(a) = plane_mesh.compute_aabb() {
            if let Some(mut aabb) = aabb_option {
                *aabb = a;
            }
        }
        mesh.0 = meshes.add(plane_mesh);
    }
}

/// This system takes [`Dimension`] data and constructs a plane mesh.
pub fn system_mesh_2d_reconstruct_from_dimension(
    mut query: Query<(&Dimension, &mut Mesh2d, Option<&mut Aabb>), (With<UiMeshPlane2d>, Changed<Dimension>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (dimension, mut mesh, aabb_option) in &mut query {
        let plane_mesh = Mesh::from(Rectangle::new(dimension.x, dimension.y));
        if let Some(a) = plane_mesh.compute_aabb() {
            if let Some(mut aabb) = aabb_option {
                *aabb = a;
            }
        }
        mesh.0 = meshes.add(plane_mesh);
    }
}



// #=======================#
// #=== CAMERA FETCHING ===#

/// **Ui Fetch From Camera** - Attaching this component to [`UiLayoutRoot`] will make the [`Dimension`]
/// component pull data from a [`Camera`] with attached [`UiSourceCamera`] that has the same index.
#[derive(Component, Reflect, Clone, PartialEq, Debug)]
pub struct UiFetchFromCamera<const INDEX: usize>;

/// **Ui Source Camera** - Marks a [`Camera`] as a source for [`UiLayoutRoot`] with [`UiFetchFromCamera`].
/// They must have the same index and only one [`UiSourceCamera`] can exist for a single index.
#[derive(Component, Reflect, Clone, PartialEq, Debug)]
pub struct UiSourceCamera<const INDEX: usize>;

/// This system takes [`Camera`] viewport data and pipes them into querried [`Dimension`] + [`UiLayoutRoot`] + [`UiFetchFromCamera`].
pub fn system_fetch_dimension_from_camera<const INDEX: usize>(
    src_query: Query<(&Camera, Option<&Projection>), (With<UiSourceCamera<INDEX>>, Changed<Camera>)>,
    mut dst_query: Query<&mut Dimension, (With<UiLayoutRoot>, With<UiFetchFromCamera<INDEX>>)>,
) {
    // Check if we have a camera dimension input
    if src_query.is_empty() { return; }
    let Ok((camera, projection_option)) = src_query.single() else {
        warn_once!("Multiple UiSourceCamera<{INDEX}> exist at once! Ignoring all camera inputs to avoid unexpected behavior!");
        return;
    };

    // Pipe the camera viewport size
    if let Some(cam_size) = camera.logical_viewport_size() {
        for mut size in &mut dst_query {
            **size = Vec2::from((cam_size.x, cam_size.y)) * if let Some(Projection::Orthographic(p)) = projection_option { p.scale } else { 1.0 };
        }
    }
}

/// This system listens for added [`UiFetchFromCamera`] components and if it finds one, mutable accesses all [`Camera`]s to trigger fetching systems.
pub fn system_touch_camera_if_fetch_added<const INDEX: usize>(
    query: Query<Entity, Added<UiFetchFromCamera<INDEX>>>,
    mut cameras: Query<&mut Camera, With<UiSourceCamera<INDEX>>>,
){
    if !query.is_empty() {
        for mut camera in &mut cameras {
            camera.as_mut();
        }
    }
}


// #===================#
// #=== STYLE COLOR ===#

/// **Ui Color** - This component is used to control the color of the Ui-Node.
/// It is synchronized with a state machine and allows for specifying unique
/// colors for each state.
///
/// Affected components:
/// - [`Sprite`]
/// - [`TextColor`]
/// - the [`ColorMaterial`] of [`MeshMaterial2d`]
/// - the [`StandardMaterial`] of [`MeshMaterial3d`]
///
/// ## 🛠️ Example
/// ```
/// # use bevy::prelude::*;
/// # use bevy::color::palettes::basic::*;
/// # use bevy_lunex::*;
/// # fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
/// # commands.spawn((
/// #     UiLayoutRoot::new_2d(),
/// # )).with_children(|ui| {
///       // Spawn as a single color
///       ui.spawn((
///           // ... Layout, etc.
///           UiColor::from(Color::Srgba(RED).with_alpha(0.8)),
///           // ... Sprite, Text, etc.
///       ));
///
///       // Spawn as a collection for different states
///       ui.spawn((
///           // ... Layout, etc.
///           UiColor::new(vec![
///               (UiBase::id(), Color::Srgba(RED).with_alpha(0.8)),
///               (UiHover::id(), Color::Srgba(YELLOW).with_alpha(1.2))
///           ]),
///           // ... Sprite, Text, etc.
///       ));
/// # });
/// # }
/// ```
#[derive(Component, Reflect, Deref, DerefMut, Default, Clone, PartialEq, Debug)]
pub struct UiColor {
    colors: HashMap<TypeId, Color>
}
/// Constructors
impl UiColor {
    /// Define multiple states at once using a vec.
    pub fn new(value: Vec<(TypeId, impl Into<Color>)>) -> Self {
        let mut map = HashMap::new();
        for (state, layout) in value {
            map.insert(state, layout.into());
        }
        Self { colors: map }
    }
}
/// Conversion implementations
impl <T: Into<Color>> From<T> for UiColor {
    fn from(value: T) -> Self {
        let mut map = HashMap::new();
        map.insert(UiBase::id(), value.into());
        Self {
            colors: map,
        }
    }
}

/// This system takes care of [`UiColor`] data and updates querried [`Sprite`] and [`TextColor`] components.
/// and updates [`ColorMaterial`] and [`StandardMaterial`]
pub fn system_color(
    mut query: Query<(
        Option<&mut Sprite>,
        Option<&mut TextColor>,
        Option<&MeshMaterial2d<ColorMaterial>>,
        Option<&MeshMaterial3d<StandardMaterial>>,
        &UiColor,
        &UiState,
    ), Or<(Changed<UiColor>, Changed<UiState>)>>,
    mut materials2d: ResMut<Assets<ColorMaterial>>,
    mut materials3d: ResMut<Assets<StandardMaterial>>,
) {
    for (node_sprite_option, node_text_option, mat2d, mat3d, node_color, node_state) in &mut query {

        // Normalize the active state weights
        let mut total_weight = 0.0;
        for (state, _) in &node_color.colors {
            if let Some(weight) = node_state.states.get(state) {
                total_weight += weight;
            }
        }

        // Combine the color into one normalized
        let mut blend_color = Hsla::new(0.0, 0.0, 0.0, 0.0);

        // If no state active just try to use base color
        if total_weight == 0.0 {
            if let Some(color) = node_color.colors.get(&UiBase::id()) {
                blend_color = (*color).into();
            }

        // Blend colors from active states
        } else {
            for (state, color) in &node_color.colors {
                if let Some(weight) = node_state.states.get(state) {
                    let converted: Hsla = (*color).into();

                    if blend_color.alpha == 0.0 {
                        blend_color.hue = converted.hue;
                    } else {
                        blend_color.hue = lerp_hue(blend_color.hue, converted.hue, weight / total_weight);
                    }

                    //blend_color.hue += converted.hue * (weight / total_weight);
                    blend_color.saturation += converted.saturation * (weight / total_weight);
                    blend_color.lightness += converted.lightness * (weight / total_weight);
                    blend_color.alpha += converted.alpha * (weight / total_weight);
                }
            }
        }

        // Apply the color to attached components
        if let Some(mut sprite) = node_sprite_option {
            sprite.color = blend_color.into();
        }
        if let Some(mut text) = node_text_option {
            **text = blend_color.into();
        }
        if let Some(id) = mat2d {
            if let Some(mat) = materials2d.get_mut(id) {
                mat.color = blend_color.into();
            }
        } else if let Some(id) = mat3d {
            if let Some(mat) = materials3d.get_mut(id) {
                mat.base_color = blend_color.into();
            }
        }
    }
}

fn lerp_hue(h1: f32, h2: f32, t: f32) -> f32 {
    let diff = (h2 - h1 + 540.0) % 360.0 - 180.0; // Ensure shortest direction
    (h1 + diff * t + 360.0) % 360.0
}


// #===============================#
// #=== THE LUNEX SETS & GROUPS ===#

/// System set for [`UiLunexPlugins`]
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UiSystems {
    /// Systems that modify data pre-computation
    PreCompute,
    /// The computation
    Compute,
    /// Systems that modify data post-computation
    PostCompute,
}

/// Gizmo group for UI 2D node debug outlines
#[derive(GizmoConfigGroup, Default, Reflect, Clone, Debug)]
pub struct LunexGizmoGroup2d;

/// Gizmo group for UI 3D node debug outlines
#[derive(GizmoConfigGroup, Default, Reflect, Clone, Debug)]
pub struct LunexGizmoGroup3d;


// #=========================#
// #=== THE LUNEX PLUGINS ===#

/// This plugin is used for the main logic.
#[derive(Debug, Default, Clone)]
pub struct UiLunexPlugin;
impl Plugin for UiLunexPlugin {
    fn build(&self, app: &mut App) {

        // Configure the system set
        app.configure_sets(PostUpdate, (
            UiSystems::PreCompute.before(UiSystems::Compute),
            UiSystems::PostCompute.after(UiSystems::Compute).before(bevy::transform::TransformSystem::TransformPropagate),
        ));

        // Add observers
        app.add_observer(observer_touch_layout_root);

        // PRE-COMPUTE SYSTEMS
        app.add_systems(PostUpdate, (

            system_state_base_balancer,
            system_text_size_to_layout.after(bevy::text::update_text2d_layout),
            system_image_size_to_layout,
            system_recompute_on_change::<UiLayout>,

        ).chain().in_set(UiSystems::PreCompute));

        #[cfg(feature = "text3d")]
        app.add_systems(PostUpdate,
            system_text_3d_size_to_layout
                .after(bevy_rich_text3d::Text3dSet)
                .in_set(UiSystems::PreCompute)
        );


        // COMPUTE SYSTEMS
        app.add_systems(PostUpdate, (

            system_layout_compute,

        ).in_set(UiSystems::Compute));


        // POST-COMPUTE SYSTEMS
        app.add_systems(PostUpdate, (

            system_color,
            system_mark_3d,
            system_pipe_sprite_size_from_dimension.before(bevy::sprite::SpriteSystem::ComputeSlices),
            system_text_size_from_dimension,
            system_mesh_3d_reconstruct_from_dimension,
            system_mesh_2d_reconstruct_from_dimension,
            system_embedd_resize,

        ).in_set(UiSystems::PostCompute));

        #[cfg(feature = "text3d")]
        app.add_systems(PostUpdate,
            system_text_3d_size_from_dimension
                .in_set(UiSystems::PostCompute)
        );


        // Add index plugins
        app.add_plugins((
            CursorPlugin,
            UiLunexStatePlugin,
            UiLunexPickingPlugin,
            UiLunexIndexPlugin::<0>,
            UiLunexIndexPlugin::<1>,
            UiLunexIndexPlugin::<2>,
            UiLunexIndexPlugin::<3>,
        ));
    }
}


/// This plugin is used to enable debug functionality.
#[derive(Debug, Default, Clone)]
pub struct UiLunexDebugPlugin<const GIZMO_2D_LAYER: usize = 0, const GIZMO_3D_LAYER: usize = 0>;
impl <const GIZMO_2D_LAYER: usize, const GIZMO_3D_LAYER: usize> Plugin for UiLunexDebugPlugin<GIZMO_2D_LAYER, GIZMO_3D_LAYER> {
    fn build(&self, app: &mut App) {

        // Configure the Gizmo render groups
        app .init_gizmo_group::<LunexGizmoGroup2d>()
            .init_gizmo_group::<LunexGizmoGroup3d>()
            .add_systems(Startup, |mut config_store: ResMut<GizmoConfigStore>| {
                let (my_config, _) = config_store.config_mut::<LunexGizmoGroup2d>();
                my_config.render_layers = RenderLayers::layer(GIZMO_2D_LAYER);

                let (my_config, _) = config_store.config_mut::<LunexGizmoGroup3d>();
                my_config.render_layers = RenderLayers::layer(GIZMO_3D_LAYER);
            });

        // Add the 2d and 3d gizmo outlines
        app.add_systems(PostUpdate, (
            system_debug_draw_gizmo_2d,
            system_debug_draw_gizmo_3d,
        ));

        // Add the debug tree printing
        app.add_systems(PostUpdate, (
            system_debug_print_data,
        ).in_set(UiSystems::PostCompute));
    }
}


/// This plugin is used to register index components.
#[derive(Debug, Default, Clone)]
pub struct UiLunexIndexPlugin<const INDEX: usize>;
impl <const INDEX: usize> Plugin for UiLunexIndexPlugin<INDEX> {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (
            system_fetch_dimension_from_camera::<INDEX>,
            system_touch_camera_if_fetch_added::<INDEX>,
        ).in_set(UiSystems::PreCompute));
    }
}


/// Plugin group adding all necessary plugins for Lunex
pub struct UiLunexPlugins;
impl PluginGroup for UiLunexPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        // Add text 3d plugin
        #[cfg(feature = "text3d")] {
            builder = builder.add(Text3dPlugin {
                load_system_fonts: true,
                ..Default::default()
            });
        }

        // Add Lunex plugin
        builder = builder.add(UiLunexPlugin);

        // Return the plugin group
        builder
    }
}
