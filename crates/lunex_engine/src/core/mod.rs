mod structs;
pub use structs::*;

mod traits;
pub use traits::*;

mod value;
pub use value::*;

pub mod prelude {

    // Export Ui units
    pub use super::UiValue;
    pub use super::{Ab, Rl, Rw, Rh, Em, Sp, Vw, Vh};

    // Necessity ?
    pub use super::UiError;
    pub use super::{Rectangle2D, Rectangle3D};


    pub use super::{UiNode, UiTree};
    pub use super::{MasterData, NodeData, NoData};

    pub use super::UiNodeTreeComputeTrait;

    pub use super::BuildAsNode;

    // TRAITS THAT ABSTRACT OVER TRAITS THAT ARE NOT SAFE TO USE
    pub use super::{UiNodeCreationTrait, UiNodeDataTrait, UiNodeTreeInitTrait};
    // EXPOSED TRAITS FOR THE USER THAT ARE SAFE TO USE
    pub use crate::nodes::prelude::{NodeGeneralTrait, NodeDisplayTrait};
}