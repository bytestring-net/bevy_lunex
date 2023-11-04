// ===========================================================
// === MOD ===

mod code;

// ===========================================================
// === EXPORT ===

pub use code::color::{tween_color_rgba/*, tween_color_hsla_long, tween_color_hsla_short*/};

pub use code::conversion::AsLunexVec2;
pub use code::conversion::InvertX;
pub use code::conversion::InvertY;
pub use code::conversion::InvertXY;

pub use code::cursor::Cursor;
pub use code::cursor::cursor_update;

pub use code::system::{tree_compute, tree_pull_window, element_update};
pub use code::system::LunexUiPlugin;
pub use code::system::Size;

pub use code::debug::LunexUiDebugPlugin2D;

// ===========================================================
// === PRELUDE ===

pub mod prelude {
    pub use super::{tween_color_rgba/*, tween_color_hsla_long, tween_color_hsla_short*/};

    pub use super::AsLunexVec2;

    pub use super::Cursor;

    pub use super::LunexUiPlugin;
    pub use super::Size;
    pub use super::{tree_compute, tree_pull_window, element_update};
    pub use super::LunexUiDebugPlugin2D;
}
