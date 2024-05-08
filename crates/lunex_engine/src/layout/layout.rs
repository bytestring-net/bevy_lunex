#[cfg(feature = "bevy")]
use bevy::prelude::Component;

use crate::{import::*, YInvert};
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
    Div(Div),
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
            Layout::Div(layout) => format!("{} {}", "Div".bold().bright_cyan(), layout.to_nicestr()),
        }
    }
}


// #=========================#
// #=== LAYOUT PROPERTIES ===#

/// **Anchor** - A type used to define where should Window node layout be anchored at.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Anchor {
    Center,
    BottomLeft,
    BottomCenter,
    BottomRight,
    CenterLeft,
    CenterRight,
    #[default] TopLeft,
    TopCenter,
    TopRight,
    /// Custom anchor point. Top left is `(-0.5, 0.5)`, center is `(0.0, 0.0)`. The value will
    /// be scaled with the sprite size.
    Custom(Vec2),
}
impl NiceDisplay for Anchor {
    fn to_nicestr(&self) -> String {
        match self {
            Anchor::Center => format!("{}", "Center".bold()),
            Anchor::BottomLeft => format!("{}", "BottomLeft".bold()),
            Anchor::BottomCenter => format!("{}", "BottomCenter".bold()),
            Anchor::BottomRight => format!("{}", "BottomRight".bold()),
            Anchor::CenterLeft => format!("{}", "CenterLeft".bold()),
            Anchor::CenterRight => format!("{}", "CenterRight".bold()),
            Anchor::TopLeft => format!("{}", "TopLeft".bold()),
            Anchor::TopCenter => format!("{}", "TopCenter".bold()),
            Anchor::TopRight => format!("{}", "TopRight".bold()),
            Anchor::Custom(point) => format!("({} {})", point.x.to_string().bold(), point.y.to_string().bold()),
        }
    }
}
impl Anchor {
    pub fn as_vec(&self) -> Vec2 {
        match self {
            Anchor::BottomLeft => Vec2::new(0.0, 1.0),
            Anchor::BottomCenter => Vec2::new(0.5, 1.0),
            Anchor::BottomRight => Vec2::new(1.0, 1.0),
            Anchor::CenterLeft => Vec2::new(0.0, 0.5),
            Anchor::Center => Vec2::new(0.5, 0.5),
            Anchor::CenterRight => Vec2::new(1.0, 0.5),
            Anchor::TopLeft => Vec2::new(0.0, 0.0),
            Anchor::TopCenter => Vec2::new(0.5, 0.0),
            Anchor::TopRight => Vec2::new(1.0, 0.0),
            Anchor::Custom(point) => *point,
        }
    }
}
#[cfg(feature = "bevy")]
impl Into<Anchor> for bevy::sprite::Anchor {
    fn into(self) -> Anchor {
        match self {
            bevy::sprite::Anchor::Center => Anchor::Center,
            bevy::sprite::Anchor::BottomLeft => Anchor::BottomLeft,
            bevy::sprite::Anchor::BottomCenter => Anchor::BottomCenter,
            bevy::sprite::Anchor::BottomRight => Anchor::BottomRight,
            bevy::sprite::Anchor::CenterLeft => Anchor::CenterLeft,
            bevy::sprite::Anchor::CenterRight => Anchor::CenterRight,
            bevy::sprite::Anchor::TopLeft => Anchor::TopLeft,
            bevy::sprite::Anchor::TopCenter => Anchor::TopCenter,
            bevy::sprite::Anchor::TopRight => Anchor::TopRight,
            bevy::sprite::Anchor::Custom(point) => Anchor::Custom(point.invert_y() + 0.5),
        }
    }
}


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


/// **Sizing** - A type used to define how should a Div node layout size itself.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::Sizing;
/// let scaling: Sizing = Sizing::Min;   // -> Tries to reach minimum size limit
/// let scaling: Sizing = Sizing::Basic; // -> Default value, as big as its content
/// let scaling: Sizing = Sizing::Max;   // -> Tries to reach maximum size limit
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Sizing {
    /// Div node layout should be as small as possible.
    Min,
    /// Div node layout should 
    #[default] Basic,
    /// Div node layout should be as big as possible.
    Max,
}
impl NiceDisplay for Sizing {
    fn to_nicestr(&self) -> String {
        match self {
            Sizing::Min => format!("{}", "Min".bold()),
            Sizing::Basic => format!("{}", "Basic".bold()),
            Sizing::Max => format!("{}", "Max".bold()),
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
    /// Decides where position should be applied at.
    pub anchor: Anchor,
    /// Size of the node layout.
    pub size: UiValue<Vec2>,
}
impl Window {
    /// Creates new empty Window node layout.
    pub const fn new() -> Self {
        Window {
            pos : UiValue::new(),
            anchor: Anchor::TopLeft,
            size: UiValue::new(),
        }
    }
    /// Creates new full Window node layout.
    pub fn full() -> Self {
        Window {
            pos : UiValue::new(),
            anchor: Anchor::TopLeft,
            size: Rl(100.0).into(),
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
    /// Replaces the anchor with a new value.
    pub fn anchor(mut self, anchor: impl Into<Anchor>) -> Self {
        self.anchor = anchor.into();
        self
    }

    /// Computes the layout based on given parameters.
    pub(crate) fn compute(&self, parent: Rectangle2D, absolute_scale: f32, viewport_size: Vec2, font_size: f32) -> Rectangle2D {
        let pos = self.pos.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size));
        let size = self.size.evaluate(Vec2::splat(absolute_scale), parent.size, viewport_size, Vec2::splat(font_size));
        Rectangle2D {
            pos: parent.pos + pos - size * self.anchor.as_vec(),
            size,
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
        let t = format!("[pos: ({}) size: ({}) anchor: {}]", self.pos.to_nicestr(), self.size.to_nicestr(), self.anchor.to_nicestr());
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


/// **Div** - Parametric layout type that is defined by margin, border and padding. Its location and size
/// is based on the surrounding nodes, like HTML. It is also the only node layout that uses the [`Sp`] unit.
/// You can use this unit for alignment and justification.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::{Div, Sp};
/// let layout: Layout = Div::new().pad_x(2.0).margin_y(Sp(1.0)).br().pack();
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Div {
    /// Describes how width should size itself.
    pub width: Sizing,
    /// Describes how height should size itself.
    pub height: Sizing,
    /// Optional minamal size the node layout can be.
    pub min_size: Option<UiValue<Vec2>>,
    /// Optional maximal size the node layout can be.
    pub max_size: Option<UiValue<Vec2>>,
    /// The space between the node border and the node content. `x-left`, `y-top`, `z-right`, `w-bottom`
    pub padding: UiValue<Vec4>,
    /// The line width of each border. `x-left`, `y-top`, `z-right`, `w-bottom`
    pub border: UiValue<Vec4>,
    /// The space between this node and surrounding nodes. `x-left`, `y-top`, `z-right`, `w-bottom`
    pub margin: UiValue<Vec4>,
    /// Force a line break in the ui flow after this node.
    pub br: bool,
}
impl Div {
    /// Creates new empty Div node layout.
    pub const fn new() -> Self {
        Div {
            width: Sizing::Basic,
            height: Sizing::Basic,
            min_size: None,
            max_size: None,
            padding: UiValue::new(),
            border: UiValue::new(),
            margin: UiValue::new(),
            br: false,
        }
    }
    /// Replaces the width with a new value.
    pub fn width(mut self, sizing: Sizing) -> Self {
        self.width = sizing;
        self
    }
    /// Replaces the height with a new value.
    pub fn height(mut self, sizing: Sizing) -> Self {
        self.height = sizing;
        self
    }
    /// Replaces the minimal size with a new value.
    pub fn min(mut self, size: impl Into<UiValue<Vec2>>) -> Self {
        self.min_size = Some(size.into());
        self
    }
    /// Replaces the minimal width with a new value.
    pub fn min_width(mut self, size: impl Into<UiValue<f32>>) -> Self {
        if let Some(mut minsize) = self.min_size {
            minsize.set_x(size.into())
        } else {
            self.min_size = Some(UiValue::<Vec2>::new().with_x(size.into()));
        }
        self
    }
    /// Replaces the minimal height with a new value.
    pub fn min_height(mut self, size: impl Into<UiValue<f32>>) -> Self {
        if let Some(mut minsize) = self.min_size {
            minsize.set_y(size.into())
        } else {
            self.min_size = Some(UiValue::<Vec2>::new().with_y(size.into()));
        }
        self
    }
    /// Replaces the maximum size with a new value.
    pub fn max(mut self, size: impl Into<UiValue<Vec2>>) -> Self {
        self.max_size = Some(size.into());
        self
    }
    /// Replaces the maximal width with a new value.
    pub fn max_width(mut self, size: impl Into<UiValue<f32>>) -> Self {
        if let Some(mut maxsize) = self.max_size {
            maxsize.set_x(size.into())
        } else {
            self.max_size = Some(UiValue::<Vec2>::new().with_x(size.into()));
        }
        self
    }
    /// Replaces the maximal height with a new value.
    pub fn max_height(mut self, size: impl Into<UiValue<f32>>) -> Self {
        if let Some(mut maxsize) = self.max_size {
            maxsize.set_y(size.into())
        } else {
            self.max_size = Some(UiValue::<Vec2>::new().with_y(size.into()));
        }
        self
    }
    /// Replaces the padding with a new value.
    pub fn pad(mut self, pad: impl Into<UiValue<Vec4>>) -> Self {
        self.padding = pad.into();
        self
    }
    /// Replaces the horizontal padding with a new value.
    pub fn pad_x(mut self, pad: impl Into<UiValue<Vec2>>) -> Self {
        let pad: UiValue<Vec2> = pad.into();
        let val = pad.get_x();
        self.padding.set_x(val);
        self.padding.set_z(val);
        self
    }
    /// Replaces the vertical padding with a new value.
    pub fn pad_y(mut self, pad: impl Into<UiValue<Vec2>>) -> Self {
        let pad: UiValue<Vec2> = pad.into();
        let val = pad.get_y();
        self.padding.set_y(val);
        self.padding.set_w(val);
        self
    }
    /// Replaces the left padding with a new value.
    pub fn pad_l(mut self, pad: impl Into<UiValue<f32>>) -> Self {
        self.padding.set_x(pad);
        self
    }
    /// Replaces the top padding with a new value.
    pub fn pad_t(mut self, pad: impl Into<UiValue<f32>>) -> Self {
        self.padding.set_y(pad);
        self
    }
    /// Replaces the right padding with a new value.
    pub fn pad_r(mut self, pad: impl Into<UiValue<f32>>) -> Self {
        self.padding.set_z(pad);
        self
    }
    /// Replaces the bottom padding with a new value.
    pub fn pad_b(mut self, pad: impl Into<UiValue<f32>>) -> Self {
        self.padding.set_w(pad);
        self
    }
    /// Replaces the border with a new value.
    pub fn border(mut self, border: impl Into<UiValue<Vec4>>) -> Self {
        self.border = border.into();
        self
    }
    /// Replaces the horizontal border with a new value.
    pub fn border_x(mut self, border: impl Into<UiValue<Vec2>>) -> Self {
        let border: UiValue<Vec2> = border.into();
        let val = border.get_x();
        self.border.set_x(val);
        self.border.set_z(val);
        self
    }
    /// Replaces the vertical border with a new value.
    pub fn border_y(mut self, border: impl Into<UiValue<Vec2>>) -> Self {
        let border: UiValue<Vec2> = border.into();
        let val = border.get_y();
        self.border.set_y(val);
        self.border.set_w(val);
        self
    }
    /// Replaces the left border with a new value.
    pub fn border_l(mut self, border: impl Into<UiValue<f32>>) -> Self {
        self.border.set_x(border);
        self
    }
    /// Replaces the top border with a new value.
    pub fn border_t(mut self, border: impl Into<UiValue<f32>>) -> Self {
        self.border.set_y(border);
        self
    }
    /// Replaces the right border with a new value.
    pub fn border_r(mut self, border: impl Into<UiValue<f32>>) -> Self {
        self.border.set_z(border);
        self
    }
    /// Replaces the bottom border with a new value.
    pub fn border_b(mut self, border: impl Into<UiValue<f32>>) -> Self {
        self.border.set_w(border);
        self
    }
    /// Replaces the margin with a new value.
    pub fn margin(mut self, margin: impl Into<UiValue<Vec4>>) -> Self {
        self.margin = margin.into();
        self
    }
    /// Replaces the horizontal margin with a new value.
    pub fn margin_x(mut self, margin: impl Into<UiValue<Vec2>>) -> Self {
        let margin: UiValue<Vec2> = margin.into();
        let val = margin.get_x();
        self.margin.set_x(val);
        self.margin.set_z(val);
        self
    }
    /// Replaces the vertical margin with a new value.
    pub fn margin_y(mut self, margin: impl Into<UiValue<Vec2>>) -> Self {
        let margin: UiValue<Vec2> = margin.into();
        let val = margin.get_y();
        self.margin.set_y(val);
        self.margin.set_w(val);
        self
    }
    /// Replaces the left margin with a new value.
    pub fn margin_l(mut self, margin: impl Into<UiValue<f32>>) -> Self {
        self.margin.set_x(margin);
        self
    }
    /// Replaces the top margin with a new value.
    pub fn margin_t(mut self, margin: impl Into<UiValue<f32>>) -> Self {
        self.margin.set_y(margin);
        self
    }
    /// Replaces the right margin with a new value.
    pub fn margin_r(mut self, margin: impl Into<UiValue<f32>>) -> Self {
        self.margin.set_z(margin);
        self
    }
    /// Replaces the bottom margin with a new value.
    pub fn margin_b(mut self, margin: impl Into<UiValue<f32>>) -> Self {
        self.margin.set_w(margin);
        self
    }
    /// Makes any container after this start at new line
    pub fn br(mut self) -> Self {
        self.br = true;
        self
    }


    /// Packs the struct into Layout
    pub fn pack(self) -> Layout {
        self.into()
    }
}
impl Into<Layout> for Div {
    fn into(self) -> Layout {
        Layout::Div(self)
    }
}
impl NiceDisplay for Div {
    fn to_nicestr(&self) -> String {
        let t = format!("[pad: ({}) mar: ({})]", self.padding.to_nicestr(), self.margin.to_nicestr());
        format!("{}", t.black())
    }
}
