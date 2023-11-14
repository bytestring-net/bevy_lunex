use bevy::prelude::*;

/// Allows converting a Vec2 in Bevy's coordinate system to Lunex's coordinate system.
pub trait AsLunexVec2 {
    /// This function is used for translating Vec2 from Bevy coordinate system to Lunex coordinate system.
    /// It is necessary to go through this step if you want entities to be able to interact with Lunex.
    ///
    /// Example of this is the cursor entity which has [`Transform`] component.
    /// Due to the nature of Bevy, the y+ direction is upwards instead of downwards. This is extremely counterintuitive, especially for UI.
    /// * This function will invert the Y component.
    /// * In addition it will offset the values because Bevy-Lunex always starts at 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let offset = Vec2::new(-window.size.x / 2.0, window.size.y / 2.0);
    /// let cursor_position = Vec2::new(40.0, 20.0);
    ///
    /// let inside:bool = widget.contains_position(&tree, cursor_position.as_lunex(offset))?;
    /// ```
    fn as_lunex(self, offset: Vec2) -> Vec2;
}
impl AsLunexVec2 for Vec2 {
    fn as_lunex(self, offset: Vec2) -> Vec2 {
        Vec2::new(self.x - offset.x, offset.y - self.y)
    }
}


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