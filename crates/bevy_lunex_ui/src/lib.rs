// ===========================================================
// === MOD ===

mod code;

// ===========================================================
// === EXPORT ===

pub use code::color::{tween_color_rgba/*, tween_color_hsla_long, tween_color_hsla_short*/};

pub use code::conversion::AsLunexVec2;

pub use code::cursor::Cursor;
pub use code::cursor::cursor_update;

pub use code::system::{tree_compute, tree_pull_window, element_update};
pub use code::system::LunexUiPlugin;
pub use code::system::Rectangle;

pub use code::debug::LunexUiDebugPlugin;

// ===========================================================
// === PRELUDE ===

pub mod prelude {
    pub use super::{tween_color_rgba/*, tween_color_hsla_long, tween_color_hsla_short*/};

    pub use super::AsLunexVec2;

    pub use super::Cursor;

    pub use super::LunexUiPlugin;
    pub use super::Rectangle;
    pub use super::{tree_compute, tree_pull_window, element_update};
    pub use super::LunexUiDebugPlugin;
}
