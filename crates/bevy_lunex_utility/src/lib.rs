// ===========================================================
// === MOD ===

mod code;

// ===========================================================
// === EXPORT ===

pub use code::element::Element;
pub use code::element::{ImageParams, ImageElementBundle, TextParams, TextElementBundle};

pub use code::grid::GridParams;
pub use code::grid::{grid_generate, grid_generate_inside};


// ===========================================================
// === PRELUDE ===

pub mod prelude {
    pub use super::Element;
    pub use super::{ImageParams, ImageElementBundle, TextParams, TextElementBundle};

    pub use super::GridParams;
    pub use super::{grid_generate, grid_generate_inside};

    pub use crate::textrow;
    pub use crate::textgrid;
}