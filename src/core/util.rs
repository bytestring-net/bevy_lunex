use bevy::prelude::*;

/// Allows converting a Vec2 in Bevy's coordinate system to Lunex's coordinate system.
pub trait AsLunexVec2 {
    /// This function is used for translating Vec2 from Bevy coordinate system to Lunex coordinate system.
    /// It is necessary to go through this step if you want entities to be able to interact with Lunex.
    ///
    /// Example of this is the cursor entity which has [`Transform`] component.
    /// Due to the nature of Bevy, the y+ direction is upwards instead of downwards. This is extremely counterintuitive, especially for UI.
    /// * This function will invert the Y component.
    /// * In addition it will offset the values because Bevy-Lunex always starts at 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let offset = Vec2::new(-window.size.x / 2.0, window.size.y / 2.0);
    /// let cursor_position = Vec2::new(40.0, 20.0);
    ///
    /// let inside:bool = widget.is_within(&system, "", cursor_position.as_lunex(offset)).unwrap();
    /// ```
    fn as_lunex(self, offset: Vec2) -> Vec2;
}

impl AsLunexVec2 for Vec2 {
    fn as_lunex(self, offset: Vec2) -> Vec2 {
        Vec2::new(self.x - offset.x, offset.y - self.y)
    }
}

// ===========================================================
// === GENERAL ===

/// Very simple function for linear interpolation between 2 values.
/// 
/// * `slide` ranges from 0.0 to 1.0
pub fn tween(value_1: f32, value_2: f32, slide: f32) -> f32 {
    let diff = value_2 - value_1;
    value_1 + diff * slide
}

/// Returns value that is normalized into a given period.
/// Allows you to easily clamp values with overflow.
///
/// The most common example would be normalizing degrees between 0 and 360.
/// 
/// # Example
/// 
/// ```
///  let period = 360.0;
///  assert_eq!(315.0, periodical(period, -45.0));
///  assert_eq!(45.0, periodical(period, 45.0));
///  assert_eq!(0.0, periodical(period, 360.0));
///  assert_eq!(90.0, periodical(period, 450.0));
/// ```
pub fn periodical(period: f32, x: f32) -> f32 {
    let value = x % period;
    if value < 0.0 {
        value + period
    } else {
        value
    }
}

/// Returns a difference between 2 periodical values.
/// Uses the shortest path.
///
/// The most common example would be getting a difference between 2 angles in degrees.
/// Because of the nature of trigonometry, you can sometimes get inner or outer angle depending on use case.
/// This function will always return the INNER angle.
/// 
/// # Example
/// 
/// ```
///  let period = 360.0;
///  assert_eq!(120.0, periodical_difference_short(period, 0.0, 120.0));
///  assert_eq!(-90.0, periodical_difference_short(period, 0.0, 270.0)); //Always returns the inner angle
///  assert_eq!(45.0, periodical_difference_short(period, 45.0, 90.0));
///  assert_eq!(-45.0, periodical_difference_short(period, 90.0, 45.0));
/// ```
pub fn periodical_difference_short(period: f32, x1: f32, x2: f32) -> f32 {
    let difference = (periodical(period, x2) - periodical(period, x1)) % period;
    if difference > period / 2.0 {
        difference - period
    } else if difference < -period / 2.0 {
        difference + period
    } else {
        difference
    }
}

/// Returns a difference between 2 periodical values.
/// Uses the longest path.
///
/// The most common example would be getting a difference between 2 angles in degrees.
/// Because of the nature of trigonometry, you can sometimes get inner or outer angle depending on use case. This function will always return the OUTER angle.
/// 
/// # Example
/// 
/// ```
///  let period = 360.0;
///  assert_eq!(-240.0, periodical_difference_long(period, 0.0, 120.0)); //Always returns the outer angle
///  assert_eq!(270.0, periodical_difference_long(period, 0.0, 270.0));
///  assert_eq!(-315.0, periodical_difference_long(period, 45.0, 90.0));
///  assert_eq!(315.0, periodical_difference_long(period, 90.0, 45.0));
/// ```
pub fn periodical_difference_long(period: f32, x1: f32, x2: f32) -> f32 {
    let difference = (periodical(period, x2) - periodical(period, x1)) % period;
    if difference < 0.0 {
        difference + period
    } else if difference <= period / 2.0 {
        difference - period
    } else {
        difference
    }
}

pub fn periodical_tween_short(period: f32, x1: f32, x2: f32, slider: f32) -> f32 {
    let start = periodical(period, x1);
    periodical(
        period,
        start + periodical_difference_short(period, x1, x2) * slider,
    )
}

pub fn periodical_tween_long(period: f32, x1: f32, x2: f32, slider: f32) -> f32 {
    let start = periodical(period, x1);
    let difference = periodical_difference_long(period, x1, x2);
    periodical(period, start + difference * slider)
}

pub fn tween_color_rgba(color1: Color, color2: Color, slide: f32) -> Color {
    Color::rgba(
        tween(color1.r(), color2.r(), slide),
        tween(color1.g(), color2.g(), slide),
        tween(color1.b(), color2.b(), slide),
        tween(color1.a(), color2.a(), slide),
    )
}

pub fn tween_color_hsla_short(color1: Color, color2: Color, slide: f32) -> Color {
    Color::hsla(
        periodical_tween_short(360.0, color1.h(), color2.h(), slide),
        tween(color1.s(), color2.s(), slide),
        tween(color1.l(), color2.l(), slide),
        tween(color1.a(), color2.a(), slide),
    )
}

pub fn tween_color_hsla_long(color1: Color, color2: Color, slide: f32) -> Color {
    Color::hsla(
        periodical_tween_long(360.0, color1.h(), color2.h(), slide),
        tween(color1.s(), color2.s(), slide),
        tween(color1.l(), color2.l(), slide),
        tween(color1.a(), color2.a(), slide),
    )
}

pub fn blend_color(color1: Color, color2: Color) -> Color {
    Color::rgba(
        (color1.r() + color2.r()) / 2.0,
        (color1.g() + color2.g()) / 2.0,
        (color1.b() + color2.b()) / 2.0,
        (color1.a() + color2.a()) / 2.0,
    )
}

// ===========================================================
// === CRATE ONLY ===

/// Returns if a string represents a numerical ID in the tree
pub(crate) fn is_numerical_id(str: &str) -> bool {
    match str.chars().nth(0) {
        Some(value) => value == '#',
        None => false,
    }
}

/// Same as `split_once`, but inverted.
pub(crate) fn split_last(string: &str, delimiter: &str) -> (String, String) {
    let str_list: Vec<&str> = string.split(delimiter).collect();
    let mut output = String::new();
    let mut is_first = true;
    for x in str_list.iter().take(str_list.len() - 1) {
        if !is_first {
            output += delimiter
        } else {
            is_first = false
        };
        output += x;
    }
    (output, String::from(str_list[str_list.len() - 1]))
}

/// ### Extract ID
/// This will extract id from numeric path
pub(super) fn extract_id(str: &str) -> Result<usize, String> {
    match str.chars().nth(0) {
        Some(_) => match str::parse::<usize>(&str[1..]) {
            Ok (value) => Ok (value),
            Err (_) => Err (format!("{} caused syntax error!", str))
        },
        None => Err (format!("This is not a numeric path!")),
    }
}