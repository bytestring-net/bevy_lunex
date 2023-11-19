// ===========================================================
// === MOD ===

mod code;

// ===========================================================
// === EXPORT ===

pub use code::element::{Element, ElementBundle};
pub use code::element::{ImageParams, ImageElementBundle, TextParams, TextElementBundle};

//pub use code::grid::GridParams;
pub use code::grid::{GridCell, GridSegment, GridOrientation, Grid};
//pub use code::grid::{grid_generate, grid_generate_inside, grid_generate_solid};


// ===========================================================
// === PRELUDE ===

pub mod prelude {
    pub use super::{Element, ElementBundle};
    pub use super::{ImageParams, ImageElementBundle, TextParams, TextElementBundle};

    //pub use super::GridParams;
    //pub use super::{grid_generate, grid_generate_inside, grid_generate_solid};
    pub use super::{GridCell, GridSegment, GridOrientation, Grid};

    pub use crate::textrow;
    pub use crate::textgrid;

    pub use crate::import_use;
    pub use crate::script_plugin;
}