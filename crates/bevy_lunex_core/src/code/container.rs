use bevy::prelude::Vec2;

// ===========================================================
// === LAYOUT VARIANTS ===

/// # Window Layout
/// Under the hood it works the exact same way as [`RelativeLayout`], but is defined in a way that makes it easier to animate.
///
/// It is defined by a **`position`**, **`width`** and **`height`**.
/// All fields come in 2 variations, *relative* and *absolute*.
///
/// Relative fields range from `0.0` to `100.0` % of parent widget.
/// Absolute fields are defined as pixels.
///
/// You are allowed to go out of bounds (-5.0, 120.0, etc..)
///
/// The final rectangle is the **sum** of these 2 values.
///
/// # Fields
/// * `absolute` = position of **top left** corner of the widget in pixels.
/// * `relative` = position of **top left** corner of the widget in % of parent widget.
/// * `width_absolute` = width of the widget in pixels.
/// * `width_relative` = width of the widget in % of parent widget.
/// * `height_absolute` = height of the widget in pixels.
/// * `height_relative` = height of the widget in % of parent widget.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowLayout {
    pub absolute: Vec2,
    pub relative: Vec2,
    pub width_absolute: f32,
    pub width_relative: f32,
    pub height_absolute: f32,
    pub height_relative: f32,
}
impl WindowLayout {
    /// Creates new window layout from default.
    pub fn new() -> WindowLayout {
        WindowLayout::default()
    }

    /// This method will package the struct into LayoutPackage for further processing.
    pub fn pack(self) -> LayoutPackage {
        LayoutPackage::Window(self)
    }

    /// This method calculates the position of the widget from this layout. As argument you supply parenting widget position and dimensions.
    pub(super) fn calculate(&self, point: Vec2, width: f32, height: f32) -> (Vec2, f32, f32) {
        let xs = width / 100.0;
        let ys = height / 100.0;
        (
            Vec2::new(
                point.x + self.absolute.x + (self.relative.x * xs),
                point.y + self.absolute.y + (self.relative.y * ys),
            ),
            self.width_absolute + (self.width_relative * xs),
            self.height_absolute + (self.height_relative * ys),
        )
    }
}
impl Default for WindowLayout {
    fn default() -> Self {
        WindowLayout {
            absolute: Vec2::default(),
            relative: Vec2::default(),
            width_absolute: 0.0,
            width_relative: 100.0,
            height_absolute: 0.0,
            height_relative: 100.0,
        }
    }
}

/// # Relative Layout
/// Under the hood it works the exact same way as [`WindowLayout`], but is defined in a way that makes it easier to define boundaries.
///
/// It is defined by 2 **`positions`**.
/// All fields come in 2 variations, *relative* and *absolute*.
///
/// Relative fields range from `0.0` to `100.0` % of parent widget.
/// Absolute fields are defined as pixels.
///
/// You are allowed to go out of bounds (-5.0, 120.0, etc..)
///
/// The final rectangle is the **sum** of these 2 values.
///
/// # Fields
/// * `absolute_1` = position of **top left** corner of the widget in pixels.
/// * `absolute_2` = position of **bottom right** corner of the widget in pixels.
/// * `relative_1` = position of **top left** corner of the widget in % of parent widget.
/// * `relative_2` = position of **bottom right** corner of the widget in % of parent widget.
#[derive(Clone, Debug, PartialEq)]
pub struct RelativeLayout {
    pub absolute_1: Vec2,
    pub absolute_2: Vec2,
    pub relative_1: Vec2,
    pub relative_2: Vec2,
}
impl RelativeLayout {
    /// Creates new relative layout from default.
    pub fn new() -> RelativeLayout {
        RelativeLayout::default()
    }

    /// This method will package the struct into LayoutPackage for further processing.
    pub fn pack(self) -> LayoutPackage {
        LayoutPackage::Relative(self)
    }

    /// This method calculates the position of the widget from this layout. As argument you supply parenting widget position and dimensions.
    pub(super) fn calculate(&self, point: Vec2, width: f32, height: f32) -> (Vec2, f32, f32) {
        let xs = width / 100.0;
        let ys = height / 100.0;
        let v1 = Vec2::new(
            point.x + self.absolute_1.x + (self.relative_1.x * xs),
            point.y + self.absolute_1.y + (self.relative_1.y * ys),
        );
        let v2 = Vec2::new(
            point.x + self.absolute_2.x + (self.relative_2.x * xs),
            point.y + self.absolute_2.y + (self.relative_2.y * ys),
        );
        let _width = v2.x - v1.x;
        let _height = v2.y - v1.y;
        (v1, _width, _height)
    }
}
impl Default for RelativeLayout {
    fn default() -> Self {
        RelativeLayout {
            absolute_1: Vec2::default(),
            absolute_2: Vec2::default(),
            relative_1: Vec2::default(),
            relative_2: Vec2::new(100.0, 100.0),
        }
    }
}

/// # Solid Layout
/// This is a special layout that will **ALWAYS** keep size ratio.
///
/// It is defined by a size ratio. Meaning that `10.0/10.0` is the same as `1000.0/1000.0`.
/// Both will be perfect square.
///
/// Default scaling is `Fit`, meaning that the widget will always be **INSIDE** the parenting widget and will **NEVER** leave the bounds.
/// In most cases you want to use this scaling.
///
/// Scaling `Fill` means that the parenting container will be **COVERED** by this widget and the bounds will **OVERFLOW**. But the size ratio will stay the same.
/// This is useful for example when adding **background**. This scaling will ensure the background covers 100% of the parenting widget.
///
/// Always put **images** inside a solid widget so that no matter the window size, no images will be deformed.
///
/// Anchoring ensures that widget will try to move to that side if there is space.
/// It is hard to explain, so just experiment with it, you will quickly understand what it does.
///
/// # Fields
/// * `width` = width size ratio.
/// * `height` = height size ratio.
/// * `horizontal_anchor` = where should it align on x-axis, range from -1.0 to 1.0, default is 0.0.
/// * `vertical_anchor` = where should it align on y-axis, range from -1.0 to 1.0, default is 0.0.
/// * `scaling` = should the widget **fit** the parenting container or **fill** the parenting container.
#[derive(Clone, Debug, PartialEq)]
pub struct SolidLayout {
    pub width: f32,
    pub height: f32,
    pub horizontal_anchor: f32,
    pub vertical_anchor: f32,
    pub scaling: SolidScale,
}
impl SolidLayout {
    /// Creates new solid layout from default.
    pub fn new() -> SolidLayout {
        SolidLayout::default()
    }

    /// This method will package the struct into LayoutPackage for further processing.
    pub fn pack(self) -> LayoutPackage {
        LayoutPackage::Solid(self)
    }

    /// This method calculates the position of the widget from this layout. As argument you supply parenting widget position and dimensions.
    pub(super) fn calculate(&self, point: Vec2, width: f32, height: f32) -> (Vec2, f32, f32) {
        let scale = match self.scaling {
            SolidScale::Fill => f32::max(width / self.width, height / self.height),
            SolidScale::Fit => f32::min(width / self.width, height / self.height),
        };

        let center = [point.x + width / 2.0, point.y + height / 2.0];
        let vanilla_width = self.width * scale;
        let vanilla_height = self.height * scale;
        let vanilla_point = [
            center[0] - vanilla_width / 2.0,
            center[1] - vanilla_height / 2.0,
        ];

        (
            Vec2::new(
                vanilla_point[0] + (vanilla_point[0] - point[0]) * self.horizontal_anchor,
                vanilla_point[1] + (vanilla_point[1] - point[1]) * self.vertical_anchor,
            ),
            vanilla_width,
            vanilla_height,
        )
    }

    /// Solid layout set to a custom width
    pub fn with_width(mut self, width: f32) -> SolidLayout {
        self.width = width;
        self
    }

    /// Solid layout set to a custom height
    pub fn with_height(mut self, height: f32) -> SolidLayout {
        self.height = height;
        self
    }

    /// Solid layout set to a custom horizontal_anchor
    pub fn with_horizontal_anchor(mut self, horizontal_anchor: f32) -> SolidLayout {
        self.horizontal_anchor = horizontal_anchor;
        self
    }

    /// Solid layout set to a custom vertical_anchor
    pub fn with_vertical_anchor(mut self, vertical_anchor: f32) -> SolidLayout {
        self.vertical_anchor = vertical_anchor;
        self
    }

    /// Solid layout set to a custom scaling
    pub fn with_scaling(mut self, scaling: SolidScale) -> SolidLayout {
        self.scaling = scaling;
        self
    }
}
impl Default for SolidLayout {
    fn default() -> Self {
        SolidLayout {
            width: 1.0,
            height: 1.0,
            horizontal_anchor: 0.0,
            vertical_anchor: 0.0,
            scaling: SolidScale::default(),
        }
    }
}

/// # Solid Scale
/// Enum for 2 options on how to scale [`SolidLayout`] container.
/// # Variants
/// * `Fit` = Fit the parent container.
/// * `Fill` = Fill the parent contaier.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum SolidScale {
    #[default]
    Fit,
    Fill,
}


// ===========================================================
// === LAYOUT PACKAGE AND POSITION ===

/// # Layout Package
/// Enum holding one of the possible layouts widget can have.
///
/// It is necessary to wrap new layouts into this enum for further processing.
/// # Types
/// * [`WindowLayout`]
/// * [`RelativeLayout`]
/// * [`SolidLayout`]
#[derive(Clone, Debug, PartialEq)]
pub enum LayoutPackage {
    Window(WindowLayout),
    Relative(RelativeLayout),
    Solid(SolidLayout),
}
impl LayoutPackage {
    /// Unwrap package into `&WindowLayout`, panic if this is not window.
    pub fn expect_window_ref(&self) -> &WindowLayout {
        match self {
            LayoutPackage::Window(window) => window,
            _ => panic!("Window layout was expected!"),
        }
    }

    /// Unwrap package into `&RelativeLayout`, panic if this is not window.
    pub fn expect_relative_ref(&self) -> &RelativeLayout {
        match self {
            LayoutPackage::Relative(relative) => relative,
            _ => panic!("Relative layout was expected!"),
        }
    }

    /// Unwrap package into `&SolidLayout`, panic if this is not window.
    pub fn expect_solid_ref(&self) -> &SolidLayout {
        match self {
            LayoutPackage::Solid(solid) => solid,
            _ => panic!("Solid layout was expected!"),
        }
    }

    /// Unwrap package into `mut &WindowLayout`, panic if this is not window.
    pub fn expect_window_mut(&mut self) -> &mut WindowLayout {
        match self {
            LayoutPackage::Window(window) => window,
            _ => panic!("Window layout was expected!"),
        }
    }

    /// Unwrap package into `mut &RelativeLayout`, panic if this is not window.
    pub fn expect_relative_mut(&mut self) -> &mut RelativeLayout {
        match self {
            LayoutPackage::Relative(relative) => relative,
            _ => panic!("Relative layout was expected!"),
        }
    }

    /// Unwrap package into `mut &SolidLayout`, panic if this is not window.
    pub fn expect_solid_mut(&mut self) -> &mut SolidLayout {
        match self {
            LayoutPackage::Solid(solid) => solid,
            _ => panic!("Solid layout was expected!"),
        }
    }
}
impl Default for LayoutPackage {
    fn default() -> Self {
        LayoutPackage::Relative(RelativeLayout::default())
    }
}

/// # Position
/// This struct holds the dimensions of the widget, they are updated every step, changing this means nothing.
/// It is meant as read only.
/// # Fields
/// * `point_1` = top left corner
/// * `point_2` = bottom right corner
/// * `width` = width of the widget
/// * `height` = height of the widget
/// * `depth` = depth
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Position {
    pub point_1: Vec2,
    pub point_2: Vec2,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}
impl Position {
    /// Returns cloned [`Position`], but with inverted Y coordinate, this translation is required so Bevy (Y is up) can properly render Lunex (Y is down).
    pub fn invert_y(&self) -> Position {
        Position {
            point_1: Vec2::new(self.point_1.x, -self.point_1.y),
            point_2: Vec2::new(self.point_2.x, -self.point_2.y),
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }

    /// Returns a position from a custom relative point on this widget.
    pub fn get_pos(&self, relative: Vec2) -> Vec2 {
        Vec2::new(
            self.point_1.x + self.width * relative.x / 100.0,
            self.point_1.y + self.height * relative.y / 100.0,
        )
    }
    
    /// Returns a position from a custom relative point on this widget, but Y is inverted due to the same reason as `invert_y` method.
    pub fn get_pos_y_inverted(&self, relative: Vec2) -> Vec2 {
        Vec2::new(
            self.point_1.x + self.width * relative.x / 100.0,
            self.point_1.y + self.height * -relative.y / 100.0,
        )
    }
}


// ===========================================================
// === CONTAINER STRUCT ===

/// # Container
/// This struct is responsible for all the positioning of the widget.
/// Through this struct and its methods you can interact with widgets position.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Container {
    position_cached: Position,
    position_layout: LayoutPackage,
}
impl Container {
    /// Creates a new container
    pub(super) fn new() -> Container {
        Container {
            position_cached: Position::default(),
            position_layout: LayoutPackage::default(),
        }
    }

    /// Calculates the layout and saves the output to `position_cached` field
    pub(super) fn calculate(&mut self, point: Vec2, width: f32, height: f32) {
        let values = match &self.position_layout {
            LayoutPackage::Window(container) => container.calculate(point, width, height),
            LayoutPackage::Relative(container) => container.calculate(point, width, height),
            LayoutPackage::Solid(container) => container.calculate(point, width, height),
        };
        self.position_cached.point_1 = values.0;
        self.position_cached.width = values.1;
        self.position_cached.height = values.2;
        self.position_cached.point_2 = Vec2::new(
            self.position_cached.point_1.x + self.position_cached.width,
            self.position_cached.point_1.y + self.position_cached.height,
        );
    }

    /// Returns a read only reference to a container position
    pub fn position_get(&self) -> &Position {
        &self.position_cached
    }

    /// Set a new layout to a container
    pub fn layout_set(&mut self, position: LayoutPackage) {
        self.position_layout = position;
    }

    /// Returns a read only reference to a layout
    pub fn layout_get(&self) -> &LayoutPackage {
        &self.position_layout
    }

    /// Returns mutable reference to a layout
    pub fn layout_get_mut(&mut self) -> &mut LayoutPackage {
        &mut self.position_layout
    }
}