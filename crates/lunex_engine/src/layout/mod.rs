//mod common;
//pub use common::*;

mod layout;
pub use layout::*;

//mod parametric;
//pub use parametric::*;

pub mod prelude {
    pub use super::Layout;
    pub use super::{Align, Scaling};
    //pub use super::{FlexBox, FlexDirection, FlexJustify};

    #[allow(non_snake_case)]
    pub mod UiLayout {
        pub use super::super::{Boundary, Window, Solid};
    }
}