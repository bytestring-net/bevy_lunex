#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy::sprite::Anchor;


// #=================================#
// #=== THE MAIN LUNEX COMPONENTS ===#

/// **Dimension** - This component holds width and height used for different Ui components
#[derive(Component, Deref, DerefMut, Default, Clone, PartialEq, Debug)]
pub struct Dimension(pub Vec2);
impl <T: Into<Vec2>> From<T> for Dimension {
    fn from(value: T) -> Self {
        Dimension(value.into())
    }
}


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
///         UiLayoutRoot,
///         UiFetchFromCamera::<0>, // Pipe the size from Camera
///     )).with_children(|ui| {
///         // ... spawn your Ui Here
///     });
/// # }
/// ```
#[derive(Component)]
#[require(Visibility, Transform, Dimension)]
pub struct UiLayoutRoot;


/// **Ui Layout** - This component specifies the layout of a Ui-Node, which must be spawned as a child
/// of either [`UiLayoutRoot`] or [`UiLayout`] to work. Based on the provided layout other attached
/// components on this entity are overwritten to match the computed structure.
/// 
/// Direct output components:
/// - [`Transform`] - The computed position of the Ui-Node _(Read-only)_
/// - [`Dimension`] - The computed size of the Ui-Node _(Read-only)_
/// 
/// Indirectly affected components:
/// - [`Sprite`] - `custom_size` to match [`Dimension`]
/// 
/// ## 🛠️ Example
/// ```
/// # use bevy::prelude::*;
/// # use bevy_lunex::*;
/// # fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
/// # commands.spawn((
/// #     UiLayoutRoot,
/// #     UiFetchFromCamera::<0>, // Pipe the size from Camera
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
#[derive(Component)]
#[require(Visibility, Transform, Dimension)]
pub struct UiLayout {
    layouts: Vec<UiLayoutType>
}
impl UiLayout {
    /// **Boundary** - Declarative layout type that is defined by its top-left corner and bottom-right corner.
    /// Nodes with this layout are not included in the ui flow.
    /// ## 🛠️ Example
    /// ```
    /// # use bevy_lunex::{UiLayout, Rl};
    /// let layout: UiLayout = UiLayout::Boundary().pos1(Rl(20.0)).pos2(Rl(80.0)).pack();
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
}
impl From<UiLayoutType> for UiLayout {
    fn from(value: UiLayoutType) -> Self {
        UiLayout {
            layouts: vec![value],
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


/// **Ui Fetch From Camera** - Attaching this component to [`UiLayoutRoot`] will make the [`Dimension`]
/// component pull data from a [`Camera`] with attached [`UiSourceCamera`] with the same index.
#[derive(Component, Clone, PartialEq, Debug)]
pub struct UiFetchFromCamera<const INDEX: usize>;

/// **Ui Source Camera** - Marks a [`Camera`] as a source for [`UiLayoutRoot`] with [`UiFetchFromCamera`].
/// They must have the same index and only one [`UiSourceCamera`] can exist for a single index.
#[derive(Component, Clone, PartialEq, Debug)]
pub struct UiSourceCamera<const INDEX: usize>;


// #==============================#
// #=== THE MAIN LUNEX SYSTEMS ===#

/// This system takes [`Camera`] viewport data and pipes them into querried [`Dimension`] + [`UiLayoutRoot`] + [`UiFetchFromCamera`].
pub fn fetch_dimension_from_camera<const INDEX: usize>(
    src_query: Query<(&Camera, Option<&OrthographicProjection>), (With<UiSourceCamera<INDEX>>, Changed<Camera>)>,
    mut dst_query: Query<&mut Dimension, (With<UiLayoutRoot>, With<UiFetchFromCamera<INDEX>>)>,
) {
    // Check if we have a camera dimension input
    if src_query.is_empty() { return; }
    let Ok((camera, projection_option)) = src_query.get_single() else {
        warn_once!("Multiple UiSourceCamera<{INDEX}> exist at once! Ignoring all camera inputs to avoid unexpected behavior!");
        return;
    };

    // Pipe the camera viewport size
    if let Some(cam_size) = camera.physical_viewport_size() {
        for mut size in &mut dst_query {
            **size = Vec2::from((cam_size.x as f32, cam_size.y as f32)) * if let Some(p) = projection_option { p.scale } else { 1.0 };
        }
    }
}

/// This system listens for added [`UiFetchFromCamera`] components and if it finds one, mutable accesses all [`Camera`]s to trigger fetching systems.
pub fn touch_camera_if_fetch_added<const INDEX: usize>(
    query: Query<Entity, Added<UiFetchFromCamera<INDEX>>>,
    mut cameras: Query<&mut Camera, With<UiSourceCamera<INDEX>>>,
){
    if !query.is_empty() {
        for mut camera in &mut cameras {
            camera.as_mut();
        }
    }
}

/// This system draws the outlines of [`UiLayout`] and [`UiLayoutRoot`] as gizmos.
pub fn debug_draw_gizmo<G:GizmoConfigGroup>(
    query: Query<(&GlobalTransform, &Dimension), Or<(With<UiLayout>, With<UiLayoutRoot>)>>,
    mut gizmos: Gizmos<G>
) {
    for (transform, dimension) in &query {
        // Draw the gizmo outline
        gizmos.rect(
            Isometry3d::from(transform.translation()),
            **dimension,
            Color::linear_rgb(0.0, 1.0, 0.0),
        );
    }
}

/// This system traverses the hierarchy and computes all nodes.
pub fn compute_children(
    root_query: Query<(&UiLayoutRoot, &Transform, &Dimension, &Children), (Without<UiLayout>, Changed<Dimension>)>,
    mut node_query: Query<(&UiLayout, &mut Transform, &mut Dimension, Option<&Children>), Without<UiLayoutRoot>>,
) {
    for (_, root_transform, root_dimension, root_children) in &root_query {
        // Size of the viewport
        let root_rectangle = Rectangle2D {
            pos: root_transform.translation.xy(),
            size: **root_dimension,
        };

        // Stack-based traversal
        let mut stack: Vec<(Entity, Rectangle2D, usize)> = root_children
            .iter()
            .map(|&child| (child, root_rectangle, 1))
            .collect();

        while let Some((current_entity, parent_rectangle, depth)) = stack.pop() {
            if let Ok((node_layout, mut node_transform, mut node_dimension, node_children_option)) = node_query.get_mut(current_entity) {
                // Compute all layouts for the node
                let mut computed_rectangles = Vec::with_capacity(node_layout.layouts.len());
                for layout in &node_layout.layouts {
                    computed_rectangles.push(layout.compute(&parent_rectangle, 1.0, root_rectangle.size, 16.0));
                }

                // Save the computed layout
                node_transform.translation.x = computed_rectangles[0].pos.x;
                node_transform.translation.y = -computed_rectangles[0].pos.y;
                node_transform.translation.z = depth as f32;
                **node_dimension = computed_rectangles[0].size;

                if let Some(node_children) = node_children_option {
                    let child_parent_rectangle = Rectangle2D {
                        pos: node_transform.translation.xy(),
                        size: **node_dimension,
                    };

                    // Add children to the stack
                    stack.extend(node_children.iter().map(|&child| (child, child_parent_rectangle, depth + 1)));
                }
            }
        }
    }
}

/// This system takes [`Dimension`] data and pipes them into querried [`Sprite`].
pub fn element_sprite_size_from_dimension(
    mut query: Query<(&mut Sprite, &Dimension), Changed<Dimension>>,
) {
    for (mut sprite, dimension) in &mut query {
        sprite.custom_size = Some(**dimension)
    }
}


// #=============================#
// #=== THE MAIN LUNEX PLUGIN ===#

/// This plugin is used for the main logic.
pub struct UiLunexPlugin;
impl Plugin for UiLunexPlugin {
    fn build(&self, app: &mut App) {

        app.add_systems(Update, (
            compute_children,
            element_sprite_size_from_dimension,
        ));

        app.add_plugins((
            UiLunexIndexPlugin::<0>,
            UiLunexIndexPlugin::<1>,
            UiLunexIndexPlugin::<2>,
            UiLunexIndexPlugin::<3>,
        ));
    }
}

/// This plugin is used to enable debug functionality.
pub struct UiLunexDebugPlugin;
impl Plugin for UiLunexDebugPlugin {
    fn build(&self, app: &mut App) {

        app.add_systems(Update, (
            debug_draw_gizmo::<DefaultGizmoConfigGroup>,
        ));
    }
}

/// This plugin is used to register index components.
pub struct UiLunexIndexPlugin<const INDEX: usize>;
impl <const INDEX: usize> Plugin for UiLunexIndexPlugin<INDEX> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            fetch_dimension_from_camera::<INDEX>,
            touch_camera_if_fetch_added::<INDEX>,
        ));
    }
}

// #============================#
// #=== MULTIPURPOSE STRUCTS ===#

/// **Rectangle 2D** - Contains computed values from node layout.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rectangle2D {
    pub pos : Vec2,
    pub size: Vec2,
}
impl Rectangle2D {
    pub fn lerp(self, rhs: Self, lerp: f32) -> Self {
        Rectangle2D {
            pos: self.pos.lerp(rhs.pos, lerp),
            size: self.size.lerp(rhs.size, lerp),
        }
    }
}
impl Rectangle2D {
    /// A new empty [`Rectangle2D`]. Has `0` size. 
    pub const EMPTY: Rectangle2D = Rectangle2D { pos : Vec2::ZERO, size: Vec2::ZERO };
    /// Creates new empty Window layout.
    pub const fn new() -> Self {
        Rectangle2D::EMPTY
    }
    /// Replaces the position with the new value.
    pub fn with_pos(mut self, pos: impl Into<Vec2>) -> Self {
        self.pos = pos.into();
        self
    }
    /// Replaces the x position with the new value.
    pub fn with_x(mut self, width: f32) -> Self {
        self.pos.x = width;
        self
    }
    /// Replaces the y position with the new value.
    pub fn with_y(mut self, height: f32) -> Self {
        self.pos.y = height;
        self
    }
    /// Replaces the size with the new value.
    pub fn with_size(mut self, size: impl Into<Vec2>) -> Self {
        self.size = size.into();
        self
    }
    /// Replaces the width with the new value.
    pub fn with_width(mut self, width: f32) -> Self {
        self.size.x = width;
        self
    }
    /// Replaces the height with the new value.
    pub fn with_height(mut self, height: f32) -> Self {
        self.size.y = height;
        self
    }    
}

/// **Align** - A type used to define alignment in a node layout.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::Align;
/// let align: Align = Align::START; // -> -1.0
/// let align: Align = Align(-1.0);  // -> -1.0
/// let align: Align = (-1.0).into();  // -> -1.0
/// ```
/// The expected range is `-1.0` to `1.0`, but you can extrapolate.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct Align (pub f32);
impl Align {
    pub const START: Align = Align(-1.0);
    pub const LEFT: Align = Align(-1.0);
    pub const CENTER: Align = Align(0.0);
    pub const MIDDLE: Align = Align(0.0);
    pub const END: Align = Align(1.0);
    pub const RIGHT: Align = Align(1.0);
}
impl From<f32> for Align {
    fn from(val: f32) -> Self {
        Align(val)
    }
}


/// **Scaling** - A type used to define how should a Solid node layout scale relative to a parent.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::Scaling;
/// let scaling: Scaling = Scaling::HorFill; // -> always cover the horizontal axis
/// let scaling: Scaling = Scaling::VerFill; // -> always cover the vertical axis
/// let scaling: Scaling = Scaling::Fit;  // -> always fit inside
/// let scaling: Scaling = Scaling::Fill; // -> always cover all
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum Scaling {
    /// Node layout should always cover the horizontal axis of the parent node.
    HorFill,
    /// Node layout should always cover the vertical axis of the parent node.
    VerFill,
    /// Node layout should always fit inside the parent node.
    #[default] Fit,
    /// Node layout should always cover all of the parent node.
    Fill,
}


// #====================#
// #=== LAYOUT TYPES ===#

/// **Ui Layout Type** - Enum holding all UI layout variants.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum UiLayoutType {
    Boundary(UiLayoutTypeBoundary),
    Window(UiLayoutTypeWindow),
    Solid(UiLayoutTypeSolid),
}
impl UiLayoutType {
    /// Computes the layout based on given parameters.
    pub(crate) fn compute(&self, parent: &Rectangle2D, absolute_scale: f32, viewport_size: Vec2, font_size: f32) -> Rectangle2D {
        match self {
            UiLayoutType::Boundary(layout) => layout.compute(&parent, absolute_scale, viewport_size, font_size),
            UiLayoutType::Window(layout) => layout.compute(&parent, absolute_scale, viewport_size, font_size),
            UiLayoutType::Solid(layout) => layout.compute(&parent, absolute_scale, viewport_size, font_size),
        }
    }
}
impl From<UiLayoutTypeBoundary> for UiLayoutType {
    fn from(value: UiLayoutTypeBoundary) -> Self {
        UiLayoutType::Boundary(value)
    }
}
impl From<UiLayoutTypeWindow> for UiLayoutType {
    fn from(value: UiLayoutTypeWindow) -> Self {
        UiLayoutType::Window(value)
    }
}
impl From<UiLayoutTypeSolid> for UiLayoutType {
    fn from(value: UiLayoutTypeSolid) -> Self {
        UiLayoutType::Solid(value)
    }
}


/// **Boundary** - Declarative layout type that is defined by its top-left corner and bottom-right corner.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct UiLayoutTypeBoundary {
    /// Position of the top-left corner.
    pub pos1: UiValue<Vec2>,
    /// Position of the bottom-right corner.
    pub pos2: UiValue<Vec2>,
}
impl UiLayoutTypeBoundary {
    /// Creates new empty Boundary node layout.
    pub const fn new() -> Self {
        Self {
            pos1: UiValue::new(),
            pos2: UiValue::new(),
        }
    }
    /// Replaces the position of the top-left corner with a new value.
    pub fn pos1(mut self, pos: impl Into<UiValue<Vec2>>) -> Self {
        self.pos1 = pos.into();
        self
    }
    /// Replaces the position of the bottom-right corner with a new value.
    pub fn pos2(mut self, pos: impl Into<UiValue<Vec2>>) -> Self {
        self.pos2 = pos.into();
        self
    }
    /// Replaces the x position of the top-left corner with a new value.
    pub fn x1(mut self, x: impl Into<UiValue<f32>>) -> Self {
        self.pos1.set_x(x);
        self
    }
    /// Replaces the y position of the top-left corner with a new value.
    pub fn y1(mut self, y: impl Into<UiValue<f32>>) -> Self {
        self.pos1.set_y(y);
        self
    }
    /// Replaces the x position of the bottom-right corner with a new value.
    pub fn x2(mut self, x: impl Into<UiValue<f32>>) -> Self {
        self.pos2.set_x(x);
        self
    }
    /// Replaces the y position of the bottom-right corner with a new value.
    pub fn y2(mut self, y: impl Into<UiValue<f32>>) -> Self {
        self.pos2.set_y(y);
        self
    }
    /// Sets the position of the top-left corner to a new value.
    pub fn set_pos1(&mut self, pos: impl Into<UiValue<Vec2>>) {
        self.pos1 = pos.into();
    }
    /// Sets the position of the bottom-right corner to a new value.
    pub fn set_pos2(&mut self, pos: impl Into<UiValue<Vec2>>) {
        self.pos2 = pos.into();
    }
    /// Sets the x position of the top-left corner to a new value.
    pub fn set_x1(&mut self, x: impl Into<UiValue<f32>>) {
        self.pos1.set_x(x);
    }
    /// Sets the y position of the top-left corner to a new value.
    pub fn set_y1(&mut self, y: impl Into<UiValue<f32>>) {
        self.pos1.set_y(y);
    }
    /// Sets the x position of the bottom-right corner to a new value.
    pub fn set_x2(&mut self, x: impl Into<UiValue<f32>>) {
        self.pos2.set_x(x);
    }
    /// Sets the y position of the bottom-right corner to a new value.
    pub fn set_y2(&mut self, y: impl Into<UiValue<f32>>) {
        self.pos2.set_y(y);
    }
    /// Pack the layout type into UiLayout
    pub fn pack(self) -> UiLayout {
        UiLayout::from(self)
    }
    /// Computes the layout based on given parameters.
    pub(crate) fn compute(&self, parent: &Rectangle2D, absolute_scale: f32, viewport_size: Vec2, font_size: f32) -> Rectangle2D {
        let pos1 = self.pos1.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size));
        let pos2 = self.pos2.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size));
        Rectangle2D {
            pos: pos1,
            size: pos2 - pos1,
        }
    }
}

/// **Window** - Declarative layout type that is defined by its size and position.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct UiLayoutTypeWindow {
    /// Position of the node.
    pub pos : UiValue<Vec2>,
    /// Decides where position should be applied at.
    pub anchor: Anchor,
    /// Size of the node layout.
    pub size: UiValue<Vec2>,
}
impl UiLayoutTypeWindow {
    /// Creates new empty Window node layout.
    pub const fn new() -> Self {
        Self {
            pos: UiValue::new(),
            anchor: Anchor::TopLeft,
            size: UiValue::new(),
        }
    }
    /// Replaces the size to make the window fully cover the parent.
    pub fn full(self) -> Self {
        self.size(Rl(100.0))
    }
    /// Replaces the position with a new value.
    pub fn pos(mut self, pos: impl Into<UiValue<Vec2>>) -> Self {
        self.pos = pos.into();
        self
    }
    /// Replaces the x position with a new value.
    pub fn x(mut self, x: impl Into<UiValue<f32>>) -> Self {
        self.pos.set_x(x);
        self
    }
    /// Replaces the y position with a new value.
    pub fn y(mut self, y: impl Into<UiValue<f32>>) -> Self {
        self.pos.set_y(y);
        self
    }
    /// Replaces the size with a new value.
    pub fn size(mut self, size: impl Into<UiValue<Vec2>>) -> Self {
        self.size = size.into();
        self
    }
    /// Replaces the width with a new value.
    pub fn width(mut self, width: impl Into<UiValue<f32>>) -> Self {
        self.size.set_x(width);
        self
    }
    /// Replaces the height with a new value.
    pub fn height(mut self, height: impl Into<UiValue<f32>>) -> Self {
        self.size.set_y(height);
        self
    }
    /// Replaces the anchor with a new value.
    pub fn anchor(mut self, anchor: impl Into<Anchor>) -> Self {
        self.anchor = anchor.into();
        self
    }
    /// Sets the position to a new value.
    pub fn set_pos(&mut self, pos: impl Into<UiValue<Vec2>>){
        self.pos = pos.into();
    }
    /// Sets the x position to a new value.
    pub fn set_x(&mut self, x: impl Into<UiValue<f32>>){
        self.pos.set_x(x);
    }
    /// Sets the y position to a new value.
    pub fn set_y(&mut self, y: impl Into<UiValue<f32>>){
        self.pos.set_y(y);
    }
    /// Sets the size to a new value.
    pub fn set_size(&mut self, size: impl Into<UiValue<Vec2>>){
        self.size = size.into();
    }
    /// Sets the width to a new value.
    pub fn set_width(&mut self, width: impl Into<UiValue<f32>>){
        self.size.set_x(width);
    }
    /// Sets the height to a new value.
    pub fn set_height(&mut self, height: impl Into<UiValue<f32>>){
        self.size.set_y(height);
    }
    /// Sets the anchor to a new value.
    pub fn set_anchor(&mut self, anchor: impl Into<Anchor>){
        self.anchor = anchor.into();
    }
    /// Pack the layout type into UiLayout
    pub fn pack(self) -> UiLayout {
        UiLayout::from(self)
    }
    /// Computes the layout based on given parameters.
    pub(crate) fn compute(&self, parent: &Rectangle2D, absolute_scale: f32, viewport_size: Vec2, font_size: f32) -> Rectangle2D {
        let pos = self.pos.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size));
        let size = self.size.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size));
        let mut anchor = self.anchor.as_vec();
        anchor.y *= -1.0;
        Rectangle2D {
            pos: -parent.size / 2.0 + pos - size * (anchor),
            size,
        }
    }
}

/// **Solid** - Declarative layout type that is defined by its width and height ratio.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct UiLayoutTypeSolid {
    /// Aspect ratio of the width and height. `1:1 == 10:10 == 100:100`.
    pub size: UiValue<Vec2>,
    /// Horizontal alignment within parent.
    pub align_x: Align,
    /// Vertical alignment within parent.
    pub align_y: Align,
    /// Specifies container scaling.
    pub scaling: Scaling,
}
impl UiLayoutTypeSolid {
    /// Creates new empty Solid node layout.
    pub fn new() -> Self {
        Self {
            size: Ab(Vec2::ONE).into(),
            align_x: Align::CENTER,
            align_y: Align::CENTER,
            scaling: Scaling::Fit,
        }
    }
    /// Replaces the size with a new value.
    pub fn size(mut self, size: impl Into<UiValue<Vec2>>) -> Self {
        self.size = size.into();
        self
    }
    /// Replaces the width with a new value.
    pub fn width(mut self, width: impl Into<UiValue<f32>>) -> Self {
        self.size.set_x(width);
        self
    }
    /// Replaces the height with a new value.
    pub fn height(mut self, height: impl Into<UiValue<f32>>) -> Self {
        self.size.set_y(height);
        self
    }
    /// Replaces the x alignment with a new value.
    pub fn align_x(mut self, align: impl Into<Align>) -> Self {
        self.align_x = align.into();
        self
    }
    /// Replaces the y alignment with a new value.
    pub fn align_y(mut self, align: impl Into<Align>) -> Self {
        self.align_y = align.into();
        self
    }
    /// Replaces the scaling mode with a new value.
    pub fn scaling(mut self, scaling: Scaling) -> Self {
        self.scaling = scaling;
        self
    }
    /// Sets the size to a new value.
    pub fn set_size(&mut self, size: impl Into<UiValue<Vec2>>) {
        self.size = size.into();
    }
    /// Sets the width to a new value.
    pub fn set_width(&mut self, width: impl Into<UiValue<f32>>) {
        self.size.set_x(width);
    }
    /// Sets the height to a new value.
    pub fn set_height(&mut self, height: impl Into<UiValue<f32>>) {
        self.size.set_y(height);
    }
    /// Sets the x alignment to a new value.
    pub fn set_align_x(&mut self, align: impl Into<Align>) {
        self.align_x = align.into();
    }
    /// Sets the y alignment to a new value.
    pub fn set_align_y(&mut self, align: impl Into<Align>) {
        self.align_y = align.into();
    }
    /// Sets the scaling mode to a new value.
    pub fn set_scaling(&mut self, scaling: Scaling) {
        self.scaling = scaling;
    }
    /// Pack the layout type into UiLayout
    pub fn pack(self) -> UiLayout {
        UiLayout::from(self)
    }
    /// Computes the layout based on given parameters.
    pub(crate) fn compute(&self, parent: &Rectangle2D, absolute_scale: f32, viewport_size: Vec2, font_size: f32) -> Rectangle2D {
        
        let size = self.size.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size));

        let scale = match self.scaling {
            Scaling::HorFill => parent.size.x / size.x,
            Scaling::VerFill => parent.size.y / size.y,
            Scaling::Fit => f32::min(parent.size.x / size.x, parent.size.y / size.y),
            Scaling::Fill => f32::max(parent.size.x / size.x, parent.size.y / size.y),
        };

        let center_point = parent.size / 2.0;

        let computed_width = size.x * scale;
        let computed_height = size.y * scale;
        let computed_point = Vec2::new(center_point.x - computed_width / 2.0, center_point.y - computed_height / 2.0);

        Rectangle2D {
            pos: Vec2::new(
                computed_point.x * self.align_x.0,
                computed_point.y * self.align_y.0,
            ),
            size: (computed_width, computed_height).into(),
        }
    }
}


// #=========================#
// #=== THE UI UNIT TYPES ===#

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;

/// **Absolute** - Represents non-changing unit. Scale can be modified but by default `1Ab = 1Px`.
/// ## 🛠️ Example
/// ```
/// # use crate::Ab;
/// let a: Ab<f32> = Ab(4.0) + Ab(6.0); // -> 10px
/// let b: Ab<f32> = Ab(4.0) * 2.0;     // -> 8px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Ab<T>(pub T);

/// **Relative** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// ## 🛠️ Example
/// ```
/// # use crate::Rl;
/// let a: Rl<f32> = Rl(25.0) + Rl(40.0); // -> 65%
/// let b: Rl<f32> = Rl(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Rl<T>(pub T);

/// **Relative width** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// Proportional to a width measure even when used in a height field.
/// ## 🛠️ Example
/// ```
/// # use crate::Rw;
/// let a: Rw<f32> = Rw(25.0) + Rw(40.0); // -> 65%
/// let b: Rw<f32> = Rw(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Rw<T>(pub T);

/// **Relative height** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// Proportional to a height measure even when used in a width field.
/// ## 🛠️ Example
/// ```
/// # use crate::Rh;
/// let a: Rh<f32> = Rh(25.0) + Rh(40.0); // -> 65%
/// let b: Rh<f32> = Rh(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Rh<T>(pub T);

/// **Size of M** - Represents unit that is the size of the symbol `M`. Which is `16px` with `font size 16px` and so on.
/// ## 🛠️ Example
/// ```
/// # use crate::Em;
/// let a: Em<f32> = Em(1.0) + Em(2.0); // -> 3em == 48px with font size 16px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Em<T>(pub T);

/// **Viewport** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// ## 🛠️ Example
/// ```
/// # use crate::Vp;
/// let a: Vp<f32> = Vp(25.0) + Vp(40.0); // -> 65%
/// let b: Vp<f32> = Vp(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Vp<T>(pub T);

/// **Viewport width** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// Proportional to a width measure even when used in a height field.
/// ## 🛠️ Example
/// ```
/// # use crate::Vw;
/// let a: Vw<f32> = Vw(25.0) + Vw(40.0); // -> 65%
/// let b: Vw<f32> = Vw(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Vw<T>(pub T);

/// **Viewport Height** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// Proportional to a height measure even when used in a width field.
/// ## 🛠️ Example
/// ```
/// # use crate::Vh;
/// let a: Vh<f32> = Vh(25.0) + Vh(40.0); // -> 65%
/// let b: Vh<f32> = Vh(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Vh<T>(pub T);


/// Implement basic math and conversions for a type
macro_rules! init_uiunit {
    ($($unit:ident), *) => {
        $(
            // Implement negation of the same type
            impl <T: Neg<Output = T>> Neg for $unit<T> {
                type Output = Self;
                fn neg(self) -> Self::Output {
                    $unit(-self.0)
                }
            }
            
            // Implement addition of the same type
            impl <T: Add<Output = T>> Add for $unit<T> {
                type Output = Self;
                fn add(self, other: Self) -> Self::Output {
                    $unit(self.0 + other.0)
                }
            }
            impl <T: AddAssign<T>> AddAssign for $unit<T> {
                fn add_assign(&mut self, rhs: Self) {
                    self.0 += rhs.0
                }
            }
            
            // Implement subtraction of the same type
            impl <T: Sub<Output = T>> Sub for $unit<T> {
                type Output = Self;
                fn sub(self, other: Self) -> Self::Output {
                    $unit(self.0 - other.0)
                }
            }
            impl <T: SubAssign<T>> SubAssign for $unit<T> {
                fn sub_assign(&mut self, rhs: Self) {
                    self.0 -= rhs.0
                }
            }
            
            // Implement multiplication of the same type
            impl <T: Mul<Output = T>> Mul for $unit<T> {
                type Output = Self;
                fn mul(self, other: Self) -> Self::Output {
                    $unit(self.0 * other.0)
                }
            }
            impl <T: MulAssign<T>> MulAssign for $unit<T> {
                fn mul_assign(&mut self, rhs: Self) {
                    self.0 *= rhs.0
                }
            }
            
            // Implement multiplication with the f32 type
            impl <T: Mul<f32, Output = T>> Mul<f32> for $unit<T> {
                type Output = $unit<T>;
                fn mul(self, rhs: f32) -> Self::Output {
                    $unit(self.0 * rhs)
                }
            }
            impl <T: MulAssign<f32>> MulAssign<f32> for $unit<T> {
                fn mul_assign(&mut self, rhs: f32) {
                    self.0 *= rhs
                }
            }
        )*
    };
}
init_uiunit!(Ab, Rl, Rw, Rh, Em, Vp, Vw, Vh);


// #===================================#
// #=== THE UI VALUE IMPLEMENTATION ===#

/// Declare [`UiValue`] struct with these fields
macro_rules! init_uivalue {
    ($($struct_field:ident), *) => {
        /// **Ui value** - A collection of different units used for UI.
        /// They are computed at runtime when layout is being calculated (context-aware).
        /// The supported units that implement `Into<UiValue>` are:
        /// * [`Ab`] [`Rl`] [`Rw`] [`Rh`] [`Em`] [`Sp`] [`Vw`] [`Vh`]
        /// ## 📦 Types
        /// First class implementations for `(T)` are:
        /// * [`f32`] [`Vec2`] [`Vec3`] [`Vec4`]
        /// ## 🛠️ Example
        /// ```
        /// # use lunex_engine::{UiValue, Ab, Em, Rl, Sp};
        /// # use bevy::prelude::Vec2;
        /// let a: UiValue<f32> = Ab(4.0) + Em(1.0);  // -> 4px + 1em
        /// let b: UiValue<f32> = Ab(40.0) - Rl(5.0); // -> 40px - 5%
        /// let c: UiValue<Vec2> = (Ab(20.0), Em(2.0)).into(); // -> [20px, 2em]
        /// ```
        #[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
        pub struct UiValue<T> {
            $(
                $struct_field: Option<T>,
            )*
        }
        impl <T> UiValue<T> {
            /// Creates new empty [`UiValue`]
            pub const fn new() -> Self {
                UiValue {
                    $(
                        $struct_field: None,
                    )*
                }
            }
        }
        
        // Implement negation of the same type
        impl <T: Neg<Output = T>> Neg for UiValue<T> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                UiValue {
                    $(
                        $struct_field: if let Some(v) = self.$struct_field { Some(-v) } else { None },
                    )*
                }
            }
        }
        
        // Implement addition of the same type
        impl <T: Add<Output = T> + Add> Add for UiValue<T> {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                UiValue {
                    $(
                        $struct_field: if let Some(v1) = self.$struct_field {
                            if let Some(v2) = other.$struct_field { Some(v1 + v2) } else { Some(v1) }
                        } else { other.$struct_field },
                    )*
                }
            }
        }
        impl <T: Add<Output = T> + Copy> AddAssign for UiValue<T> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs
            }
        }

        // Implement subtraction of the same type
        impl <T: Sub<Output = T> + Sub + Neg<Output = T>> Sub for UiValue<T> {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                UiValue {
                    $(
                        $struct_field: if let Some(v1) = self.$struct_field {
                            if let Some(v2) = other.$struct_field { Some(v1 - v2) } else { Some(v1) }
                        } else { other.$struct_field },
                    )*
                }
            }
        }
        impl <T: Sub<Output = T> + Copy + Neg<Output = T>> SubAssign for UiValue<T> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs
            }
        }

        // Implement multiplication of the same type
        impl <T: Mul<Output = T> + Mul> Mul for UiValue<T> {
            type Output = Self;
            fn mul(self, other: Self) -> Self::Output {
                let mut output = UiValue::new();
                $(
                    if let Some(v1) = self.$struct_field {
                        if let Some(v2) = other.$struct_field {
                            output.$struct_field = Some(v1 * v2);
                        }
                    }
                )*
                output
            }
        }
        impl <T: Mul<Output = T> + Copy> MulAssign for UiValue<T> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs
            }
        }    
        
        // Implement multiplication with the f32 type
        impl <T: Mul<f32, Output = T>> Mul<f32> for UiValue<T> {
            type Output = Self;
            fn mul(self, other: f32) -> Self::Output {
                let mut output = UiValue::new();
                $(
                    if let Some(v1) = self.$struct_field {
                        output.$struct_field = Some(v1 * other);
                    }
                )*
                output
            }
        }
        impl <T: Mul<f32, Output = T> + Copy> MulAssign<f32> for UiValue<T> {
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs
            }
        }
    }
}
init_uivalue!(ab, rl, rw, rh, em, vp, vw, vh);

/// Bind these structs to appropriate [`UiValue`] fields and implement math operations
macro_rules! bind_uivalue {
    ($( ($unit:ident, $struct_field:ident) ),* ) => {

        $(
            // Bind conversion of the type to the field
            impl <T> From<$unit<T>> for UiValue<T> {
                fn from(val: $unit<T>) -> UiValue<T> {
                    let mut ret = UiValue::new();
                    ret.$struct_field = Some(val.0);
                    ret
                }
            }
            
            // Bind addition of the type to the field
            impl <T: Add<Output = T> + Add> Add<$unit<T>> for UiValue<T> {
                type Output = Self;
                fn add(mut self, other: $unit<T>) -> Self::Output {
                    match self.$struct_field {
                        Some(v) => {
                            self.$struct_field = Some(v + other.0);
                            self
                        },
                        None => {
                            self.$struct_field = Some(other.0);
                            self
                        },
                    }
                }
            }
            impl <T: Add<Output = T> + Copy> AddAssign<$unit<T>> for UiValue<T> {
                fn add_assign(&mut self, rhs: $unit<T>) {
                    match self.$struct_field {
                        Some(v) => self.$struct_field = Some(v + rhs.0),
                        None => self.$struct_field = Some(rhs.0),
                    }
                }
            }
            
            // Bind subtraction of the type to the field
            impl <T: Sub<Output = T> + Sub> Sub<$unit<T>> for UiValue<T> {
                type Output = Self;
                fn sub(mut self, other: $unit<T>) -> Self::Output {
                    match self.$struct_field {
                        Some(v) => {
                            self.$struct_field = Some(v - other.0);
                            self
                        },
                        None => {
                            self.$struct_field = Some(other.0);
                            self
                        },
                    }
                }
            }
            impl <T: Sub<Output = T> + Copy> SubAssign<$unit<T>> for UiValue<T> {
                fn sub_assign(&mut self, rhs: $unit<T>) {
                    match self.$struct_field {
                        Some(v) => self.$struct_field = Some(v - rhs.0),
                        None => self.$struct_field = Some(rhs.0),
                    }
                }
            }
            
            // Bind multiplication of the type to the field
            impl <T: Mul<Output = T> + Mul> Mul<$unit<T>> for UiValue<T> {
                type Output = Self;
                fn mul(mut self, other: $unit<T>) -> Self::Output {
                    if let Some(v) = self.$struct_field {
                        self.$struct_field = Some(v * other.0);
                    }
                    self
                }
            }
            impl <T: Mul<Output = T> + Copy> MulAssign<$unit<T>> for UiValue<T> {
                fn mul_assign(&mut self, rhs: $unit<T>) {
                    if let Some(v) = self.$struct_field {
                        self.$struct_field = Some(v * rhs.0);
                    }
                }
            }
        )*

        impl UiValue<Vec2> {
            /// Gets the X value of all units.
            pub fn get_x(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.x) }
                )*
                out
            }
            /// Gets the Y value of all units.
            pub fn get_y(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.y) }
                )*
                out
            }

            /// Replaces the X value of appropriate units with the new value.
            pub fn with_x(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.x = v2 } else { self.$struct_field = Some(Vec2::new(v2, 0.0)) } }
                )*
                self
            }
            /// Replaces the Y value of appropriate units with the new value.
            pub fn with_y(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.y = v2 } else { self.$struct_field = Some(Vec2::new(0.0, v2)) } }
                )*
                self
            }

            /// Sets the X value of appropriate units with the new value.
            pub fn set_x(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.x = v2 } else { self.$struct_field = Some(Vec2::new(v2, 0.0)) } }
                )*
            }
            /// Sets the Y value of appropriate units with the new value.
            pub fn set_y(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.y = v2 } else { self.$struct_field = Some(Vec2::new(0.0, v2)) } }
                )*
            }

        }
        impl UiValue<Vec3> {
            /// Gets the X value of all units.
            pub fn get_x(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.x) }
                )*
                out
            }
            /// Gets the Y value of all units.
            pub fn get_y(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.y) }
                )*
                out
            }
            /// Gets the Z value of all units.
            pub fn get_z(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.z) }
                )*
                out
            }

            /// Replaces the X value of appropriate units with the new value.
            pub fn with_x(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.x = v2 } else { self.$struct_field = Some(Vec3::new(v2, 0.0, 0.0)) } }
                )*
                self
            }
            /// Replaces the Y value of appropriate units with the new value.
            pub fn with_y(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.y = v2 } else { self.$struct_field = Some(Vec3::new(0.0, v2, 0.0)) } }
                )*
                self
            }
            /// Replaces the Z value of appropriate units with the new value.
            pub fn with_z(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.z = v2 } else { self.$struct_field = Some(Vec3::new(0.0, 0.0, v2)) } }
                )*
                self
            }

            /// Sets the X value of appropriate units with the new value.
            pub fn set_x(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.x = v2 } else { self.$struct_field = Some(Vec3::new(v2, 0.0, 0.0)) } }
                )*
            }
            /// Sets the Y value of appropriate units with the new value.
            pub fn set_y(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.y = v2 } else { self.$struct_field = Some(Vec3::new(0.0, v2, 0.0)) } }
                )*
            }
            /// Sets the Z value of appropriate units with the new value.
            pub fn set_z(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.z = v2 } else { self.$struct_field = Some(Vec3::new(0.0, 0.0, v2)) } }
                )*
            }
        }
        impl UiValue<Vec4> {
            /// Gets the X value of all units.
            pub fn get_x(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.x) }
                )*
                out
            }
            /// Gets the Y value of all units.
            pub fn get_y(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.y) }
                )*
                out
            }
            /// Gets the Z value of all units.
            pub fn get_z(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.z) }
                )*
                out
            }
            /// Gets the W value of all units.
            pub fn get_w(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.w) }
                )*
                out
            }

            /// Replaces the X value of appropriate units with the new value.
            pub fn with_x(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.x = v2 } else { self.$struct_field = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
                )*
                self
            }
            /// Replaces the Y value of appropriate units with the new value.
            pub fn with_y(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.y = v2 } else { self.$struct_field = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
                )*
                self
            }
            /// Replaces the Z value of appropriate units with the new value.
            pub fn with_z(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.z = v2 } else { self.$struct_field = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
                )*
                self
            }
            /// Replaces the W value of appropriate units with the new value.
            pub fn with_w(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.w = v2 } else { self.$struct_field = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
                )*
                self
            }
            
            /// Sets the X value of appropriate units with the new value.
            pub fn set_x(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.x = v2 } else { self.$struct_field = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
                )*
            }
            /// Sets the Y value of appropriate units with the new value.
            pub fn set_y(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.y = v2 } else { self.$struct_field = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
                )*
            }
            /// Sets the Z value of appropriate units with the new value.
            pub fn set_z(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.z = v2 } else { self.$struct_field = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
                )*
            }
            /// Sets the W value of appropriate units with the new value.
            pub fn set_w(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.w = v2 } else { self.$struct_field = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
                )*
            }
        }
    }
}
bind_uivalue!((Ab, ab), (Rl, rl), (Rw, rw), (Rh, rh), (Em, em), (Vp, vp), (Vw, vw), (Vh, vh));

/// Implement basic math and conversions for a type
macro_rules! impl_uiunit {
    ($($unit:ident), *) => {
        $(
            impl From<$unit<(f32, f32)>> for UiValue<Vec2> {
                fn from(val: $unit<(f32, f32)>) -> UiValue<Vec2> {
                    $unit(Vec2::new(val.0.0, val.0.1)).into()
                }
            }
            impl From<$unit<(f32, f32, f32)>> for UiValue<Vec3> {
                fn from(val: $unit<(f32, f32, f32)>) -> UiValue<Vec3> {
                    $unit(Vec3::new(val.0.0, val.0.1, val.0.2)).into()
                }
            }
            impl From<$unit<(f32, f32, f32, f32)>> for UiValue<Vec4> {
                fn from(val: $unit<(f32, f32, f32, f32)>) -> UiValue<Vec4> {
                    $unit(Vec4::new(val.0.0, val.0.1, val.0.2, val.0.3)).into()
                }
            }

            impl From<$unit<f32>> for UiValue<Vec2> {
                fn from(val: $unit<f32>) -> UiValue<Vec2> {
                    $unit(Vec2::new(val.0, val.0)).into()
                }
            }
            impl From<$unit<f32>> for UiValue<Vec3> {
                fn from(val: $unit<f32>) -> UiValue<Vec3> {
                    $unit(Vec3::new(val.0, val.0, val.0)).into()
                }
            }
            impl From<$unit<f32>> for UiValue<Vec4> {
                fn from(val: $unit<f32>) -> UiValue<Vec4> {
                    $unit(Vec4::new(val.0, val.0, val.0, val.0)).into()
                }
            }
        )*
    };
}
impl_uiunit!(Ab, Rl, Rw, Rh, Em, Vp, Vw, Vh);


// # Impl (A, B) => UiValue(Vec2)
impl <A, B> From<(A, B)> for UiValue<Vec2> where 
    A: Into<UiValue<f32>>, 
    B: Into<UiValue<f32>>
{
    fn from(val: (A, B)) -> Self {
        UiValue::<Vec2>::new().with_x(val.0).with_y(val.1)
    }
}

// # Impl (A, B, C) => UiValue(Vec3)
impl <A, B, C> From<(A, B, C)> for UiValue<Vec3> where 
    A: Into<UiValue<f32>>, 
    B: Into<UiValue<f32>>,
    C: Into<UiValue<f32>>
{
    fn from(val: (A, B, C)) -> Self {
        UiValue::<Vec3>::new().with_x(val.0).with_y(val.1).with_z(val.2)
    }
}

// # Impl (A, B, C, D) => UiValue(Vec4)
impl <A, B, C, D> From<(A, B, C, D)> for UiValue<Vec4> where 
    A: Into<UiValue<f32>>, 
    B: Into<UiValue<f32>>,
    C: Into<UiValue<f32>>,
    D: Into<UiValue<f32>>
{
    fn from(val: (A, B, C, D)) -> Self {
        UiValue::<Vec4>::new().with_x(val.0).with_y(val.1).with_z(val.2).with_w(val.3)
    }
}

// # Impl f32 => UiValue(f32)
impl From<f32> for UiValue<f32> {
    fn from(val: f32) -> Self {
        Ab(val).into()
    }
}
// # Impl f32 => UiValue(Vec2)
impl From<f32> for UiValue<Vec2> {
    fn from(val: f32) -> Self {
        Ab(Vec2::new(val, val)).into()
    }
}
// # Impl f32 => UiValue(Vec3)
impl From<f32> for UiValue<Vec3> {
    fn from(val: f32) -> Self {
        Ab(Vec3::new(val, val, val)).into()
    }
}
// # Impl f32 => UiValue(Vec4)
impl From<f32> for UiValue<Vec4> {
    fn from(val: f32) -> Self {
        Ab(Vec4::new(val, val, val, val)).into()
    }
}

// # Impl UiValue(f32) => UiValue(Vec2)
impl From<UiValue<f32>> for UiValue<Vec2> {
    fn from(val: UiValue<f32>) -> Self {
        let mut out = UiValue::<Vec2>::new();
        out.set_x(val);
        out.set_y(val);
        out
    }
}
// # Impl UiValue(f32) => UiValue(Vec3)
impl From<UiValue<f32>> for UiValue<Vec3> {
    fn from(val: UiValue<f32>) -> Self {
        let mut out = UiValue::<Vec3>::new();
        out.set_x(val);
        out.set_y(val);
        out.set_z(val);
        out
    }
}
// # Impl UiValue(f32) => UiValue(Vec4)
impl From<UiValue<f32>> for UiValue<Vec4> {
    fn from(val: UiValue<f32>) -> Self {
        let mut out = UiValue::<Vec4>::new();
        out.set_x(val);
        out.set_y(val);
        out.set_z(val);
        out.set_w(val);
        out
    }
}

/// Trait for implementing evaluation logic for `(T)`.
pub trait UiValueEvaluate<T> {
    /// Evaluates the NodeSize for `(T)`
    fn evaluate(&self, absolute_scale: T, parent_size: T, viewport_size: T, font_size: T) -> T;
}

// # Impl evaluate
impl UiValueEvaluate<f32> for UiValue<f32> {
    fn evaluate(&self, absolute_scale: f32, parent_size: f32, viewport_size: f32, font_size: f32) -> f32 {
        let mut out = 0.0;
        if let Some(v) = self.ab { out += v * absolute_scale }
        if let Some(v) = self.rl { out += (v/100.0) * parent_size }
        if let Some(v) = self.rw { out += (v/100.0) * parent_size }
        if let Some(v) = self.rh { out += (v/100.0) * parent_size }
        if let Some(v) = self.em { out += v * font_size }
        if let Some(v) = self.vp { out += (v/100.0) * viewport_size }
        if let Some(v) = self.vh { out += (v/100.0) * viewport_size }
        out
    }
}
impl UiValueEvaluate<Vec2> for UiValue<Vec2> {
    fn evaluate(&self, absolute_scale: Vec2, parent_size: Vec2, viewport_size: Vec2, font_size: Vec2) -> Vec2 {
        let mut out = Vec2::ZERO;
        if let Some(v) = self.ab { out += v * absolute_scale }
        if let Some(v) = self.rl { out += (v/100.0) * parent_size }
        if let Some(v) = self.rw { out += (v/100.0) * parent_size.x }
        if let Some(v) = self.rh { out += (v/100.0) * parent_size.y }
        if let Some(v) = self.em { out += v * font_size }
        if let Some(v) = self.vp { out += (v/100.0) * viewport_size.x }
        if let Some(v) = self.vh { out += (v/100.0) * viewport_size.y }
        out
    }
}
impl UiValueEvaluate<Vec3> for UiValue<Vec3> {
    fn evaluate(&self, absolute_scale: Vec3, parent_size: Vec3, viewport_size: Vec3, font_size: Vec3) -> Vec3 {
        let mut out = Vec3::ZERO;
        if let Some(v) = self.ab { out += v * absolute_scale }
        if let Some(v) = self.rl { out += (v/100.0) * parent_size }
        if let Some(v) = self.rw { out += (v/100.0) * parent_size.x }
        if let Some(v) = self.rh { out += (v/100.0) * parent_size.y }
        if let Some(v) = self.em { out += v * font_size }
        if let Some(v) = self.vp { out += (v/100.0) * viewport_size.x }
        if let Some(v) = self.vh { out += (v/100.0) * viewport_size.y }
        out
    }
}
impl UiValueEvaluate<Vec4> for UiValue<Vec4> {
    fn evaluate(&self, absolute_scale: Vec4, parent_size: Vec4, viewport_size: Vec4, font_size: Vec4) -> Vec4 {
        let mut out = Vec4::ZERO;
        if let Some(v) = self.ab { out += v * absolute_scale }
        if let Some(v) = self.rl { out += (v/100.0) * parent_size }
        if let Some(v) = self.rw { out += (v/100.0) * parent_size.x }
        if let Some(v) = self.rh { out += (v/100.0) * parent_size.y }
        if let Some(v) = self.em { out += v * font_size }
        if let Some(v) = self.vp { out += (v/100.0) * viewport_size.x }
        if let Some(v) = self.vh { out += (v/100.0) * viewport_size.y }
        out
    }
}