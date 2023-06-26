#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;


//===========================================================================


#[derive(Clone, Debug, PartialEq, Default)]
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
        Window {
            absolute: Vec2 { x: 0.0, y: 0.0 },
            relative: Vec2 { x: 0.0, y: 0.0 },
            width_absolute: 0.0,
            width_relative: 0.0,
            height_absolute: 0.0,
            height_relative: 0.0,
        }
    }
    pub fn wrap (self) -> PositionLayout {
        PositionLayout::Window(self)
    }
    fn calculate (&self, point: Vec2, width: f32, height: f32) -> (Vec2, f32, f32) {
        let xs = width / 100.0;
        let ys = height / 100.0;
        (
            Vec2 {x: point.x + self.absolute.x + (self.relative.x * xs), y: point.y + self.absolute.y + (self.relative.y * ys)},
            self.width_absolute + (self.width_relative * xs),
            self.height_absolute + (self.height_relative * ys),
        )
    }
}


#[derive(Clone, Debug, PartialEq, Default)]
pub struct Relative {
    pub absolute_1: Vec2,
    pub absolute_2: Vec2,
    pub relative_1: Vec2,
    pub relative_2: Vec2,
}
impl Relative {
    pub fn new () -> Relative {
        Relative {
            absolute_1: Vec2 { x: 0.0, y: 0.0 },
            absolute_2: Vec2 { x: 0.0, y: 0.0 },
            relative_1: Vec2 { x: 0.0, y: 0.0 },
            relative_2: Vec2 { x: 0.0, y: 0.0 },
        }
    }
    pub fn wrap (self) -> PositionLayout {
        PositionLayout::Relative(self)
    }
    fn calculate (&self, point: Vec2, width: f32, height: f32) -> [Vec2; 2] {
        let xs = width / 100.0;
        let ys = height / 100.0;
        [
            Vec2 {x: point.x + self.absolute_1.x + (self.relative_1.x * xs), y: point.y + self.absolute_1.y + (self.relative_1.y * ys)},
            Vec2 {x: point.x + self.absolute_2.x + (self.relative_2.x * xs), y: point.y + self.absolute_2.y + (self.relative_2.y * ys)},
        ]
    }
}


#[derive(Clone, Debug, PartialEq, Default)]
pub struct Solid {
    pub width: u32,
    pub height: u32,
    pub horizontal_anchor: f32,     // (-1.0 to 1.0)
    pub vertical_anchor: f32,       // (-1.0 to 1.0)
    pub scaling: Scale,
}
impl Solid {
    pub fn new () -> Solid {
        Solid {
            width: 0,
            height: 0,
            horizontal_anchor: 0.0,
            vertical_anchor: 0.0,
            scaling: Scale::Fit,
        }
    }
    pub fn wrap (self) -> PositionLayout {
        PositionLayout::Solid(self)
    }
    fn calculate (&self, point: Vec2, width: f32, height: f32) -> (Vec2, f32, f32) {
        let scale = match self.scaling {
            Scale::Fill => f32::max(width/self.width as f32, height/self.height as f32),
            Scale::Fit => f32::min(width/self.width as f32, height/self.height as f32),
        };

        let center = [point.x + width/2.0, point.y + height/2.0];
        let vanilla_width = self.width as f32*scale;
        let vanilla_height = self.height as f32*scale;
        let vanilla_point = [center[0] - vanilla_width/2.0, center[1] - vanilla_height/2.0];

        (
            Vec2 {x: (vanilla_point[0] + (vanilla_point[0] - point[0])*self.horizontal_anchor), y: (vanilla_point[1] + (vanilla_point[1] - point[1])*self.vertical_anchor)},
            vanilla_width,
            vanilla_height,
        )
    }
}


//===========================================================================


#[derive(Clone, Debug, PartialEq, Default)]
pub enum Scale {
    #[default]
    Fit,
    Fill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum PositionLayout {
    Window (Window),
    Relative (Relative),
    Solid (Solid),
}
impl PositionLayout {
    pub fn expect_window_ref (&self) -> &Window {
        match self {
            PositionLayout::Window (window) => window,
            PositionLayout::Relative(..) => panic!("Layout window expected!"),
            PositionLayout::Solid(..) => panic!("Layout window expected!"),
        }
    }
    pub fn expect_relative_ref (&self) -> &Relative {
        match self {
            PositionLayout::Window (..) => panic!("Layout relative expected!"),
            PositionLayout::Relative(relative) => relative,
            PositionLayout::Solid(..) => panic!("Layout relative expected!"),
        }
    }
    pub fn expect_solid_ref (&self) -> &Solid {
        match self {
            PositionLayout::Window (..) => panic!("Layout solid expected!"),
            PositionLayout::Relative(..) => panic!("Layout solid expected!"),
            PositionLayout::Solid(solid) => solid,
        }
    }
    pub fn expect_window_mut (&mut self) -> &mut Window {
        match self {
            PositionLayout::Window (window) => window,
            PositionLayout::Relative(..) => panic!("Layout window expected!"),
            PositionLayout::Solid(..) => panic!("Layout window expected!"),
        }
    }
    pub fn expect_relative_mut (&mut self) -> &mut Relative {
        match self {
            PositionLayout::Window (..) => panic!("Layout relative expected!"),
            PositionLayout::Relative(relative) => relative,
            PositionLayout::Solid(..) => panic!("Layout relative expected!"),
        }
    }
    pub fn expect_solid_mut (&mut self) -> &mut Solid {
        match self {
            PositionLayout::Window (..) => panic!("Layout solid expected!"),
            PositionLayout::Relative(..) => panic!("Layout solid expected!"),
            PositionLayout::Solid(solid) => solid,
        }
    }
    pub fn expect_window (self) -> Window {
        match self {
            PositionLayout::Window (window) => window,
            PositionLayout::Relative(..) => panic!("Layout window expected!"),
            PositionLayout::Solid(..) => panic!("Layout window expected!"),
        }
    }
    pub fn expect_relative (self) -> Relative {
        match self {
            PositionLayout::Window (..) => panic!("Layout relative expected!"),
            PositionLayout::Relative(relative) => relative,
            PositionLayout::Solid(..) => panic!("Layout relative expected!"),
        }
    }
    pub fn expect_solid (self) -> Solid {
        match self {
            PositionLayout::Window (..) => panic!("Layout solid expected!"),
            PositionLayout::Relative(..) => panic!("Layout solid expected!"),
            PositionLayout::Solid(solid) => solid,
        }
    }
}
impl Default for PositionLayout {
    fn default() -> Self {
        PositionLayout::Relative(Relative {..Default::default()})
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Position {
    pub point_1: Vec2,
    pub point_2: Vec2,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}


//===========================================================================



#[derive(Clone, Debug, PartialEq, Default)]
pub struct Container {
    position_cached: Position,
    position_layout: PositionLayout,
}
impl Container {
    pub fn new () -> Container {
        Container {
            position_cached: Position::default(),
            position_layout: PositionLayout::default(),
        }
    }
    pub fn update (&mut self, point: Vec2, width: f32, height: f32) {
        match &self.position_layout {
            PositionLayout::Window(container) => {
                let values = container.calculate(point, width, height);
                self.position_cached.point_1 = values.0;
                self.position_cached.width = values.1;
                self.position_cached.height = values.2;
                self.position_cached.point_2 = Vec2 {x: self.position_cached.point_1.x + self.position_cached.width, y: self.position_cached.point_1.y + self.position_cached.height};
            },
            PositionLayout::Relative(container) => {
                let values = container.calculate(point, width, height);
                self.position_cached.point_1 = values[0];
                self.position_cached.width = values[1][0] - values[0][0];
                self.position_cached.height = values[1][1] - values[0][1];
                self.position_cached.point_2 = Vec2 {x: self.position_cached.point_1.x + self.position_cached.width, y: self.position_cached.point_1.y + self.position_cached.height};
            },
            PositionLayout::Solid(container) => {
                let values = container.calculate(point, width, height);
                self.position_cached.point_1 = values.0;
                self.position_cached.width = values.1;
                self.position_cached.height = values.2;
                self.position_cached.point_2 = Vec2 {x: self.position_cached.point_1.x + self.position_cached.width, y: self.position_cached.point_1.y + self.position_cached.height};
            },
        }   
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
    pub fn position_layout_set (&mut self, position: PositionLayout) {
        self.position_layout = position;
    }
    pub fn position_layout_get (&self) -> &PositionLayout {
        &self.position_layout
    }
    pub fn position_layout_get_mut (&mut self) -> &mut PositionLayout {
        &mut self.position_layout
    }
}
