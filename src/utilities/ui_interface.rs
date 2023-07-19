use bevy::prelude::*;
use crate::prelude::*;


// ===========================================================
// === SPAWN COMMADS ===

#[derive(Clone, Debug)]
pub struct TextParams {
    pub style: TextStyle,
    pub scale: f32,
    pub relative: Vec2,
    pub absolute: Vec2,
    pub alignment: TextAlignment,
    pub anchor: bevy::sprite::Anchor,
}
impl Default for TextParams {
    fn default() -> Self {
        TextParams {
            style: TextStyle::default(),
            scale: 100.0,
            relative: Vec2::new(0.0, 0.0),
            absolute: Vec2::new(0.0, 0.0),
            alignment: TextAlignment::Left,
            anchor: bevy::sprite::Anchor::CenterLeft,
        }
    }
}
impl TextParams {
    pub fn centered (style: TextStyle, scale: f32) -> TextParams {
        TextParams {
            style,
            scale,
            relative: Vec2::new(50.0, 50.0),
            alignment: TextAlignment::Center,
            anchor: bevy::sprite::Anchor::Center,
            ..Default::default()
        }
    }
    pub fn left (style: TextStyle, scale: f32, relative: Vec2) -> TextParams {
        TextParams {
            style,
            scale,
            relative,
            alignment: TextAlignment::Left,
            anchor: bevy::sprite::Anchor::CenterLeft,
            ..Default::default()
        }
    }
    pub fn right (style: TextStyle, scale: f32, relative: Vec2) -> TextParams {
        TextParams {
            style,
            scale,
            relative,
            alignment: TextAlignment::Right,
            anchor: bevy::sprite::Anchor::CenterRight,
            ..Default::default()
        }
    }
}


#[derive(Clone, Debug)]
pub struct ImageParams {
    pub scale: f32,
    pub relative: Vec2,
    pub absolute: Vec2,
    pub anchor: bevy::sprite::Anchor,
}
impl Default for ImageParams {
    fn default() -> Self {
        ImageParams {
            scale: 100.0,
            relative: Vec2::new(0.0, 0.0),
            absolute: Vec2::new(0.0, 0.0),
            anchor: bevy::sprite::Anchor::TopLeft,
        }
    }
}


pub fn spawn_text (commands: &mut Commands, widget: Widget, text_params: TextParams, text: &str) {
    commands.spawn (
        TextElementBundle {
            widget,
            element: Element {
                relative: text_params.relative,
                absolute: text_params.absolute,
                boundary: text_compute_size_simple(text, text_params.style.font_size),
                scale: text_params.scale,
                ..default()
            },
            text: Text::from_section(text, text_params.style.clone()).with_alignment(text_params.alignment),
            text_anchor: text_params.anchor,
            ..Default::default()
        }
    );
}

pub fn spawn_image (commands: &mut Commands, asset_server: &Res<AssetServer>, widget: Widget, image_params: ImageParams, path: &str) {
    commands.spawn (
        ImageElementBundle {
            widget,
            element: Element {
                relative: image_params.relative,
                absolute: image_params.absolute,
                scale: image_params.scale,
                ..default()
            },
            texture: asset_server.load(path),
            sprite: Sprite {
                anchor: image_params.anchor,
                ..default()
            },
            ..Default::default()
        }
    );
    
}

pub fn spawn_image_with_text (commands: &mut Commands, asset_server: &Res<AssetServer>, widget: Widget, image_params: ImageParams, path: &str, text_params: TextParams, text: &str) {
    commands.spawn (
        ImageElementBundle {
            widget : widget.clone(),
            element: Element {
                relative: image_params.relative,
                absolute: image_params.absolute,
                scale: image_params.scale,
                ..default()
            },
            texture: asset_server.load(path),
            sprite: Sprite {
                anchor: image_params.anchor,
                ..default()
            },
            ..Default::default()
        }
    );

    commands.spawn (
        TextElementBundle {
            widget,
            element: Element {
                relative: text_params.relative,
                absolute: text_params.absolute,
                boundary: text_compute_size_simple(text, text_params.style.font_size),
                scale: text_params.scale,
                ..default()
            },
            text: Text::from_section(text, text_params.style.clone()).with_alignment(text_params.alignment),
            text_anchor: text_params.anchor,
            ..Default::default()
        }
    );
}
