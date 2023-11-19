// ===========================================================
// === MOD ===

mod code;

// ===========================================================
// === EXPORT ===

pub use code::color::{tween_color_rgba, tween_color_hsla_long, tween_color_hsla_short};

pub use code::conversion::InvertX;
pub use code::conversion::InvertY;
pub use code::conversion::InvertXY;

pub use code::cursor::Cursor;
pub use code::cursor::{cursor_update, cursor_update_texture};

pub use code::system::{tree_compute, tree_pull_window, element_update};
pub use code::system::LunexUiPlugin2D;
pub use code::system::LunexUiPlugin2DShared;
pub use code::system::LunexUiPlugin2DGeneric;

pub use code::debug::LunexUiDebugPlugin2D;
pub use code::debug::LunexUiDebugPlugin2DShared;
pub use code::debug::LunexUiDebugPlugin2DGeneric;

pub use code::implementations::UiTreeUi;

// ===========================================================
// === PRELUDE ===

pub mod prelude {
    pub use super::{tween_color_rgba, tween_color_hsla_long, tween_color_hsla_short};

    pub use super::InvertX;
    pub use super::InvertY;
    pub use super::InvertXY;

    pub use super::Cursor;

    pub use super::{tree_compute, tree_pull_window, element_update};

    pub use super::LunexUiPlugin2D;
    pub use super::LunexUiDebugPlugin2D;

    pub use super::UiTreeUi;
}
