// ===========================================================
// === MOD ===

mod code;

// ===========================================================
// === EXPORT ===

pub use code::element::Element;
pub use code::element::{ImageParams, ImageElementBundle, TextParams, TextElementBundle};

// ===========================================================
// === PRELUDE ===

pub mod prelude {
    pub use super::Element;
    pub use super::{ImageParams, ImageElementBundle, TextParams, TextElementBundle};
}