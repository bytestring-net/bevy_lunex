// ===========================================================
// === MOD ===

mod code;

// ===========================================================
// === EXPORT ===

pub use code::container::{WindowLayout, RelativeLayout, SolidLayout};
pub use code::container::SolidScale;
pub use code::container::LayoutPackage;
pub use code::container::{Position, Container};

pub use code::tree::{UiTree, UiBranch, Data};

pub use code::types::LunexError;

pub use code::widget::Widget;

// ===========================================================
// === PRELUDE ===

pub mod prelude {
    pub use super::{WindowLayout, RelativeLayout, SolidLayout};
    pub use super::{SolidScale, LayoutPackage};
    pub use super::UiTree;
    pub use super::Widget;
}

// ??????

pub(crate) use code::util::{is_numerical_id, split_last, extract_id};
