use crate::*;

// Exported prelude
pub mod prelude {
    // All standard exports
    pub use super::{
        Align,
        Scaling,
    };
}

// #============================#
// #=== MULTIPURPOSE STRUCTS ===#

/// **Rectangle 2D** - Contains computed values from node layout.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
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
/// # use bevy_lunex::*;
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
/// # use bevy_lunex::*;
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
            UiLayoutType::Boundary(layout) => layout.compute(parent, absolute_scale, viewport_size, font_size),
            UiLayoutType::Window(layout) => layout.compute(parent, absolute_scale, viewport_size, font_size),
            UiLayoutType::Solid(layout) => layout.compute(parent, absolute_scale, viewport_size, font_size),
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
    /// Wrap the layout type into UiLayout
    pub fn wrap(self) -> UiLayoutType {
        UiLayoutType::from(self)
    }
    /// Computes the layout based on given parameters.
    pub(crate) fn compute(&self, parent: &Rectangle2D, absolute_scale: f32, viewport_size: Vec2, font_size: f32) -> Rectangle2D {
        let pos1 = self.pos1.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size));
        let pos2 = self.pos2.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size));
        let size = pos2 - pos1;
        Rectangle2D {
            pos: -parent.size / 2.0 + pos1 + size/2.0,
            size,
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
    /// Wrap the layout type into UiLayout
    pub fn wrap(self) -> UiLayoutType {
        UiLayoutType::from(self)
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
    /// Wrap the layout type into UiLayout
    pub fn wrap(self) -> UiLayoutType {
        UiLayoutType::from(self)
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
