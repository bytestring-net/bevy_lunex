use bevy::prelude::*;
use mathio::*;

pub fn tween_color_rgba(color1: Color, color2: Color, slide: f32) -> Color {
    Color::rgba(
        tween(color1.r(), color2.r(), slide),
        tween(color1.g(), color2.g(), slide),
        tween(color1.b(), color2.b(), slide),
        tween(color1.a(), color2.a(), slide),
    )
}
/*
pub fn tween_color_hsla_short(color1: Color, color2: Color, slider: f32) -> Color {
    Color::hsla(
        periodical_tween_short(360.0, color1.h(), color2.h(), slider),
        tween(color1.s(), color2.s(), slider),
        tween(color1.l(), color2.l(), slider),
        tween(color1.a(), color2.a(), slider),
    )
}

pub fn tween_color_hsla_long(color1: Color, color2: Color, slider: f32) -> Color {
    Color::hsla(
        periodical_tween_long(360.0, color1.h(), color2.h(), slider),
        tween(color1.s(), color2.s(), slider),
        tween(color1.l(), color2.l(), slider),
        tween(color1.a(), color2.a(), slider),
    )
}*/
