use bevy::prelude::*;

/// Inverts the X coordinate
pub trait InvertX<T> {
    /// This function inverts the X coordinate
    fn invert_x(self) -> T;
}
/// Inverts the Y coordinate
pub trait InvertY<T> {
    /// This function inverts the Y coordinate
    fn invert_y(self) -> T;
}
/// Inverts the XY coordinate
pub trait InvertXY<T> {
    /// This function inverts the XY coordinate
    fn invert_xy(self) -> T;
}

impl InvertX<Vec2> for Vec2 {
    fn invert_x(mut self) -> Vec2 {
        self.x *= -1.0;
        self
    }
}
impl InvertY<Vec2> for Vec2 {
    fn invert_y(mut self) -> Vec2 {
        self.y *= -1.0;
        self
    }
}
impl InvertXY<Vec2> for Vec2 {
    fn invert_xy(mut self) -> Vec2 {
        self.x *= -1.0;
        self.y *= -1.0;
        self
    }
}