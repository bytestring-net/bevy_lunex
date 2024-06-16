use crate::*;


/// Color lerping functionality
pub trait LerpColor {
    fn lerp(&self, color: Color, value: f32) -> Color;
}
impl LerpColor for Color {
    fn lerp(&self, color: Color, value: f32) -> Color {
        let c1 = self.hsla_to_vec4();
        let c2 = color.hsla_to_vec4();
        Color::hsla(c1.x.lerp(c2.x, value), c1.y.lerp(c2.y, value), c1.z.lerp(c2.z, value), c1.w.lerp(c2.w, value))
    }
}


// #==============#
// #=== PLUGIN ===#

pub struct StylePlugin;
impl Plugin for StylePlugin {
    fn build(&self, _app: &mut App) {
        //app;
    }
}
