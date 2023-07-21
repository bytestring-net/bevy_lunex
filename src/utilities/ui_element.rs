use bevy::prelude::*;
use crate::prelude::*;


// ===========================================================
// === ELEMENT ===

#[macro_export]
macro_rules! widget_spawn {
    // When additional components $x:expr are provided
    ($commands:ident, $widget:expr, $( $x:expr ),*) => {
        let _commands: &mut Commands = $commands;
        let _widget: Widget = $widget;
        _commands.spawn((
            _widget,
            $(
                $x,
            )*
        ));
    };
}


#[derive(Component, Clone, Debug, Default)]
pub struct Element {
    pub relative: Vec2,
    pub absolute: Vec2,
    pub boundary: Vec2,
    pub scale: f32,
    pub depth: f32,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

//# --------------------------------------------------------------------------------------------------------------

#[derive(Bundle, Clone, Debug, Default)]
pub struct ElementBundle {
    pub widget: Widget,
    pub element: Element,
    pub transform: Transform,
    pub visibility: Visibility,
    pub global_transform: GlobalTransform,
    pub computed_visibility: ComputedVisibility,
}

//# --------------------------------------------------------------------------------------------------------------

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

#[derive(Clone, Debug)]
pub struct ImageParams {
    pub scale: f32,
    pub relative: Vec2,
    pub absolute: Vec2,
    pub anchor: bevy::sprite::Anchor,
    pub width: Option<f32>,
    pub height: Option<f32>,
}
impl Default for ImageParams {
    fn default() -> Self {
        ImageParams {
            scale: 100.0,
            relative: Vec2::new(0.0, 0.0),
            absolute: Vec2::new(0.0, 0.0),
            anchor: bevy::sprite::Anchor::TopLeft,
            width: Option::None,
            height: Option::None,
        }
    }
}

#[macro_export]
macro_rules! image_element_spawn {
    // When only the required parameters are provided (without $x:expr)
    ($commands:ident, $asset_server:ident, $widget:expr, $image_params:expr, $path:expr) => {
        let _commands: &mut Commands = $commands;
        let _asset_server: &Res<AssetServer> = $asset_server;
        let _widget: Widget = $widget;
        let _image_params: &ImageParams = $image_params;
        let _path: &str = $path;
        _commands.spawn((
            ImageElementBundle {
                widget: _widget,
                element: Element {
                    relative: _image_params.relative,
                    absolute: _image_params.absolute,
                    scale: _image_params.scale,
                    width: _image_params.width,
                    height: _image_params.height,
                    ..default()
                },
                texture: _asset_server.load(_path),
                sprite: Sprite {
                    anchor: _image_params.anchor.clone(),
                    ..default()
                },
                ..Default::default()
            }
        ));
    };

    // When additional components $x:expr are provided
    ($commands:ident, $asset_server:ident, $widget:expr, $image_params:expr, $path:expr, $( $x:expr ),*) => {
        let _commands: &mut Commands = $commands;
        let _asset_server: &Res<AssetServer> = $asset_server;
        let _widget: Widget = $widget;
        let _image_params: &ImageParams = $image_params;
        let _path: &str = $path;
        _commands.spawn((
            ImageElementBundle {
                widget: _widget,
                element: Element {
                    relative: _image_params.relative,
                    absolute: _image_params.absolute,
                    scale: _image_params.scale,
                    width: _image_params.width,
                    height: _image_params.height,
                    ..default()
                },
                texture: _asset_server.load(_path),
                sprite: Sprite {
                    anchor: _image_params.anchor.clone(),
                    ..default()
                },
                ..Default::default()
            },
            $(
                $x,
            )*
        ));
    };
}

//# --------------------------------------------------------------------------------------------------------------

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

#[derive(Clone, Debug)]
pub struct TextParams {
    pub style: TextStyle,
    pub scale: f32,
    pub relative: Vec2,
    pub absolute: Vec2,
    pub alignment: TextAlignment,
    pub anchor: bevy::sprite::Anchor,
    pub width: Option<f32>,
    pub height: Option<f32>,
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
            width: Option::None,
            height: Option::None,
        }
    }
}
impl TextParams {
    pub fn topcenter () -> TextParams {
        TextParams {
            relative: Vec2::new(50.0, 0.0),
            alignment: TextAlignment::Center,
            anchor: bevy::sprite::Anchor::TopCenter,
            ..Default::default()
        }
    }
    pub fn topleft () -> TextParams {
        TextParams {
            relative: Vec2::new(0.0, 0.0),
            alignment: TextAlignment::Left,
            anchor: bevy::sprite::Anchor::TopLeft,
            ..Default::default()
        }
    }
    pub fn topright () -> TextParams {
        TextParams {
            relative: Vec2::new(100.0, 0.0),
            alignment: TextAlignment::Right,
            anchor: bevy::sprite::Anchor::TopRight,
            ..Default::default()
        }
    }

    pub fn center () -> TextParams {
        TextParams {
            relative: Vec2::new(50.0, 50.0),
            alignment: TextAlignment::Center,
            anchor: bevy::sprite::Anchor::Center,
            ..Default::default()
        }
    }
    pub fn centerleft () -> TextParams {
        TextParams {
            relative: Vec2::new(0.0, 50.0),
            alignment: TextAlignment::Left,
            anchor: bevy::sprite::Anchor::CenterLeft,
            ..Default::default()
        }
    }
    pub fn centerright () -> TextParams {
        TextParams {
            relative: Vec2::new(100.0, 50.0),
            alignment: TextAlignment::Right,
            anchor: bevy::sprite::Anchor::CenterRight,
            ..Default::default()
        }
    }
    
    pub fn bottomcenter () -> TextParams {
        TextParams {
            relative: Vec2::new(50.0, 100.0),
            alignment: TextAlignment::Center,
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..Default::default()
        }
    }
    pub fn bottomleft () -> TextParams {
        TextParams {
            relative: Vec2::new(0.0, 100.0),
            alignment: TextAlignment::Left,
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..Default::default()
        }
    }
    pub fn bottomright () -> TextParams {
        TextParams {
            relative: Vec2::new(100.0, 100.0),
            alignment: TextAlignment::Right,
            anchor: bevy::sprite::Anchor::BottomRight,
            ..Default::default()
        }
    }
    
    pub fn at (mut self, x: f32, y: f32) -> TextParams {
        self.relative = Vec2::new(x, y);
        self
    }
    pub fn styled (mut self, style: &TextStyle) -> TextParams {
        self.style = style.clone();
        self
    }
    pub fn scaled (mut self, scale: f32) -> TextParams {
        self.scale = scale;
        self
    }
    pub fn with_width (mut self, width: f32) -> TextParams {
        self.width = Option::Some(width);
        self
    }
    pub fn with_height (mut self, height: f32) -> TextParams {
        self.height = Option::Some(height);
        self
    }
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
#[macro_export]
macro_rules! text_element_spawn {
    // When only the required parameters are provided (without $x:expr)
    ($commands:ident, $widget:expr, $text_params:expr, $text:expr) => {
        let _commands: &mut Commands = $commands;
        let _widget: Widget = $widget;
        let _text_params: &TextParams = $text_params;
        let _text: &str = $text;
        _commands.spawn((
            TextElementBundle {
                widget: _widget,
                element: Element {
                    relative: _text_params.relative,
                    absolute: _text_params.absolute,
                    boundary: text_compute_size_simple(_text, _text_params.style.font_size),
                    scale: _text_params.scale,
                    width: _text_params.width,
                    height: _text_params.height,
                    depth: 1.0, //So text can be visible over images and there is no clipping
                    ..Default::default()
                },
                text: Text::from_section(_text, _text_params.style.clone()).with_alignment(_text_params.alignment),
                text_anchor: _text_params.anchor.clone(),
                ..Default::default()
            },
        ));
    };

    // When additional components $x:expr are provided
    ($commands:ident, $widget:expr, $text_params:expr, $text:expr, $( $x:expr ),*) => {
        let _commands: &mut Commands = $commands;
        let _widget: Widget = $widget;
        let _text_params: &TextParams = $text_params;
        let _text: &str = $text;
        _commands.spawn((
            TextElementBundle {
                widget: _widget,
                element: Element {
                    relative: _text_params.relative,
                    absolute: _text_params.absolute,
                    boundary: text_compute_size_simple(_text, _text_params.style.font_size),
                    scale: _text_params.scale,
                    width: _text_params.width,
                    height: _text_params.height,
                    depth: 1.0, //So text can be visible over images and there is no clipping
                    ..Default::default()
                },
                text: Text::from_section(_text, _text_params.style.clone()).with_alignment(_text_params.alignment),
                text_anchor: _text_params.anchor.clone(),
                ..Default::default()
            },
            $(
                $x,
            )*
        ));
    };
}

//# --------------------------------------------------------------------------------------------------------------