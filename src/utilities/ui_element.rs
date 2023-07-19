use bevy::prelude::*;
use crate::prelude::*;

// ===========================================================
// === ELEMENT ===

#[derive(Component, Clone, Debug, Default)]
pub struct Element {
    pub relative: Vec2,
    pub absolute: Vec2,
    pub boundary: Vec2,
    pub scale: f32,
}

#[derive(Bundle, Clone, Debug, Default)]
pub struct ElementBundle {
    pub widget: Widget,
    pub element: Element,
    pub transform: Transform,
    pub visibility: Visibility,
    pub global_transform: GlobalTransform,
    pub computed_visibility: ComputedVisibility,
}


// ===========================================================
// === IMAGE ELEMENT ===

#[derive(Bundle, Clone, Debug, Default)]
pub struct ImageElementBundle {
    pub widget: Widget,
    pub element: Element,
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    pub transform: Transform,
    pub visibility: Visibility,
    pub global_transform: GlobalTransform,
    pub computed_visibility: ComputedVisibility,
}


// ===========================================================
// === TEXT ELEMENT ===

#[derive(Bundle, Clone, Debug, Default)]
pub struct TextElementBundle {
    pub widget: Widget,
    pub element: Element,
    pub text: Text,
    pub text_anchor: bevy::sprite::Anchor,
    pub text_2d_bounds: bevy::text::Text2dBounds,
    pub text_layout_info: bevy::text::TextLayoutInfo,
    pub transform: Transform,
    pub visibility: Visibility,
    pub global_transform: GlobalTransform,
    pub computed_visibility: ComputedVisibility,
}


pub fn text_compute_size_simple (text: &str, font_size: f32) -> Vec2 {

    const SYMBOL_WIDTH_WEIGHT: f32 = 0.8 * 0.5;
    const SYMBOL_HEIGHT_WEIGHT: f32 = 1.2 * 0.5;

    let mut width = 0.0;

    let list: Vec<&str> = text.split("/n").collect();
    for line in &list {
        let len = line.chars().count();
        if len as f32 > width {width = len as f32}
    }

    Vec2::new(width * font_size * SYMBOL_WIDTH_WEIGHT, list.len() as f32 * font_size * SYMBOL_HEIGHT_WEIGHT)
}