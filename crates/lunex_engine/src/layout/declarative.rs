use crate::import::*;
use crate::{NiceDisplay, Align, Cover, Rectangle2D, Layout, NodeSize, NodeSizeEvaluate, Abs};

/// A layput type that has defined position and size.
/// Is not included in the ui flow.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Window {
    /// Position of the top-left corner.
    pub pos : NodeSize<Vec2>,
    /// Size of the layout.
    pub size: NodeSize<Vec2>,
}
impl Window {
    /// Covers `100%` of the parent layout.
    pub const FULL: Window = Window { pos : NodeSize::new(), size: NodeSize::from_prc(Vec2::splat(100.0)) };
    /// A new empty Window. Has `None` size. 
    pub const EMPTY: Window = Window { pos : NodeSize::new(), size: NodeSize::new() };
    /// Creates new empty Window layout.
    pub const fn new() -> Self {
        Window {
            pos : NodeSize::new(),
            size: NodeSize::new(),
        }
    }
    /// Creates new empty Window layout.
    pub fn new_at(pos: impl Into<NodeSize<Vec2>>, size: impl Into<NodeSize<Vec2>> ) -> Self {
        let s = size.into();
        Window::new().pos(pos.into() - (s.clone() * 0.5)).size(s)
    }
    /// Replaces the position with the new value.
    pub fn pos(mut self, pos: impl Into<NodeSize<Vec2>>) -> Self {
        self.pos = pos.into();
        self
    }
    /// Replaces the x position with the new value.
    pub fn x(mut self, x: impl Into<NodeSize<f32>>) -> Self {
        self.pos.set_x(x);
        self
    }
    /// Replaces the y position with the new value.
    pub fn y(mut self, y: impl Into<NodeSize<f32>>) -> Self {
        self.pos.set_y(y);
        self
    }
    /// Replaces the size with the new value.
    pub fn size(mut self, size: impl Into<NodeSize<Vec2>>) -> Self {
        self.size = size.into();
        self
    }
    /// Replaces the width with the new value.
    pub fn width(mut self, width: impl Into<NodeSize<f32>>) -> Self {
        self.size.set_x(width);
        self
    }
    /// Replaces the height with the new value.
    pub fn height(mut self, height: impl Into<NodeSize<f32>>) -> Self {
        self.size.set_y(height);
        self
    }
    /// Computes the layout based on given parameters.
    pub(crate) fn compute(&self, parent: Rectangle2D, abs_scale: f32, font_size: f32) -> Rectangle2D {
        Rectangle2D {
            pos: parent.pos + self.pos.evaluate(abs_scale, parent.size, font_size),
            size: self.size.evaluate(abs_scale, parent.size, font_size),
        }
    }
    /// Packs the struct into Layout
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

/// A layout type that tries to fit inside a parent node.
/// Is not included in the ui flow.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Solid {
    /// Aspect ratio of the sides of the rectangular layout. `1:1 == 10:10 == 100:100`.
    pub size: NodeSize<Vec2>,
    /// Horizontal alignment within parent.
    pub align_x: Align,
    /// Vertical alignment within parent.
    pub align_y: Align,
    /// Specifies container scaling.
    pub cover: Cover,
}
impl Solid {
    /// Creates new Solid layout.
    pub fn new() -> Self {
        Solid {
            size: Abs(Vec2::ONE).into(),
            align_x: Align::CENTER,
            align_y: Align::CENTER,
            cover: Cover::Contain,
        }
    }
    /// Replaces the size with the new value.
    pub fn size(mut self, size: impl Into<NodeSize<Vec2>>) -> Self {
        self.size = size.into();
        self
    }
    /// Replaces the width with the new value.
    pub fn width(mut self, width: impl Into<NodeSize<f32>>) -> Self {
        self.size.set_x(width);
        self
    }
    /// Replaces the height with the new value.
    pub fn height(mut self, height: impl Into<NodeSize<f32>>) -> Self {
        self.size.set_y(height);
        self
    }
    /// Replaces the x alignment with the new value.
    pub fn align_x(mut self, align: Align) -> Self {
        self.align_x = align;
        self
    }
    /// Replaces the y alignment with the new value.
    pub fn align_y(mut self, align: Align) -> Self {
        self.align_y = align;
        self
    }
    /// Replaces both x & y cover values with the new value.
    pub fn cover(mut self, cover: Cover) -> Self {
        self.cover = cover;
        self
    }
    /// Computes the layout based on given parameters.
    pub(crate) fn compute(&self, parent: Rectangle2D, abs_scale: f32, font_size: f32) -> Rectangle2D {
        
        let size = self.size.evaluate(abs_scale, parent.size, font_size);

        let scale = match self.cover {
            Cover::Horizontal => parent.size.x / size.x,
            Cover::Vertical => parent.size.y / size.y,
            Cover::Contain => f32::min(parent.size.x / size.x, parent.size.y / size.y),
            Cover::Full => f32::max(parent.size.x / size.x, parent.size.y / size.y),
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
    /// Packs the struct into Layout
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
