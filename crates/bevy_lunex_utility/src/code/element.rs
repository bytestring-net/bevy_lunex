use bevy::prelude::*;
use bevy_lunex_core::Widget;

// ===========================================================
// === ELEMENT DEFINITION ===

/// # Element
/// Struct holding all necessary information for binding an entity to a [`Widget`].
/// # Fields
/// * `relative` = position in % relative to the widget.
/// * `absolute` = position in pixels, always the same.
/// * `boundary` = width and height, for example image dimensions or text size.
/// * `scale` = size of the element in % of parent widget.
/// * `depth` = local depth of the element, starts at 0.0, usefull when you have 2 elements overlapping (image and text)
/// * `width` = optional, will force the width of the element in % of parent widget.
/// * `height` = optional, will force the height of the element in % of parent widget.
#[derive(Component, Clone, Debug)]
pub struct Element {
    /// ### Relative
    /// Position in % relative to the widget.
    pub relative: Vec2,
    /// ### Absolute
    /// Position in pixels, always the same.
    pub absolute: Vec2,
    /// ### Boundary
    /// Width and height, for example image dimensions or text size.
    pub boundary: Vec2,
    /// ### Scale
    /// Size of the element in % of parent widget.
    pub scale: f32,
    /// ### Depth
    /// Local depth of the element, starts at 0.0, usefull when you have 2 elements overlapping (image and text)
    pub depth: f32,
    /// ### Width
    /// Optional, will force the width of the element in % of parent widget.
    pub width: Option<f32>,
    /// ### Height
    /// Optional, will force the height of the element in % of parent widget.
    pub height: Option<f32>,
}
impl Default for Element {
    fn default() -> Self {
        Element {
            relative: Vec2::new(50.0, 50.0),
            absolute: Vec2::new(0.0, 0.0),
            boundary: Vec2::new(50.0, 50.0),
            scale: 100.0,
            depth: 0.0,
            width: None,
            height: None,
        }
    }
}
impl Element {
    /// New element created from default
    pub fn new() -> Element {
        Element::default()
    }

    /// New element of the size 1x1 scaled to the size of a container
    /// * transform.scale.x = width of the container
    /// * transform.scale.y = height of the container
    pub fn fullfill() -> Element {
        Element::default().with_bounds(Vec2::splat(1.0)).with_width(Some(100.0)).with_height(Some(100.0))
    }

    /// Element set to a custom relative position
    pub fn at(mut self, x: f32, y: f32) -> Element {
        self.relative = Vec2::new(x, y);
        self
    }
    
    /// Element set to a custom x relative position
    pub fn at_x(mut self, x: f32) -> Element {
        self.relative.x = x;
        self
    }
    
    /// Element set to a custom y relative position
    pub fn at_y(mut self, y: f32) -> Element {
        self.relative.y = y;
        self
    }

    /// Element set to a custom absolute position
    pub fn at_abs(mut self, x: f32, y: f32) -> Element {
        self.absolute = Vec2::new(x, y);
        self
    }
    
    /// Element set to a custom x absolute position
    pub fn at_x_abs(mut self, x: f32) -> Element {
        self.absolute.x = x;
        self
    }
    
    // Element set to a custom y absolute position
    pub fn at_y_abs(mut self, y: f32) -> Element {
        self.absolute.y = y;
        self
    }
    
    /// Element set to a custom boundary
    pub fn with_bounds(mut self, boundary: Vec2) -> Element {
        self.boundary = boundary;
        self
    }

    /// Element set to a custom scale
    pub fn with_scale(mut self, scale: f32) -> Element {
        self.scale = scale;
        self
    }
    
    /// Element set to a custom depth
    pub fn with_depth(mut self, depth: f32) -> Element {
        self.depth = depth;
        self
    }
    
    /// Element set to a custom width
    pub fn with_width(mut self, width: Option<f32>) -> Element {
        self.width = width;
        self
    }
    
    /// Element set to a custom height
    pub fn with_height(mut self, height: Option<f32>) -> Element {
        self.height = height;
        self
    }
}

/// # Element Bundle
/// A bundle containing [`Element`] + [`Widget`].
/// It is recommended to use the [`element_spawn`] macro abstraction.
/// # Fields
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
impl ElementBundle {
    /// # New
    /// Creates new [`ElementBundle`] from given parameters.
    /// # Arguments
    /// * `widget` = widget to spawn element for.
    /// * `element` = the element to spawn.
    pub fn new(widget: Widget, element: Element) -> ElementBundle {
        ElementBundle {
            widget,
            element,
            ..default()
        }
    }
}


// ===========================================================
// === IMAGE ELEMENT ===

/// # Image Element Bundle
/// A bundle containing [`Image`] + [`Element`] + [`Widget`].
/// It is recommended to use the `new` method.
/// # Fields
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
impl ImageElementBundle {
    /// # New
    /// Creates new [`ImageElementBundle`] from given parameters.
    /// # Arguments
    /// * `widget` = widget to spawn element for.
    /// * `image_params` = dictates how the element should behave and be located.
    /// * `texture` = image handle, you can use `asset_server.load("")`.
    /// * `source_image_dimensions` = `Vec2` with width and height dimensions of the texture.
    pub fn new(
        widget: Widget,
        image_params: &ImageParams,
        texture: Handle<Image>,
        source_image_dimensions: Vec2,
    ) -> ImageElementBundle {
        ImageElementBundle {
            widget,
            element: Element {
                relative: image_params.relative,
                absolute: image_params.absolute,
                boundary: source_image_dimensions,
                scale: image_params.scale,
                depth: image_params.depth,
                width: image_params.width,
                height: image_params.height,
                ..default()
            },
            texture,
            sprite: Sprite {
                anchor: image_params.anchor.clone(),
                color: image_params.color.clone(),
                flip_x: image_params.flip_x.clone(),
                flip_y: image_params.flip_y.clone(),
                ..default()
            },
            ..Default::default()
        }
    }
}

/// # Image parameters
/// Struct that is passed to [`image_element_spawn`] macro containing image information.
/// The fields are then transfered to the [`Element`] struct inside the macro.
/// # Fields
/// * `relative` = position in % relative to the widget.
/// * `absolute` = position in pixels, always the same.
/// * `anchor` = which corner of the image is origin (0.0).
/// * `color` = color tint of the image.
/// * `flip_x` = flips the image horizontally.
/// * `flip_y` = flips the image vertically.
/// * `scale` = size of the image in % of parent widget.
/// * `depth` = local depth of the image, starts at 0.0, usefull when you have 2 elements overlapping (image and text)
/// * `width` = optional, will force the width of the image in % of parent widget.
/// * `height` = optional, will force the height of the image in % of parent widget.
#[derive(Clone, Debug)]
pub struct ImageParams {
    /// ### Relative
    /// Position in % relative to the widget.
    pub relative: Vec2,
    /// ### Absolute
    /// Position in pixels, always the same.
    pub absolute: Vec2,
    /// ### Anchor
    /// Which corner of the image is origin (0.0).
    pub anchor: bevy::sprite::Anchor,
    /// ### Color
    /// Color tint of the image.
    pub color: Color,
    /// ### Flip X
    /// Flips the image horizontally.
    pub flip_x: bool,
    /// ### Flip Y
    /// Flips the image vertically..
    pub flip_y: bool,
    /// ### Scale
    /// Size of the image in % of parent widget.
    pub scale: f32,
    /// ### Depth
    /// Local depth of the image, starts at 0.0, usefull when you have 2 elements overlapping (image and text)
    pub depth: f32,
    /// ### Width
    /// Optional, will force the width of the image in % of parent widget.
    pub width: Option<f32>,
    /// ### Height
    /// Optional, will force the height of the image in % of parent widget.
    pub height: Option<f32>,
}
impl Default for ImageParams {
    fn default() -> Self {
        ImageParams {
            relative: Vec2::new(0.0, 0.0),
            absolute: Vec2::new(0.0, 0.0),
            anchor: bevy::sprite::Anchor::TopLeft,
            color: Color::WHITE,
            flip_x: false,
            flip_y: false,
            scale: 100.0,
            depth: 0.0,
            width: None,
            height: None,
        }
    }
}
impl ImageParams {
    /// Image parameters set to top center position
    pub fn topcenter() -> ImageParams {
        ImageParams {
            relative: Vec2::new(50.0, 0.0),
            anchor: bevy::sprite::Anchor::TopCenter,
            ..Default::default()
        }
    }
    /// Image parameters set to top left position
    pub fn topleft() -> ImageParams {
        ImageParams {
            relative: Vec2::new(0.0, 0.0),
            anchor: bevy::sprite::Anchor::TopLeft,
            ..Default::default()
        }
    }
    /// Image parameters set to top right position
    pub fn topright() -> ImageParams {
        ImageParams {
            relative: Vec2::new(100.0, 0.0),
            anchor: bevy::sprite::Anchor::TopRight,
            ..Default::default()
        }
    }

    /// Image parameters set to center position
    pub fn center() -> ImageParams {
        ImageParams {
            relative: Vec2::new(50.0, 50.0),
            anchor: bevy::sprite::Anchor::Center,
            ..Default::default()
        }
    }
    /// Image parameters set to center left position
    pub fn centerleft() -> ImageParams {
        ImageParams {
            relative: Vec2::new(0.0, 50.0),
            anchor: bevy::sprite::Anchor::CenterLeft,
            ..Default::default()
        }
    }
    /// Image parameters set to center right position
    pub fn centerright() -> ImageParams {
        ImageParams {
            relative: Vec2::new(100.0, 50.0),
            anchor: bevy::sprite::Anchor::CenterRight,
            ..Default::default()
        }
    }

    /// Image parameters set to bottom center position
    pub fn bottomcenter() -> ImageParams {
        ImageParams {
            relative: Vec2::new(50.0, 100.0),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..Default::default()
        }
    }
    /// Image parameters set to bottom left position
    pub fn bottomleft() -> ImageParams {
        ImageParams {
            relative: Vec2::new(0.0, 100.0),
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..Default::default()
        }
    }
    /// Image parameters set to bottom right position
    pub fn bottomright() -> ImageParams {
        ImageParams {
            relative: Vec2::new(100.0, 100.0),
            anchor: bevy::sprite::Anchor::BottomRight,
            ..Default::default()
        }
    }


    /// Image parameters set to a custom relative position
    pub fn at(mut self, x: f32, y: f32) -> ImageParams {
        self.relative = Vec2::new(x, y);
        self
    }
    
    /// Image parameters set to a custom x relative position
    pub fn at_x(mut self, x: f32) -> ImageParams {
        self.relative.x = x;
        self
    }
    
    /// Image parameters set to a custom y relative position
    pub fn at_y(mut self, y: f32) -> ImageParams {
        self.relative.y = y;
        self
    }

    /// Image parameters set to a custom absolute position
    pub fn at_abs(mut self, x: f32, y: f32) -> ImageParams {
        self.absolute = Vec2::new(x, y);
        self
    }
    
    /// Image parameters set to a custom x absolute position
    pub fn at_x_abs(mut self, x: f32) -> ImageParams {
        self.absolute.x = x;
        self
    }
    
    /// Image parameters set to a custom y absolute position
    pub fn at_y_abs(mut self, y: f32) -> ImageParams {
        self.absolute.y = y;
        self
    }

    /// Image parameters set to a custom scale
    pub fn with_scale(mut self, scale: f32) -> ImageParams {
        self.scale = scale;
        self
    }

    /// Image parameters set to a custom color
    pub fn with_color(mut self, color: Color) -> ImageParams {
        self.color = color;
        self
    }

    /// Image parameters set to a custom flip_x
    pub fn with_flip_x(mut self, flip_x: bool) -> ImageParams {
        self.flip_x = flip_x;
        self
    }

    /// Image parameters set to a custom flip_x
    pub fn with_flip_y(mut self, flip_y: bool) -> ImageParams {
        self.flip_y = flip_y;
        self
    }
    
    /// Image parameters set to a custom depth
    pub fn with_depth(mut self, depth: f32) -> ImageParams {
        self.depth = depth;
        self
    }
    
    /// Image parameters set to a custom width
    pub fn with_width(mut self, width: Option<f32>) -> ImageParams {
        self.width = width;
        self
    }
    
    /// Image parameters set to a custom height
    pub fn with_height(mut self, height: Option<f32>) -> ImageParams {
        self.height = height;
        self
    }
}


// ===========================================================
// === TEXT ELEMENT ===

/// # Text Element Bundle
/// A bundle containing [`Text`] + [`Element`] + [`Widget`].
/// It is recommended to use the `new` method.
/// # Fields
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
impl TextElementBundle {
    /// # New
    /// Creates new [`TextElementBundle`] from given parameters.
    /// # Arguments
    /// * `widget` = widget to spawn element for.
    /// * `text_params` = dictates how the element should behave and be located.
    /// * `text` = the text you want to display.
    pub fn new(widget: Widget, text_params: &TextParams, text: &str) -> TextElementBundle {
        TextElementBundle {
            widget,
            element: Element {
                relative: text_params.relative,
                absolute: text_params.absolute,
                boundary: text_compute_size_simple(text, text_params.style.font_size),
                scale: text_params.scale,
                width: text_params.width,
                height: text_params.height,
                depth: text_params.depth,
                ..Default::default()
            },
            text: Text::from_section(text, text_params.style.clone())
                .with_alignment(text_params.alignment),
            text_anchor: text_params.anchor.clone(),
            ..Default::default()
        }
    }
}

/// # Text parameters
/// Struct that is passed to [`text_element_spawn`] macro containing text information.
/// The fields are then transfered to the [`Element`] struct inside the macro.
/// # Example:
/// ```
///  let params = TextParams::centerleft().at(20.0, 50.0).with_height(80.0);
/// ```
/// # Fields
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
    /// ### Relative
    /// Position in % relative to the widget.
    pub relative: Vec2,
    /// ### Absolute
    /// Position in pixels, always the same.
    pub absolute: Vec2,
    /// ### Style
    /// [`TextStyle`] struct from Bevy.
    pub style: TextStyle,
    /// ### Alignment
    /// Where the text is aligned - left/center/right.
    pub alignment: TextAlignment,
    /// ### Anchor
    /// Which corner of the text is origin (0.0).
    pub anchor: bevy::sprite::Anchor,
    /// ### Scale
    /// Size of the text in % of parent widget.
    pub scale: f32,
    /// ### Depth
    /// Local depth of the text, starts at 0.0, usefull when you have 2 elements overlapping (image and text)
    pub depth: f32,
    /// ### Width
    /// Optional, will force the width of the text in % of parent widget.
    pub width: Option<f32>,
    /// ### Height
    /// Optional, will force the height of the text in % of parent widget.
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
            width: None,
            height: None,
        }
    }
}
impl TextParams {
    /// Text parameters set to top center position
    pub fn topcenter() -> TextParams {
        TextParams {
            relative: Vec2::new(50.0, 0.0),
            alignment: TextAlignment::Center,
            anchor: bevy::sprite::Anchor::TopCenter,
            ..Default::default()
        }
    }
    /// Text parameters set to top left position
    pub fn topleft() -> TextParams {
        TextParams {
            relative: Vec2::new(0.0, 0.0),
            alignment: TextAlignment::Left,
            anchor: bevy::sprite::Anchor::TopLeft,
            ..Default::default()
        }
    }
    /// Text parameters set to top right position
    pub fn topright() -> TextParams {
        TextParams {
            relative: Vec2::new(100.0, 0.0),
            alignment: TextAlignment::Right,
            anchor: bevy::sprite::Anchor::TopRight,
            ..Default::default()
        }
    }

    /// Text parameters set to center position
    pub fn center() -> TextParams {
        TextParams {
            relative: Vec2::new(50.0, 50.0),
            alignment: TextAlignment::Center,
            anchor: bevy::sprite::Anchor::Center,
            ..Default::default()
        }
    }
    /// Text parameters set to center left position
    pub fn centerleft() -> TextParams {
        TextParams {
            relative: Vec2::new(0.0, 50.0),
            alignment: TextAlignment::Left,
            anchor: bevy::sprite::Anchor::CenterLeft,
            ..Default::default()
        }
    }
    /// Text parameters set to center right position
    pub fn centerright() -> TextParams {
        TextParams {
            relative: Vec2::new(100.0, 50.0),
            alignment: TextAlignment::Right,
            anchor: bevy::sprite::Anchor::CenterRight,
            ..Default::default()
        }
    }

    /// Text parameters set to bottom center position
    pub fn bottomcenter() -> TextParams {
        TextParams {
            relative: Vec2::new(50.0, 100.0),
            alignment: TextAlignment::Center,
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..Default::default()
        }
    }
    /// Text parameters set to bottom left position
    pub fn bottomleft() -> TextParams {
        TextParams {
            relative: Vec2::new(0.0, 100.0),
            alignment: TextAlignment::Left,
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..Default::default()
        }
    }
    /// Text parameters set to bottom right position
    pub fn bottomright() -> TextParams {
        TextParams {
            relative: Vec2::new(100.0, 100.0),
            alignment: TextAlignment::Right,
            anchor: bevy::sprite::Anchor::BottomRight,
            ..Default::default()
        }
    }


    /// Text parameters set to a custom relative position
    pub fn at(mut self, x: f32, y: f32) -> TextParams {
        self.relative = Vec2::new(x, y);
        self
    }
    
    /// Text parameters set to a custom x relative position
    pub fn at_x(mut self, x: f32) -> TextParams {
        self.relative.x = x;
        self
    }
    
    /// Text parameters set to a custom y relative position
    pub fn at_y(mut self, y: f32) -> TextParams {
        self.relative.y = y;
        self
    }

    /// Text parameters set to a custom absolute position
    pub fn at_abs(mut self, x: f32, y: f32) -> TextParams {
        self.absolute = Vec2::new(x, y);
        self
    }
    
    /// Text parameters set to a custom x absolute position
    pub fn at_x_abs(mut self, x: f32) -> TextParams {
        self.absolute.x = x;
        self
    }
    
    /// Text parameters set to a custom y absolute position
    pub fn at_y_abs(mut self, y: f32) -> TextParams {
        self.absolute.y = y;
        self
    }

    /// Text parameters set to a specific text style
    pub fn with_style(mut self, style: &TextStyle) -> TextParams {
        self.style = style.clone();
        self
    }

    /// Text parameters set to a custom scale
    pub fn with_scale(mut self, scale: f32) -> TextParams {
        self.scale = scale;
        self
    }
    
    /// Text parameters set to a custom depth
    pub fn with_depth(mut self, depth: f32) -> TextParams {
        self.depth = depth;
        self
    }
    
    /// Text parameters set to a custom width
    pub fn with_width(mut self, width: Option<f32>) -> TextParams {
        self.width = width;
        self
    }
    
    /// Text parameters set to a custom height
    pub fn with_height(mut self, height: Option<f32>) -> TextParams {
        self.height = height;
        self
    }
}


// ===========================================================
// === BOUNDARY COMPUTATION ===

/// # Text size compute
/// Simple and rough function to estimate boundary of a text.
/// This function exists because there is currently not a nice way on how to get text boundary from Bevy internals.
pub fn text_compute_size_simple(text: &str, font_size: f32) -> Vec2 {
    const SYMBOL_WIDTH_WEIGHT: f32 = 0.8 * 0.5;
    const SYMBOL_HEIGHT_WEIGHT: f32 = 1.2 * 0.5;

    let mut width = 0.0;

    let list: Vec<&str> = text.split("/n").collect();
    for line in &list {
        let len = line.chars().count();
        if len as f32 > width {
            width = len as f32
        }
    }

    Vec2::new(
        width * font_size * SYMBOL_WIDTH_WEIGHT,
        list.len() as f32 * font_size * SYMBOL_HEIGHT_WEIGHT,
    )
}

//# --------------------------------------------------------------------------------------------------------------
