mod common;
pub use common::*;

mod declarative;
pub use declarative::*;

mod parametric;
pub use parametric::*;

pub mod prelude {
    pub use super::Div;
    pub use super::Layout;
    pub use super::{Align, Cover};
    pub use super::{FlexBox, FlexDirection, FlexJustify};

    #[allow(non_snake_case)]
    pub mod UiLayout {
        pub use super::super::{Window, Solid, Div};
    }
}