use crate::import::*;

/// ## Nice display
/// Trait for types to implement so they can be nicely printed in terminal.
/// Used by [`crate::NodeDisplayTrait::tree`] for displaying custom node data.
pub trait NiceDisplay {
    /// Used when you want to convert type into nicely formatted string
    /// for displaying in the terminal. Only important data for the user should be shown.
    /// Use `colorise` crate for nice colors.
    fn to_nicestr(&self) -> String;
}


/// ## Y invert
/// Trait for implementing Y value invert for Glam types due to inverted coordinate system between Ui and Bevy.
pub trait YInvert {
    /// Multiplies the Y value by -1
    fn invert_y(self) -> Self;
}
impl YInvert for Vec2 {
    fn invert_y(mut self) -> Self {
        self.y *= -1.0;
        self
    }
}
impl YInvert for Vec3 {
    fn invert_y(mut self) -> Self {
        self.y *= -1.0;
        self
    }
}
impl YInvert for Vec4 {
    fn invert_y(mut self) -> Self {
        self.y *= -1.0;
        self
    }
}
