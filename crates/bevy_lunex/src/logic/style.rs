use crate::*;


/// Color lerping functionality
pub trait LerpColor {
    fn lerp(&self, color: Color, value: f32) -> Color;
}
impl LerpColor for Color {
    fn lerp(&self, color: Color, value: f32) -> Color {
        let c1: Hsla = (*self).into();
        let c2: Hsla = color.into();
        Color::hsla(c1.hue.lerp(c2.hue, value), c1.saturation.lerp(c2.saturation, value), c1.lightness.lerp(c2.lightness, value), c1.alpha.lerp(c2.alpha, value))
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
