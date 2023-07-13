#![allow(non_snake_case)]

pub use crate::ui_widget::Widget;
pub use crate::ui_widget::WidgetListStyle;

pub use crate::ui_core::Data;
pub use crate::ui_core::Hierarchy;
pub use crate::ui_core::hierarchy_update;

pub use crate::ui_cursor::Cursor;
pub use crate::ui_cursor::cursor_update;

pub use crate::ui_element::Element;
pub use crate::ui_element::ElementBundle;


pub use crate::ui_container::Scale;

pub mod Layout {
    pub use crate::ui_container::Relative;
    pub use crate::ui_container::Window;
    pub use crate::ui_container::Solid;
}

pub use crate::general::tween;
pub use crate::general::vec_convert;
pub use crate::general::lunex_setup_debug;
pub use crate::general::lunex_update_debug;
pub use crate::general::lunex_camera_move_debug;
pub use crate::general::LunexDebugPlugin;


//# ONLY FOR USE INSIDE THE LIBRARY
pub (in crate) use ahash::AHashMap as HashMap;
pub (in crate) use crate::general::is_absolute;
pub (in crate) use crate::general::split_last;