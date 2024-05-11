mod layout;
pub use layout::*;

mod stack;
pub use stack::*;

// #======================#
// #=== PRELUDE EXPORT ===#

pub mod prelude {
    pub use super::UiLayout;
    pub use super::{Align, Scaling, Sizing};

    pub use super::UiStack;
    pub use super::{StackDirection, StackMargin};

    #[allow(non_snake_case)]
    pub mod ui {
        pub use super::super::{Boundary, Window, Solid, Div};
    }
}