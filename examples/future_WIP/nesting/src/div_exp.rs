use bevy::prelude::{Vec2, Vec4};


pub enum Placement {
    Flow {
        padding: Vec4,
        align: f32,
    },
    Fixed
}

pub enum DivClass {
    LineAlign (LineAlign),
    Box,
}
pub struct LineAlign (f32);

pub struct Div {
    pub class : DivClass,
    pub placement: Placement,
    content_size: Vec2,
}
impl Div {
    pub fn test_new(size: Vec2) -> Div {
        Div {
            class: DivClass::Box,
            placement: Placement::Flow { padding: Vec4::ZERO, align: 0. },
            content_size: size,
        }
    }
    pub fn size(&self) -> Vec2 {
        match self.placement {
            Placement::Fixed => unreachable!(),
            Placement::Flow { padding, align: _ } => {
                Vec2::new(
                    self.content_size.x + padding.x + padding.z,
                    self.content_size.y + padding.y + padding.w,
                )
            }
        }
    }
}

pub struct DivRect {
    pub point: Vec2,
    pub width: f32,
    pub height: f32,
}


pub struct DivManager {
    output: Vec<DivRect>,
    layout: Vec<Div>,
}
impl DivManager {
    pub fn compute(&mut self, boundary: DivRect) {

        // Prepare for line processing
        self.output.clear();

        // Establish defaults for line processing
        let mut line_align = -1.0;

        // Start line processing
        for div in &self.layout {
            match div.placement {
                Placement::Fixed => {},
                Placement::Flow { padding:div_padding, align:div_align } => {
                    match &div.class {
                        DivClass::LineAlign (_line_align) => {
                            line_align = _line_align.0;
                        },
                        DivClass::Box => {
        
                            let div_size = div.size();
        
                            let xpoint = boundary.x + 0. + boundary.width/2. + boundary.width/2. * line_align;
                            let ypoint = boundary.y + 0. + boundary.height/2. + boundary.height/2. * div_align;
        
                            let div_rect = DivRect {
                                point: Vec2::new(xpoint, ypoint),
                                width: div_size.x,
                                height: div_size.y,
                            };
                            
                            self.output.push(div_rect)
        
                        },
                    }
                },
            }
        }
    }
}