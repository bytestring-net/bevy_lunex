use bevy::math::Vec2Swizzles;

use crate::{import::*, Align, NodeSizeEvaluate, Sizing};

use crate::{NiceDisplay, Layout, NodeSize};



/// A simple div using the box-paradigm to encapsulate it's content
/// It's size and position is dependent on the layout flow.
/// * padding = encapsulate the inside (increases it's size)
/// * margin = encapsulate the outside (pushes other divs away from border)
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Div {
    /// Describes how the container will size itself
    pub sizing: Sizing,
    /// Minamal size the container can be.
    pub min_size: Option<NodeSize<Vec2>>,
    /// Maximal size the container can be.
    pub max_size: Option<NodeSize<Vec2>>,

    /// Padding distancing border from context.
    pub padding: NodeSize<Vec4>,
    /// Padding used for rendering. 0 by default.
    pub border: NodeSize<Vec4>,
    /// Padding distancing other divs from border.
    pub margin: NodeSize<Vec4>,

    /// Forces a line break in stack.
    pub force_break: bool,
    /// Overwrite the stack dictated alignment.
    pub align_x: Option<Align>,
    /// Overwrite the stack dictated alignment.
    pub align_y: Option<Align>,
}
impl Div {
    pub fn new() -> Self {
        Default::default()
    }
    /// Sets the sizing to the new value.
    pub fn sizing(mut self, sizing: Sizing) -> Self {
        self.sizing = sizing;
        self
    }
    /// Sets the minimal size to the new value.
    pub fn min(mut self, size: impl Into<NodeSize<Vec2>>) -> Self {
        self.min_size = Some(size.into());
        self
    }
    /// Sets the maximal size to the new value.
    pub fn max(mut self, size: impl Into<NodeSize<Vec2>>) -> Self {
        self.max_size = Some(size.into());
        self
    }
    /// Replaces the padding with the new value.
    pub fn pad(mut self, pad: impl Into<NodeSize<Vec4>>) -> Self {
        self.padding = pad.into();
        self
    }
    /// Replaces the horizontal padding with the new value.
    pub fn pad_x(mut self, pad: impl Into<NodeSize<Vec2>>) -> Self {
        let pad: NodeSize<Vec2> = pad.into();
        let val = pad.get_x();
        self.padding.set_x(val);
        self.padding.set_z(val);
        self
    }
    /// Replaces the vertical padding with the new value.
    pub fn pad_y(mut self, pad: impl Into<NodeSize<Vec2>>) -> Self {
        let pad: NodeSize<Vec2> = pad.into();
        let val = pad.get_y();
        self.padding.set_y(val);
        self.padding.set_w(val);
        self
    }
    /// Replaces the left padding with the new value.
    pub fn pad_l(mut self, pad: impl Into<NodeSize<f32>>) -> Self {
        self.padding.set_x(pad);
        self
    }
    /// Replaces the top padding with the new value.
    pub fn pad_t(mut self, pad: impl Into<NodeSize<f32>>) -> Self {
        self.padding.set_y(pad);
        self
    }
    /// Replaces the right padding with the new value.
    pub fn pad_r(mut self, pad: impl Into<NodeSize<f32>>) -> Self {
        self.padding.set_z(pad);
        self
    }
    /// Replaces the bottom padding with the new value.
    pub fn pad_b(mut self, pad: impl Into<NodeSize<f32>>) -> Self {
        self.padding.set_w(pad);
        self
    }
    /// Replaces the border with the new value.
    pub fn border(mut self, border: impl Into<NodeSize<Vec4>>) -> Self {
        self.border = border.into();
        self
    }
    /// Replaces the horizontal border with the new value.
    pub fn border_x(mut self, border: impl Into<NodeSize<Vec2>>) -> Self {
        let border: NodeSize<Vec2> = border.into();
        let val = border.get_x();
        self.border.set_x(val);
        self.border.set_z(val);
        self
    }
    /// Replaces the vertical border with the new value.
    pub fn border_y(mut self, border: impl Into<NodeSize<Vec2>>) -> Self {
        let border: NodeSize<Vec2> = border.into();
        let val = border.get_y();
        self.border.set_y(val);
        self.border.set_w(val);
        self
    }
    /// Replaces the left border with the new value.
    pub fn border_l(mut self, border: impl Into<NodeSize<f32>>) -> Self {
        self.border.set_x(border);
        self
    }
    /// Replaces the top border with the new value.
    pub fn border_t(mut self, border: impl Into<NodeSize<f32>>) -> Self {
        self.border.set_y(border);
        self
    }
    /// Replaces the right border with the new value.
    pub fn border_r(mut self, border: impl Into<NodeSize<f32>>) -> Self {
        self.border.set_z(border);
        self
    }
    /// Replaces the bottom border with the new value.
    pub fn border_b(mut self, border: impl Into<NodeSize<f32>>) -> Self {
        self.border.set_w(border);
        self
    }
    /// Replaces the margin with the new value.
    pub fn margin(mut self, margin: impl Into<NodeSize<Vec4>>) -> Self {
        self.margin = margin.into();
        self
    }
    /// Replaces the horizontal margin with the new value.
    pub fn margin_x(mut self, margin: impl Into<NodeSize<Vec2>>) -> Self {
        let margin: NodeSize<Vec2> = margin.into();
        let val = margin.get_x();
        self.margin.set_x(val);
        self.margin.set_z(val);
        self
    }
    /// Replaces the vertical margin with the new value.
    pub fn margin_y(mut self, margin: impl Into<NodeSize<Vec2>>) -> Self {
        let margin: NodeSize<Vec2> = margin.into();
        let val = margin.get_y();
        self.margin.set_y(val);
        self.margin.set_w(val);
        self
    }
    /// Replaces the left margin with the new value.
    pub fn margin_l(mut self, margin: impl Into<NodeSize<f32>>) -> Self {
        self.margin.set_x(margin);
        self
    }
    /// Replaces the top margin with the new value.
    pub fn margin_t(mut self, margin: impl Into<NodeSize<f32>>) -> Self {
        self.margin.set_y(margin);
        self
    }
    /// Replaces the right margin with the new value.
    pub fn margin_r(mut self, margin: impl Into<NodeSize<f32>>) -> Self {
        self.margin.set_z(margin);
        self
    }
    /// Replaces the bottom margin with the new value.
    pub fn margin_b(mut self, margin: impl Into<NodeSize<f32>>) -> Self {
        self.margin.set_w(margin);
        self
    }
    /// Makes any container after this start at new line
    pub fn br(mut self) -> Self {
        self.force_break = true;
        self
    }
    /// Replaces the x alignment with the new value.
    pub fn align_x(mut self, align: Align) -> Self {
        self.align_x = Some(align);
        self
    }
    /// Replaces the y alignment with the new value.
    pub fn align_y(mut self, align: Align) -> Self {
        self.align_y = Some(align);
        self
    }
    /// Packs the struct into Layout
    pub fn pack(self) -> Layout {
        self.into()
    }


    /// Computes the layout based on given parameters.
    pub(crate) fn compute_padding(&self, parent_size: Vec2, abs_scale: f32, font_size: f32) -> Vec4 {
        self.padding.evaluate(abs_scale, parent_size.xyxy(), font_size)
    }
    /// Computes the layout based on given parameters.
    pub(crate) fn compute_margin(&self, parent_size: Vec2, abs_scale: f32, font_size: f32) -> Vec4 {
        self.margin.evaluate(abs_scale, parent_size.xyxy(), font_size)
    }
    /// Computes the layout based on given parameters.
    pub(crate) fn compute_border(&self, parent_size: Vec2, abs_scale: f32, font_size: f32) -> Vec4 {
        self.border.evaluate(abs_scale, parent_size.xyxy(), font_size)
    }
    /// Computes the layout based on given parameters.
    pub(crate) fn compute_size(&self, content_size: Vec2, padding: Vec4, border: Vec4) -> Vec2 {
        Vec2 {
            x: border.x + padding.x + content_size.x + padding.z + border.z,
            y: border.y + padding.y + content_size.y + padding.w + border.w,
        }
        
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
