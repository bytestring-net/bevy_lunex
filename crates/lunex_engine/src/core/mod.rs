mod structs;
pub use structs::*;

mod traits;
pub use traits::*;

pub mod prelude {
    pub use super::{UiNode, UiTree};
    pub use super::{MasterData, NodeData, NoData};

    pub use super::{UiNodeCreationTrait, UiNodeDataTrait, UiNodeTreeInitTrait, UiNodeTreeComputeTrait};
    pub use super::BuildAsNode;

    //RE-EXPORT FROM NODES                          // NEEDS ABSTRACTION
    pub use crate::nodes::prelude::{NodeGeneralTrait, NodeTopDataTrait, NodeDisplayTrait};
}