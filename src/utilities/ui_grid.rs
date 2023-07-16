use bevy::prelude::*;
use crate::prelude::*;

// ===========================================================
// === GRID GENERATION ===

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Grid {
    pub gap_relative: Vec2,
    pub width_relative: f32,
    pub height_relative: f32,
    pub width_padding_gap: bool,
    pub height_padding_gap: bool,
}
impl Grid {
    pub fn create (&self, system: &mut Hierarchy, path: &String, grid: &Vec<Vec<&str>>, relative: Vec2) -> Result<Widget, String>{
        let xx = grid.len();
        let yy = grid[0].len();

        for i in 0..grid.len() {
            if grid[i].len() != yy {
                return Result::Err(format!("Grid column {}(len: {}) has different length than column 0(len: {}). All columns should have the same length!", i, grid[i].len(), xx))
            }
        }
    
        let total_width = self.width_relative * xx as f32;
        let total_height = self.height_relative * yy as f32;
    
        let xi = if self.width_padding_gap == true {1.0} else {-1.0};
        let yi = if self.height_padding_gap == true {1.0} else {-1.0};
    
        let total_wgap = self.gap_relative.x * (xx as f32 + xi);
        let total_hgap = self.gap_relative.y * (yy as f32 + yi);
    
        let container_width = total_width + total_wgap;
        let container_height = total_height + total_hgap;
    
        let widget = match Widget::create(system, path, Box::Window {
            relative,
            width_relative: container_width,
            height_relative: container_height,
            ..Default::default()
        }.pack()) {
            Result::Ok (widget) => widget,
            Result::Err(message) => return Result::Err(message),
        };
    
        let width = (100.0 * total_width/container_width)/xx as f32;
        let height = (100.0 * total_height/container_height)/yy as f32;
    
        let wgap = (100.0 * total_wgap/container_width)/(xx as f32 + xi);
        let hgap = (100.0 * total_hgap/container_height)/(yy as f32 + xi);
    
        for x in 0..xx {
            for y in 0..yy {
                match Widget::create(system, &widget.end(grid[x][y]), Box::Window {
                    relative: Vec2::new(
                        width*x as f32 + wgap*x as f32 + if self.width_padding_gap == true {wgap} else {0.0},
                        height*y as f32 + hgap*y as f32 + if self.height_padding_gap == true {hgap} else {0.0},
                    ),
                    width_relative: width,
                    height_relative: height,
                    ..Default::default()
                }.pack()) {
                        Result::Ok (..) => (),
                        Result::Err(message) => return Result::Err(message),
                };
            }
        }
        Result::Ok(widget)
    }
    pub fn create_inside (&self, system: &mut Hierarchy, widget: &Widget, grid: &Vec<Vec<&str>>) -> Result<(), String>{
        let xx = grid.len();
        let yy = grid[0].len();
        
        for i in 0..grid.len() {
            if grid[i].len() != yy {
                return Result::Err(format!("Grid column {}(len: {}) has different length than column 0(len: {}). All columns should have the same length!", i, grid[i].len(), yy))
            }
        }

        let total_width = self.width_relative * xx as f32;
        let total_height = self.height_relative * yy as f32;
    
        let xi = if self.width_padding_gap == true {1.0} else {-1.0};
        let yi = if self.height_padding_gap == true {1.0} else {-1.0};
    
        let total_wgap = self.gap_relative.x * (xx as f32 + xi);
        let total_hgap = self.gap_relative.y * (yy as f32 + yi);
    
        let container_width = total_width + total_wgap;
        let container_height = total_height + total_hgap;
    
        let width = (100.0 * total_width/container_width)/xx as f32;
        let height = (100.0 * total_height/container_height)/yy as f32;
    
        let wgap = (100.0 * total_wgap/container_width)/(xx as f32 + xi);
        let hgap = (100.0 * total_hgap/container_height)/(yy as f32 + xi);
    
        for x in 0..xx {
            for y in 0..yy{
                match Widget::create(system, &widget.end(grid[x][y]), Box::Window {
                    relative: Vec2::new(
                        width*x as f32 + wgap*x as f32 + if self.width_padding_gap == true {wgap} else {0.0},
                        height*y as f32 + hgap*y as f32 + if self.height_padding_gap == true {hgap} else {0.0},
                    ),
                    width_relative: width,
                    height_relative: height,
                    ..Default::default()
                }.pack()) {
                        Result::Ok (..) => (),
                        Result::Err(message) => return Result::Err(message),
                };
            }
        }
        Result::Ok(())
    }
}