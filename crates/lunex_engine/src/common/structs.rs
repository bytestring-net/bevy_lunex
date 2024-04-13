use crate::NiceDisplay;
use crate::import::*;
use crate::NodeError;


// #==================#
// #=== ERROR TYPE ===#

/// ## Ui error
/// Error type indicating something went wrong.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum UiError {
    /// Error that occurs when something went wrong with NodeTree.
    #[error("NodeTree error: {0}")]
    NodeError(NodeError),
}
impl From<NodeError> for UiError {
    fn from(value: NodeError) -> Self {
        UiError::NodeError(value)
    }
}



/// A struct for holding a 2D rectangle data.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rectangle2D {
    pub pos : Vec2,
    pub size: Vec2,
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
impl Into<Rectangle3D> for Rectangle2D {
    fn into(self) -> Rectangle3D {
        Rectangle3D {
            pos: self.pos.extend(0.0),
            size: self.size,
            ..Default::default()
        }
    }
}


/// A struct for holding a 3D rectangle data.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rectangle3D {
    pub pos : Vec3,
    pub size: Vec2,
    pub roll: f32,
    pub yaw : f32,
    pub tilt: f32,
}
impl Into<Rectangle2D> for Rectangle3D {
    fn into(self) -> Rectangle2D {
        Rectangle2D {
            pos: self.pos.truncate(),
            size: self.size,
        }
    }
}
impl NiceDisplay for Rectangle3D {
    fn to_nicestr(&self) -> String {
        let text = format!("[pos: {} size: {}]", self.pos.to_string(), self.size.to_string());
        format!("{} {}", "Computed".bright_magenta(), text.black())
    }
}