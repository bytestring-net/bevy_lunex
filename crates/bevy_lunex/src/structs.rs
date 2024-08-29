use crate::*;
use bevy::{render::primitives::Aabb, sprite::{Anchor, Material2d, Mesh2dHandle, SpriteSource}, text::{Text2dBounds, TextLayoutInfo}};


// #=====================#
// #=== STATE STRUCTS ===#

/// Trait for creating new UI states
pub trait UiState where Self: Send + Sync + 'static {
    const INDEX: usize;
}


/// UI state of a component, this is the normal default
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Base;
impl UiState for Base {
    const INDEX: usize = 0;
}

/// UI state of a component, is active on hover
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Hover;
impl UiState for Hover {
    const INDEX: usize = 1;
}

/// UI state of a component, is active when clicked
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Clicked;
impl UiState for Clicked {
    const INDEX: usize = 2;
}

/// UI state of a component, is active when selected
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Selected;
impl UiState for Selected {
    const INDEX: usize = 3;
}

/// UI state of a component, is active after entity is spawned
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Intro;
impl UiState for Intro {
    const INDEX: usize = 4;
}

/// UI state of a component, is active before entity is despawned
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Outro;
impl UiState for Outro {
    const INDEX: usize = 5;
}


// #=========================#
// #=== MARKER COMPONENTS ===#

/// This struct marks [`UiTree`] entity to receive piped [`Camera`] size and position to its [`Dimension`] and [`Transform`] component.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct SourceFromCamera;

/// This struct is used to mark linked UI entities as elements for easier rendering.
/// They are picked up by different systems, that ensure their piped [`Transform`] is centered,
/// instead of being aligned in a top-left corner like the normal UI entities.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct Element;


// #======================#
// #=== STD COMPONENTS ===#

/// This struct holds rectangular data. If the component covers some kind of 2D area, it should be stored in this component.
/// Lunex uses this component to mirror node size in & out from parent [`UiTree`].
#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct Dimension {
    pub size: Vec2,
}
impl Dimension {
    pub fn new(size: impl Into<Vec2>) -> Self {
        Dimension {
            size: size.into()
        }
    }
}

/// # WIP - used for Div layout
#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct UiContent {
    pub size: Vec2,
}
impl UiContent {
    pub fn new(size: impl Into<Vec2>) -> Self {
        UiContent { size: size.into() }
    }
}


/// This struct is used to specify size of the font in UI.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct UiTextSize {
    /// The unit type and scale value of the text height
    pub size: UiValueType<f32>,
}
impl Default for UiTextSize {
    fn default() -> Self {
        Self { size: Rh(1.0).into() }
    }
}
impl UiTextSize {
    /// Creates new instance from default
    pub fn new() -> Self {
        Default::default()
    }
    /// Specify the height of the font
    pub fn size(mut self, size: impl Into<UiValueType<f32>>) -> Self {
        self.size = size.into();
        self
    }
}

// #=======================#
// #=== MAIN COMPONENTS ===#

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct UiLayout<S = Base> {
    pub layout: Layout,
    state: PhantomData<S>,
}
impl UiLayout {
    /// **Boundary** - Declarative layout type that is defined by its top-left corner and bottom-right corner.
    /// Nodes with this layout are not included in the ui flow.
    /// ## 🛠️ Example
    /// ```
    /// # use lunex_engine::{UiLayout, Rl};
    /// let layout: UiLayout = UiLayout::boundary().pos1(Rl(20.0)).pos2(Rl(80.0)).pack();
    /// ```
    pub fn boundary() -> ui::Boundary {
        ui::Boundary::new()
    }
    /// **Window** - Declarative layout type that is defined by its size and position.
    /// Nodes with this layout are not included in the ui flow.
    /// ## 🛠️ Example
    /// ```
    /// # use lunex_engine::{UiLayout, Ab, Rl};
    /// let layout: UiLayout = UiLayout::window().pos(Ab(100.0)).size(Rl(50.0)).pack();
    /// ```
    pub fn window() -> ui::Window {
        ui::Window::new()
    }
    /// **Window** (full) - Declarative layout type that is defined by its size and position.
    /// Nodes with this layout are not included in the ui flow.
    /// ## 🛠️ Example
    /// ```
    /// # use lunex_engine::{UiLayout, Rl};
    /// let layout: UiLayout = UiLayout::window().size(Rl(100.0)).pack(); // Same as UiLayout::window_full()
    /// ```
    pub fn window_full() -> ui::Window {
        ui::Window::full()
    }
    /// **Solid** - Declarative layout type that is defined by its width and height ratio.
    /// Scales in a way to fit itself inside parent container. It never deforms.
    /// Nodes with this layout are not included in the ui flow.
    /// ## 🛠️ Example
    /// ```
    /// # use lunex_engine::UiLayout;
    /// let layout: UiLayout = UiLayout::solid().size((4.0, 3.0)).align_x(-0.8).pack();
    /// ```
    pub fn solid() -> ui::Solid {
        ui::Solid::new()
    }
    /// **Div** - Parametric layout type that is defined by margin, border and padding. Its location and size
    /// is based on the surrounding nodes, like HTML. It is also the only node layout that uses the [`Sp`] unit.
    /// You can use this unit for alignment and justification.
    /// ## 🛠️ Example
    /// ```
    /// # use lunex_engine::{UiLayout, Sp};
    /// let layout: UiLayout = UiLayout::new().pad_x(2.0).margin_y(Sp(1.0)).br().pack();
    /// ```
    pub fn div() -> ui::Div {
        ui::Div::new()
    }
}
impl <S> UiLayout<S> {
    /// Creates struct from layout
    pub fn from(layout: impl Into<Layout>) -> UiLayout<S> {
        UiLayout {
            layout: layout.into(),
            state: PhantomData,
        }
    }
}
impl <S> Default for UiLayout<S> {
    fn default() -> Self {
        UiLayout {
            layout: Layout::default(),
            state: PhantomData,
        }
    }
}

pub trait PackageLayout {
    fn pack<S>(self) -> UiLayout<S>;
}

// Implement packaging 
impl <S> From<ui::Boundary> for UiLayout<S> {
    fn from(val: ui::Boundary) -> Self {
        val.pack::<S>()
    }
}
impl PackageLayout for ui::Boundary {
    fn pack<S>(self) -> UiLayout<S> {
        UiLayout::<S>::from(self)
    }
}
impl <S> From<ui::Window> for UiLayout<S> {
    fn from(val: ui::Window) -> Self {
        val.pack::<S>()
    }
}
impl PackageLayout for ui::Window {
    fn pack<S>(self) -> UiLayout<S> {
        UiLayout::<S>::from(self)
    }
}
impl <S> From<ui::Solid> for UiLayout<S> {
    fn from(val: ui::Solid) -> Self {
        val.pack::<S>()
    }
}
impl PackageLayout for ui::Solid {
    fn pack<S>(self) -> UiLayout<S> {
        UiLayout::<S>::from(self)
    }
}
impl <S> From<ui::Div> for UiLayout<S> {
    fn from(val: ui::Div) -> Self {
        val.pack::<S>()
    }
}
impl PackageLayout for ui::Div {
    fn pack<S>(self) -> UiLayout<S> {
        UiLayout::<S>::from(self)
    }
}

/// This struct controls what 2 layouts should be computed and lerped between.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct UiLayoutController {
    /// Indexes of the two layouts to tween between
    pub index: [usize; 2],
    /// The transition ranging from 0.0 to 1.0
    pub tween: f32,
    /// The method called for smoothing the tween value
    pub method: fn(f32) -> f32,
}
impl Default for UiLayoutController {
    fn default() -> Self {
        UiLayoutController { 
            index: [0, 0],
            tween: 0.0,
            method: |i|{i},
        }
    }
}


/// This struct is a string reference to a specific node in a parent [`UiTree`].
/// Lunex uses this component to locate what data this entity should be working with.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct UiLink<T = MainUi> {
    pub path: String,
    marker: PhantomData<T>,
}
impl <T> UiLink<T> {
    pub fn path( path: impl Borrow<str>) -> Self {
        UiLink {
            path: path.borrow().to_string(),
            marker: PhantomData,
        }
    }
    pub fn add( &self, path: impl Borrow<str>) -> Self {
        UiLink {
            path: format!("{}/{}", self.path, path.borrow()),
            marker: PhantomData,
        }
    }
    pub fn new() -> Self {
        UiLink {
            path: "/".to_string(),
            marker: PhantomData,
        }
    }
}
impl <T> Default for UiLink<T> {
    fn default() -> Self {
        UiLink {
            path: "/".to_string(),
            marker: PhantomData,
        }
    }
}


/// This struct holds depth bias that will be relatively added to `depth` in the layout calculation.
/// Nodes with higher depth bias will be placed on top of nodes with lower depth bias.
/// It is recursive.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct UiDepthBias (pub f32);


// #====================#
// #=== MAIN BUNDLES ===#

/// Main bundle for spawning `UiTree` entity
#[derive(Bundle, Debug, Clone)]
pub struct UiTreeBundle <T:Component = MainUi, N:Default + Component = NoData> {
    /// Required to be picked up by compute system.
    pub link: UiLink<T>,
    /// The ui layout data of the entity and it's children.
    pub tree: UiTree<T, N>,
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,
    /// The transform of the entity.
    pub transform: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
}
impl <T:Component, N:Default + Component> From<UiTree<T, N>> for UiTreeBundle<T, N> {
    fn from(value: UiTree<T, N>) -> Self {
        UiTreeBundle::<T, N> {
            tree: value,
            ..default()
        }
    }
}
impl <T:Component, N:Default + Component> Default for UiTreeBundle<T, N> {
    fn default() -> Self {
        UiTreeBundle {
            link: Default::default(),
            tree: Default::default(),
            dimension: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}


/// Main bundle for spawning `UiNode` entity as a child of [`UiTree`] entity.
/// All this does is defines the layout within [`UiTree`]. Use additional
/// bundles for further functionality.
#[derive(Bundle, Debug, Clone, Default)]
pub struct UiNodeBundle<T: Component = MainUi> {
    /// The corresponding path that leads to the node data in parent UiTree entity.
    pub link: UiLink<T>,
    /// The layout to use when computing this node.
    pub layout: UiLayout,
}


/// Additional bundle for `UiNode` entity.
/// This is used for entities that have a mesh or a sprite.
/// For this purpose [`Element`] component is provided which centers
/// the anchor for piped position from node.
#[derive(Bundle, Debug, Clone, Default)]
pub struct UiElementBundle {
    /// Marks this as node element.
    pub element: Element,
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,
    /// The transform of the entity.
    pub transform: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
}


/// Additional bundle for `UiNode` entity.
/// This is used for entities that don't have a mesh or a sprite,
/// but still needs to be pickable.
#[derive(Bundle, Default)]
pub struct UiZoneBundle {
    /// The required bundle to make entity pickable
    pub pickable: PickableBundle,
    /// This component is required for picking to work on non-sprite entities
    pub sprite_source: SpriteSource,
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,
    /// The transform of the entity.
    pub transform: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
}


/// Additional bundle for `UiNode` entity.
/// This is required by any UI entity that needs to exist in worldspace.
#[derive(Bundle, Debug, Clone, Default)]
pub struct UiSpatialBundle {
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,
    /// The transform of the entity.
    pub transform: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
}


// #=======================#
// #=== SPECIAL BUNDLES ===#

/// Additional bundle for `UiNode` entity.
/// Provides functionality to bind sprite in 3D on a plane mesh to `UiNode`.
#[derive(Bundle, Debug, Default, Clone)]
pub struct UiMaterial3dBundle {
    /// Quad mesh that is generated every time node is changed.
    pub mesh: Handle<Mesh>,
    /// The material used for the quad.
    pub material: Handle<StandardMaterial>,
    /// Image boundary for culling.
    pub aabb: Aabb,
    /// Marks this as node element.
    pub element: Element,
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,
    /// The transform of the entity.
    pub transform: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
}
impl From<Handle<StandardMaterial>> for UiMaterial3dBundle {
    fn from(value: Handle<StandardMaterial>) -> Self {
        UiMaterial3dBundle {
            material: value,
            ..default()
        }
    }
}
impl UiMaterial3dBundle {
    pub fn from_image(materials: &mut ResMut<'_, Assets<StandardMaterial>>, value: Handle<Image>) -> Self {
        UiMaterial3dBundle {
            material: materials.add(StandardMaterial { base_color_texture: Some(value), unlit: true, ..default() }),
            ..default()
        }
    }
    pub fn from_transparent_image(materials: &mut ResMut<'_, Assets<StandardMaterial>>, value: Handle<Image>) -> Self {
        UiMaterial3dBundle {
            material: materials.add(StandardMaterial { base_color_texture: Some(value), unlit: true, alpha_mode: AlphaMode::Blend, ..default() }),
            ..default()
        }
    }
}


/// Additional bundle for `UiNode` entity.
/// Provides functionality to bind mesh in 2D to a `UiNode`.
#[derive(Bundle, Default, Clone)]
pub struct UiMaterial2dBundle<M: Material2d> {
    /// The mesh
    pub mesh: Mesh2dHandle,
    /// Material of the mesh
    pub material: Handle<M>,
    /// Marks this as node element.
    pub element: Element,
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,
    /// The transform of the entity.
    pub transform: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
}


/// Additional bundle for `UiNode` entity.
/// Provides functionality to bind sprite to `UiNode`.
#[derive(Bundle, Clone, Debug, Default)]
pub struct UiImage2dBundle {
    /// Image properties.
    pub sprite: Sprite,
    /// Image texture.
    pub texture: Handle<Image>,
    /// Image boundary for culling.
    pub aabb: Aabb,
    /// Marks this as node element.
    pub element: Element,
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,
    /// The transform of the entity.
    pub transform: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
}
impl From<Handle<Image>> for UiImage2dBundle {
    fn from(value: Handle<Image>) -> Self {
        UiImage2dBundle {
            texture: value,
            ..default()
        }
    }
}


/// Additional bundle for `UiNode` entity.
/// Provides functionality to bind text to `UiNode`.
#[derive(Bundle, Clone, Debug, Default)]
pub struct UiText2dBundle {
    /// Contains the text.
    pub text: Text,
    /// This is needed for visibility computation to work properly.
    pub sprite_source: SpriteSource,
    /// How the text is positioned relative to its transform.
    pub text_anchor: Anchor,
    /// The maximum width and height of the text.
    pub text_2d_bounds: Text2dBounds,
    /// Contains the size of the text and its glyph's position and scale data. Generated via [`TextPipeline::queue_text`]
    pub text_layout_info: TextLayoutInfo,
    /// Marks this as node element.
    pub element: Element,
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,
    /// The transform of the entity.
    pub transform: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
}
