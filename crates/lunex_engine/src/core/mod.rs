mod compute;
pub use compute::*;

mod structs;
pub use structs::*;

mod traits;
pub use traits::*;

mod value;
pub use value::*;


// #======================#
// #=== PRELUDE EXPORT ===#

pub mod prelude {

    // #============================#
    // #=== ALL DEFAULT UI UNITS ===#

    pub use super::UiValue;
    pub use super::{Ab, Rl, Rw, Rh, Em, Sp, Vw, Vh};


    // #===================#
    // #=== ABSTRACTION ===#

    pub use super::{UiNode, UiTree};
    pub use super::{MasterData, NodeData, NoData};
    pub use super::MainUi;


    // Necessity ?
    pub use super::UiError;
    pub use super::{Rectangle2D, Rectangle3D};

    pub use super::BuildAsNode;


    // #===================================#
    // #=== SAFE CODE-BASED NODE ACCESS ===#

    pub use super::{UiNodeCreationTrait, UiNodeDataTrait, UiNodeTreeInitTrait};
    pub use crate::nodes::prelude::{NodeGeneralTrait, NodeDisplayTrait};
}