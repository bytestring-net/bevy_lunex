#![allow(non_snake_case)]
use bevy::prelude::Vec2;

// ===========================================================
// === DIFFERENT LAYOUT VARIATIONS ===

pub mod Box {
    use bevy::prelude::Vec2;
    use super::{Layout, SolidScale};

    #[derive(Clone, Debug, PartialEq)]
    pub struct Window {
        pub absolute: Vec2,
        pub relative: Vec2,
        pub width_absolute: f32,
        pub width_relative: f32,
        pub height_absolute: f32,
        pub height_relative: f32,
    }
    impl Window {
        pub fn new () -> Window {
            Window::default()
        }
        pub fn pack (self) -> Layout {
            Layout::Window(self)
        }
        pub (in super) fn calculate (&self, point: Vec2, width: f32, height: f32) -> (Vec2, f32, f32) {
            let xs = width / 100.0;
            let ys = height / 100.0;
            (
                Vec2::new(point.x + self.absolute.x + (self.relative.x * xs), point.y + self.absolute.y + (self.relative.y * ys)),
                self.width_absolute + (self.width_relative * xs),
                self.height_absolute + (self.height_relative * ys),
            )
        }
    }
    impl Default for Window  {
        fn default() -> Self {
            Window {
                absolute: Vec2::default(),
                relative: Vec2::default(),
                width_absolute: 0.0,
                width_relative: 100.0,
                height_absolute: 0.0,
                height_relative: 100.0,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Relative {
        pub absolute_1: Vec2,
        pub absolute_2: Vec2,
        pub relative_1: Vec2,
        pub relative_2: Vec2,
    }
    impl Relative {
        pub fn new () -> Relative {
            Relative::default()
        }
        pub fn pack (self) -> Layout {
            Layout::Relative(self)
        }
        pub (in super) fn calculate (&self, point: Vec2, width: f32, height: f32) -> (Vec2, f32, f32) {
            let xs = width / 100.0;
            let ys = height / 100.0;
            let v1 = Vec2::new(point.x + self.absolute_1.x + (self.relative_1.x * xs), point.y + self.absolute_1.y + (self.relative_1.y * ys));
            let v2 = Vec2::new(point.x + self.absolute_2.x + (self.relative_2.x * xs), point.y + self.absolute_2.y + (self.relative_2.y * ys));
            let _width = v2.x - v1.x;
            let _height = v2.y - v1.y;
            (v1, _width, _width)
        }
    }
    impl Default for Relative  {
        fn default() -> Self {
            Relative {
                absolute_1: Vec2::default(),
                absolute_2: Vec2::default(),
                relative_1: Vec2::default(),
                relative_2: Vec2::new(100.0, 100.0),
            }
        }
    }


    #[derive(Clone, Debug, PartialEq)]
    pub struct Solid {
        pub width: f32,
        pub height: f32,
        pub horizontal_anchor: f32,     // (-1.0 to 1.0)
        pub vertical_anchor: f32,       // (-1.0 to 1.0)
        pub scaling: SolidScale,
    }
    impl Solid {
        pub fn new () -> Solid {
            Solid::default()
        }
        pub fn pack (self) -> Layout {
            Layout::Solid(self)
        }
        pub (in super) fn calculate (&self, point: Vec2, width: f32, height: f32) -> (Vec2, f32, f32) {
            let scale = match self.scaling {
                SolidScale::Fill => f32::max(width/width, height/height),
                SolidScale::Fit => f32::min(width/width, height/height),
            };

            let center = [point.x + width/2.0, point.y + height/2.0];
            let vanilla_width = width*scale;
            let vanilla_height = height*scale;
            let vanilla_point = [center[0] - vanilla_width/2.0, center[1] - vanilla_height/2.0];

            (
                Vec2::new(vanilla_point[0] + (vanilla_point[0] - point[0])*self.horizontal_anchor, vanilla_point[1] + (vanilla_point[1] - point[1])*self.vertical_anchor),
                vanilla_width,
                vanilla_height,
            )
        }
    }
    impl Default for Solid  {
        fn default() -> Self {
            Solid {
                width: 1.0,
                height: 1.0,
                horizontal_anchor: 0.0,
                vertical_anchor: 0.0,
                scaling: SolidScale::default()
            }
        }
    }

}
#[derive(Clone, Debug, PartialEq, Default)]
pub enum SolidScale {
    #[default]
    Fit,
    Fill,
}


// ===========================================================
// === COMMON ENUM AND STRUCT WRAPS ===

#[derive(Clone, Debug, PartialEq)]
pub enum Layout {
    Window (Box::Window),
    Relative (Box::Relative),
    Solid (Box::Solid),
}
impl Layout {
    pub fn expect_window_ref (&self) -> &Box::Window {
        match self {
            Layout::Window (window) => window,
            Layout::Relative(..) => panic!("Layout window expected!"),
            Layout::Solid(..) => panic!("Layout window expected!"),
        }
    }
    pub fn expect_relative_ref (&self) -> &Box::Relative {
        match self {
            Layout::Window (..) => panic!("Layout relative expected!"),
            Layout::Relative(relative) => relative,
            Layout::Solid(..) => panic!("Layout relative expected!"),
        }
    }
    pub fn expect_solid_ref (&self) -> &Box::Solid {
        match self {
            Layout::Window (..) => panic!("Layout solid expected!"),
            Layout::Relative(..) => panic!("Layout solid expected!"),
            Layout::Solid(solid) => solid,
        }
    }
    pub fn expect_window_mut (&mut self) -> &mut Box::Window {
        match self {
            Layout::Window (window) => window,
            Layout::Relative(..) => panic!("Layout window expected!"),
            Layout::Solid(..) => panic!("Layout window expected!"),
        }
    }
    pub fn expect_relative_mut (&mut self) -> &mut Box::Relative {
        match self {
            Layout::Window (..) => panic!("Layout relative expected!"),
            Layout::Relative(relative) => relative,
            Layout::Solid(..) => panic!("Layout relative expected!"),
        }
    }
    pub fn expect_solid_mut (&mut self) -> &mut Box::Solid {
        match self {
            Layout::Window (..) => panic!("Layout solid expected!"),
            Layout::Relative(..) => panic!("Layout solid expected!"),
            Layout::Solid(solid) => solid,
        }
    }
    pub fn expect_window (self) -> Box::Window {
        match self {
            Layout::Window (window) => window,
            Layout::Relative(..) => panic!("Layout window expected!"),
            Layout::Solid(..) => panic!("Layout window expected!"),
        }
    }
    pub fn expect_relative (self) -> Box::Relative {
        match self {
            Layout::Window (..) => panic!("Layout relative expected!"),
            Layout::Relative(relative) => relative,
            Layout::Solid(..) => panic!("Layout relative expected!"),
        }
    }
    pub fn expect_solid (self) -> Box::Solid {
        match self {
            Layout::Window (..) => panic!("Layout solid expected!"),
            Layout::Relative(..) => panic!("Layout solid expected!"),
            Layout::Solid(solid) => solid,
        }
    }
}
impl Default for Layout {
    fn default() -> Self {
        Layout::Relative(Box::Relative {..Default::default()})
    }
}

    /// ### Element
    /// Struct holding all necessary information for binding an entity to a [`Widget`].
    /// ### Fields
    /// * `relative` = position in % relative to the widget.
    /// * `absolute` = position in pixels, always the same.
    /// * `boundary` = width and height, for example image dimensions or text size. 
    /// * `scale` = size of the element in % of parent widget.
    /// * `depth` = local depth of the element, starts at 0.0, usefull when you have 2 elements overlapping (image and text)
    /// * `width` = optional, will force the width of the element in % of parent widget.
    /// * `height` = optional, will force the height of the element in % of parent widget.

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Position {
    pub point_1: Vec2,
    pub point_2: Vec2,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}
impl Position {
    pub fn invert_y(&self) -> Position {
        Position {
            point_1: Vec2::new(self.point_1.x, -self.point_1.y),
            point_2: Vec2::new(self.point_2.x, -self.point_2.y),
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
    pub fn get_pos(&self, relative: Vec2) -> Vec2 {
        Vec2::new( self.point_1.x + self.width*relative.x/100.0, self.point_1.y + self.height*relative.y/100.0)
    }
    pub fn get_pos_y_inverted(&self, relative: Vec2) -> Vec2 {
        Vec2::new( self.point_1.x + self.width*relative.x/100.0, self.point_1.y + self.height*-relative.y/100.0)
    }
}


// ===========================================================
// === MAIN CONTAINER STRUCT ===

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Container {
    position_cached: Position,
    position_layout: Layout,
}
impl Container {
    pub fn new () -> Container {
        Container {
            position_cached: Position::default(),
            position_layout: Layout::default(),
        }
    }
    pub fn update (&mut self, point: Vec2, width: f32, height: f32) {
        let values = match &self.position_layout {
            Layout::Window(container) => container.calculate(point, width, height),
            Layout::Relative(container) => container.calculate(point, width, height),
            Layout::Solid(container) => container.calculate(point, width, height),
        };
        self.position_cached.point_1 = values.0;
        self.position_cached.width = values.1;
        self.position_cached.height = values.2;
        self.position_cached.point_2 = Vec2::new(self.position_cached.point_1.x + self.position_cached.width, self.position_cached.point_1.y + self.position_cached.height);
    }
    pub fn position_set (&mut self, position: Position) {
        self.position_cached = position;
    }
    pub fn position_get (&self) -> &Position {
        &self.position_cached
    }
    pub fn position_get_mut (&mut self) -> &mut Position {
        &mut self.position_cached
    }
    pub fn position_layout_set (&mut self, position: Layout) {
        self.position_layout = position;
    }
    pub fn position_layout_get (&self) -> &Layout {
        &self.position_layout
    }
    pub fn position_layout_get_mut (&mut self) -> &mut Layout {
        &mut self.position_layout
    }
}
