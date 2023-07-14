use bevy::prelude::*;

// ===========================================================
// === GENERAL ===

/// ## Description
/// This function is used for translating Vec2 from Bevy 2D coordinate system to Lunex coordinate system.
/// It is necessary to go through this step if you want entities to be able to interact with Lunex.
/// 
/// Example of this is the cursor entity which has an unmodified [`Transform`] component.
/// Due to the nature of Bevy 2D, the y+ direction is upwards instead of downwards.
/// 
/// * This function will invert the Y component.
/// * In addition it will offset the values because Lunex always starts at 0.
///
/// ## Examples
/// ```
/// let system = Hierarchy::new();
/// let offset = Vec2::new( -window.size.x / 2.0, window.size.y / 2.0 );
/// let cursor_position = Vec2::new(40.0, 20.0);    //Extracted from transform.translation.x, y
///
/// //Returns bool
/// widget.is_within(&system, "", &vec_convert(cursor_position, &offset)).unwrap();
/// ```
pub fn vec_convert (vec2: &Vec2, offset: &Vec2) -> Vec2 {
    Vec2::new(vec2.x - offset.x, offset.y - vec2.y)
}

pub fn tween (value_1: f32, value_2: f32, slide: f32) -> f32 {
    let diff = value_2 - value_1;
    value_1 + diff * slide
}


// ===========================================================
// === CRATE ONLY ===

pub (in super) fn is_absolute (str: &str) -> bool {
    match str.chars().nth(0) {
        Some (value) => {
            value == '#'
        },
        None => false,
    }
}

pub (in super) fn split_last (string: &str, delimiter: &str ) -> (String, String) {
    let str_list: Vec<&str> =  string.split(delimiter).collect();
    let mut output = String::new();
    let mut is_first = true;
    for x in str_list.iter().take(str_list.len() - 1){
        if !is_first {output += delimiter} else {is_first = false};
        output += x;
    }
    (output, String::from(str_list[str_list.len() - 1]))
}