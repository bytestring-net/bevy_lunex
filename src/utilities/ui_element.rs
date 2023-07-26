use bevy::prelude::*;
use crate::prelude::*;


// ===========================================================
// === ELEMENT ===

/// ### Widget spawn macro
/// Simple abstraction for spawning an entity with [`Widget`] component.
/// ### Arguments
/// * `&mut Commands`
/// * [`Widget`]
/// * `Component` (Optional, n)
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

/// ### Element Bundle
/// A bundle containing [`Element`] + [`Widget`].
/// It is recommended to use the [`element_spawn`] macro abstraction.
/// ### Fields
/// * `widget`
/// * `element`
/// * `transform`
/// * `visibility`
/// * `global_transform`
/// * `computed_visibility`
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

/// ### Image Element Bundle
/// A bundle containing [`Image`] + [`Element`] + [`Widget`].
/// It is recommended to use the [`image_element_spawn`] macro abstraction.
/// ### Fields
/// * `widget`
/// * `element`
/// * `sprite`
/// * `texture`
/// * `transform`
/// * `visibility`
/// * `global_transform`
/// * `computed_visibility`
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

/// ### Image parameters
/// Struct that is passed to [`image_element_spawn`] macro containing image information.
/// The fields are then transfered to the [`Element`] struct inside the macro.
/// ### Fields
/// * `relative` = position in % relative to the widget.
/// * `absolute` = position in pixels, always the same.
/// * `anchor` = which corner of the image is origin (0.0). 
/// * `scale` = size of the image in % of parent widget.
/// * `depth` = local depth of the image, starts at 0.0, usefull when you have 2 elements overlapping (image and text)
/// * `width` = optional, will force the width of the image in % of parent widget.
/// * `height` = optional, will force the height of the image in % of parent widget.
#[derive(Clone, Debug)]
pub struct ImageParams {
    pub relative: Vec2,
    pub absolute: Vec2,
    pub anchor: bevy::sprite::Anchor,
    pub scale: f32,
    pub depth: f32,
    pub width: Option<f32>,
    pub height: Option<f32>,
}
impl Default for ImageParams {
    fn default() -> Self {
        ImageParams {
            relative: Vec2::new(0.0, 0.0),
            absolute: Vec2::new(0.0, 0.0),
            anchor: bevy::sprite::Anchor::TopLeft,
            scale: 100.0,
            depth: 0.0,
            width: Option::None,
            height: Option::None,
        }
    }
}

/// ### Image element spawn macro
/// Simple abstraction for spawning an entity with [`ImageElementBundle`].
/// ### Arguments
/// * `&mut Commands`
/// * `&Res<AssetServer>`
/// * [`Widget`]
/// * &[`ImageParams`]
/// * `&str` (Image path)
/// * `Component` (Optional, n)
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
                    depth: _image_params.depth,
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
                    depth: _image_params.depth,
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

/// ### Text Element Bundle
/// A bundle containing [`Text`] + [`Element`] + [`Widget`].
/// It is recommended to use the [`text_element_spawn`] macro abstraction.
/// ### Fields
/// * `widget`
/// * `element`
/// * `text`
/// * `text_anchor`
/// * `text_2d_bounds`
/// * `text_layout_info`
/// * `transform`
/// * `visibility`
/// * `global_transform`
/// * `computed_visibility`
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

/// ### Text parameters
/// Struct that is passed to [`text_element_spawn`] macro containing text information.
/// The fields are then transfered to the [`Element`] struct inside the macro.
/// ### Example:
/// ```
///  let params = TextParams::centerleft().at(20.0, 50.0).with_height(80.0);
/// ```
/// ### Fields
/// * `relative` = position in % relative to the widget.
/// * `absolute` = position in pixels, always the same.
/// * `style` = [`TextStyle`] struct from Bevy.
/// * `alignment` = where the text is aligned - left/center/right.
/// * `anchor` = which corner of the text is origin (0.0). 
/// * `scale` = size of the text in % of parent widget.
/// * `depth` = local depth of the text, starts at 0.0, usefull when you have 2 elements overlapping (image and text)
/// * `width` = optional, will force the width of the text in % of parent widget.
/// * `height` = optional, will force the height of the text in % of parent widget.
#[derive(Clone, Debug)]
pub struct TextParams {
    pub relative: Vec2,
    pub absolute: Vec2,
    pub style: TextStyle,
    pub alignment: TextAlignment,
    pub anchor: bevy::sprite::Anchor,
    pub scale: f32,
    pub depth: f32,
    pub width: Option<f32>,
    pub height: Option<f32>,
}
impl Default for TextParams {
    fn default() -> Self {
        TextParams {
            relative: Vec2::new(0.0, 0.0),
            absolute: Vec2::new(0.0, 0.0),
            style: TextStyle::default(),
            alignment: TextAlignment::Left,
            anchor: bevy::sprite::Anchor::CenterLeft,
            scale: 100.0,
            depth: 3.0,
            width: Option::None,
            height: Option::None,
        }
    }
}
impl TextParams {
    /// Text parameters set to top center position
    pub fn topcenter () -> TextParams {
        TextParams {
            relative: Vec2::new(50.0, 0.0),
            alignment: TextAlignment::Center,
            anchor: bevy::sprite::Anchor::TopCenter,
            ..Default::default()
        }
    }
    /// Text parameters set to top left position
    pub fn topleft () -> TextParams {
        TextParams {
            relative: Vec2::new(0.0, 0.0),
            alignment: TextAlignment::Left,
            anchor: bevy::sprite::Anchor::TopLeft,
            ..Default::default()
        }
    }
    /// Text parameters set to top right position
    pub fn topright () -> TextParams {
        TextParams {
            relative: Vec2::new(100.0, 0.0),
            alignment: TextAlignment::Right,
            anchor: bevy::sprite::Anchor::TopRight,
            ..Default::default()
        }
    }

    /// Text parameters set to center position
    pub fn center () -> TextParams {
        TextParams {
            relative: Vec2::new(50.0, 50.0),
            alignment: TextAlignment::Center,
            anchor: bevy::sprite::Anchor::Center,
            ..Default::default()
        }
    }
    /// Text parameters set to center left position
    pub fn centerleft () -> TextParams {
        TextParams {
            relative: Vec2::new(0.0, 50.0),
            alignment: TextAlignment::Left,
            anchor: bevy::sprite::Anchor::CenterLeft,
            ..Default::default()
        }
    }
    /// Text parameters set to center right position
    pub fn centerright () -> TextParams {
        TextParams {
            relative: Vec2::new(100.0, 50.0),
            alignment: TextAlignment::Right,
            anchor: bevy::sprite::Anchor::CenterRight,
            ..Default::default()
        }
    }
    
    /// Text parameters set to bottom center position
    pub fn bottomcenter () -> TextParams {
        TextParams {
            relative: Vec2::new(50.0, 100.0),
            alignment: TextAlignment::Center,
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..Default::default()
        }
    }
    /// Text parameters set to bottom left position
    pub fn bottomleft () -> TextParams {
        TextParams {
            relative: Vec2::new(0.0, 100.0),
            alignment: TextAlignment::Left,
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..Default::default()
        }
    }
    /// Text parameters set to bottom right position
    pub fn bottomright () -> TextParams {
        TextParams {
            relative: Vec2::new(100.0, 100.0),
            alignment: TextAlignment::Right,
            anchor: bevy::sprite::Anchor::BottomRight,
            ..Default::default()
        }
    }
    
    /// Text parameters set to a custom position
    pub fn at (mut self, x: f32, y: f32) -> TextParams {
        self.relative = Vec2::new(x, y);
        self
    }
    /// Text parameters set to a custom x position
    pub fn at_x (mut self, x: f32) -> TextParams {
        self.relative.x = x;
        self
    }
    /// Text parameters set to a custom y position
    pub fn at_y (mut self, y: f32) -> TextParams {
        self.relative.y = y;
        self
    }
    
    /// Text parameters set to a specific text style
    pub fn styled (mut self, style: &TextStyle) -> TextParams {
        self.style = style.clone();
        self
    }
    /// Text parameters set to a custom scale
    pub fn scaled (mut self, scale: f32) -> TextParams {
        self.scale = scale;
        self
    }
    /// Text parameters set to a custom width
    pub fn with_width (mut self, width: f32) -> TextParams {
        self.width = Option::Some(width);
        self
    }
    /// Text parameters set to a custom height
    pub fn with_height (mut self, height: f32) -> TextParams {
        self.height = Option::Some(height);
        self
    }
}

/// ### Text size compute
/// Simple and rough function to estimate boundary of a text.
/// This function exists because there is currently not a nice way on how to get text boundary from Bevy internals.
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

/// ### Text element spawn macro
/// Simple abstraction for spawning an entity with [`TextElementBundle`].
/// ### Arguments
/// * `&mut Commands`
/// * [`Widget`]
/// * &[`TextParams`]
/// * `&str` (Text)
/// * `Component` (Optional, n)
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
                    depth: _text_params.depth,
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
                    depth: _text_params.depth,
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