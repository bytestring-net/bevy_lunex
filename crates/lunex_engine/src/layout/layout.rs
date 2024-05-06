#[cfg(feature = "bevy")]
use bevy::prelude::Component;

use crate::import::*;
use crate::{NiceDisplay, Rectangle2D, UiValue, UiValueEvaluate, Ab, Rl};


// #===================#
// #=== LAYOUT ENUM ===#

/// **Layout** - Component that defines where should a node be located.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::{UiLayout, Rl};
/// let layout: Layout = UiLayout::Window::new().size(Rl(50.0)).pack();
/// let layout: Layout = UiLayout::Window::new().size(Rl(50.0)).pack();
/// let layout: Layout = UiLayout::Window::new().size(Rl(50.0)).pack();
/// ```
/// The expected range is `-1.0` to `1.0`, but you can extrapolate.
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Layout {
    Boundary(Boundary),
    Window(Window),
    Solid(Solid),
    //Div(Div),
}
impl Default for Layout {
    fn default() -> Self {
        Window::new().size(Rl(100.0)).into()
    }
}
impl NiceDisplay for Layout {
    fn to_nicestr(&self) -> String {
        match self {
            Layout::Boundary(layout) => format!("{} {}", "Boundary".bold().bright_cyan(), layout.to_nicestr()),
            Layout::Solid(layout) => format!("{} {}", "Solid".bold().bright_cyan(), layout.to_nicestr()),
            Layout::Window(layout) => format!("{} {}", "Window".bold().bright_cyan(), layout.to_nicestr()),
            //Layout::Div(layout) => format!("{} {}", "Div".bold().bright_cyan(), layout.to_nicestr()),
        }
    }
}


// #=========================#
// #=== LAYOUT PROPERTIES ===#

/// **Align** - A type used to define alignment in a node layout.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::Align;
/// let align: Align = Align::START; // -> -1.0
/// let align: Align = Align(-1.0);  // -> -1.0
/// let align: Align = -1.0.into();  // -> -1.0
/// ```
/// The expected range is `-1.0` to `1.0`, but you can extrapolate.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Align (pub f32);
impl Align {
    pub const START: Align = Align(-1.0);
    pub const LEFT: Align = Align(-1.0);
    pub const CENTER: Align = Align(0.0);
    pub const MIDDLE: Align = Align(0.0);
    pub const END: Align = Align(1.0);
    pub const RIGHT: Align = Align(1.0);
}
impl Into<Align> for f32 {
    fn into(self) -> Align {
        Align(self)
    }
}
impl NiceDisplay for Align {
    fn to_nicestr(&self) -> String {
        format!("{}", self.0.to_string().bold())
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
#[derive(Debug, Default, Clone, Copy, PartialEq)]
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
impl NiceDisplay for Scaling {
    fn to_nicestr(&self) -> String {
        match self {
            Scaling::HorFill => format!("{}", "HorFill".bold()),
            Scaling::VerFill => format!("{}", "VerFill".bold()),
            Scaling::Fit => format!("{}", "Fit".bold()),
            Scaling::Fill => format!("{}", "Fill".bold()),
        }
    }
}


// #====================#
// #=== LAYOUT TYPES ===#

/// **Boundary** - Declarative layout type that is defined by its top-left corner and bottom-right corner.
/// Nodes with this layout are not included in the ui flow.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::{Boundary, Rl};
/// let layout: Layout = Boundary::new().pos1(Rl(20.0)).pos2(Rl(80.0)).pack();
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Boundary {
    /// Position of the top-left corner.
    pub pos1: UiValue<Vec2>,
    /// Position of the bottom-right corner.
    pub pos2: UiValue<Vec2>,
}
impl Boundary {
    /// Creates new empty Boundary node layout.
    pub const fn new() -> Self {
        Boundary {
            pos1 : UiValue::new(),
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

    /// Computes the layout based on given parameters.
    pub(crate) fn compute(&self, parent: Rectangle2D, absolute_scale: f32, viewport_size: Vec2, font_size: f32) -> Rectangle2D {
        let pos1 = self.pos1.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size));
        let pos2 = self.pos2.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size));
        Rectangle2D {
            pos: parent.pos + pos1,
            size: pos2 - pos1,
        }
    }
    /// Packs the struct into Layout.
    pub fn pack(self) -> Layout {
        self.into()
    }
}
impl Into<Layout> for Boundary {
    fn into(self) -> Layout {
        Layout::Boundary(self)
    }
}
impl NiceDisplay for Boundary {
    fn to_nicestr(&self) -> String {
        let t = format!("[pos1: ({}) pos2: ({})]", self.pos1.to_nicestr(), self.pos2.to_nicestr());
        format!("{}", t.black())
    }
}


/// **Window** - Declarative layout type that is defined by its size and position.
/// Nodes with this layout are not included in the ui flow.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::{Window, Ab, Rl};
/// let layout: Layout = Window::new().pos(Ab(100.0)).size(Rl(50.0)).pack();
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Window {
    /// Position of the top-left corner.
    pub pos : UiValue<Vec2>,
    /// Size of the node layout.
    pub size: UiValue<Vec2>,
}
impl Window {
    /// Creates new empty Window node layout.
    pub const fn new() -> Self {
        Window {
            pos : UiValue::new(),
            size: UiValue::new(),
        }
    }
    /// Creates new empty Window node layout centered at provided position.
    pub fn new_centered_at(pos: impl Into<UiValue<Vec2>>, size: impl Into<UiValue<Vec2>> ) -> Self {
        let s = size.into();
        Window::new().pos(pos.into() - (s.clone() * 0.5)).size(s)
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
    
    /// Computes the layout based on given parameters.
    pub(crate) fn compute(&self, parent: Rectangle2D, absolute_scale: f32, viewport_size: Vec2, font_size: f32) -> Rectangle2D {
        Rectangle2D {
            pos: parent.pos + self.pos.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size)),
            size: self.size.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size)),
        }
    }
    /// Packs the struct into Layout.
    pub fn pack(self) -> Layout {
        self.into()
    }
}
impl Into<Layout> for Window {
    fn into(self) -> Layout {
        Layout::Window(self)
    }
}
impl NiceDisplay for Window {
    fn to_nicestr(&self) -> String {
        let t = format!("[pos: ({}) size: ({})]", self.pos.to_nicestr(), self.size.to_nicestr());
        format!("{}", t.black())
    }
}


/// **Solid** - Declarative layout type that is defined by its width and height ratio.
/// Scales in a way to fit itself inside parent container. It never deforms.
/// Nodes with this layout are not included in the ui flow.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::Solid;
/// let layout: Layout = Solid::new().size((4.0, 3.0)).align_x(-0.8).pack();
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Solid {
    /// Aspect ratio of the width and height. `1:1 == 10:10 == 100:100`.
    pub size: UiValue<Vec2>,
    /// Horizontal alignment within parent.
    pub align_x: Align,
    /// Vertical alignment within parent.
    pub align_y: Align,
    /// Specifies container scaling.
    pub scaling: Scaling,
}
impl Solid {
    /// Creates new empty Solid node layout.
    pub fn new() -> Self {
        Solid {
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
    pub fn scaling(mut self, cover: Scaling) -> Self {
        self.scaling = cover;
        self
    }

    /// Computes the layout based on given parameters.
    pub(crate) fn compute(&self, parent: Rectangle2D, absolute_scale: f32, viewport_size: Vec2, font_size: f32) -> Rectangle2D {
        
        let size = self.size.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size));

        let scale = match self.scaling {
            Scaling::HorFill => parent.size.x / size.x,
            Scaling::VerFill => parent.size.y / size.y,
            Scaling::Fit => f32::min(parent.size.x / size.x, parent.size.y / size.y),
            Scaling::Fill => f32::max(parent.size.x / size.x, parent.size.y / size.y),
        };

        let center_point = Vec2::new(parent.pos.x + parent.size.x / 2.0, parent.pos.y + parent.size.y / 2.0);

        let computed_width = size.x * scale;
        let computed_height = size.y * scale;
        let computed_point = Vec2::new(center_point.x - computed_width / 2.0, center_point.y - computed_height / 2.0);

        Rectangle2D {
            pos: Vec2::new(
                computed_point.x + (computed_point.x - parent.pos.x) * self.align_x.0,
                computed_point.y + (computed_point.y - parent.pos.y) * self.align_y.0,
            ),
            size: (computed_width, computed_height).into(),
        }
    }
    /// Packs the struct into Layout.
    pub fn pack(self) -> Layout {
        self.into()
    }
}
impl Into<Layout> for Solid {
    fn into(self) -> Layout {
        Layout::Solid(self)
    }
}
impl NiceDisplay for Solid {
    fn to_nicestr(&self) -> String {
        let t = format!("[size: ({}) align_x: {} align_y: {}]", self.size.to_nicestr(), self.align_x.to_nicestr(), self.align_y.to_nicestr());
        format!("{}", t.black())
    }
}
