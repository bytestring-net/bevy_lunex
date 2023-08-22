pub mod core;
pub mod utilities;

pub use crate::core::{
    util::{
        blend_color, tween, tween_color_hsla_long, tween_color_hsla_short, tween_color_rgba,
        AsLunexVec2,
    },
    container::{Container, layout, LayoutPackage},
    tree::{UiTree, Branch, BranchError, Data},
    widget::Widget,
};

pub mod prelude {
    pub use crate::{

        // Macros export
        textgrid,
        textrow,
        
        // Export core
        core::widget::Widget,
        core::container::{layout, SolidScale, LayoutPackage},
        core::tree::{UiTree, BranchError, Data, hierarchy_update},

        // Some of it will get moved to Mathia crate
        core::util::AsLunexVec2,
        core::util::tween,
        core::util::{tween_color_rgba, tween_color_hsla_short, tween_color_hsla_long},

        // Reorganize in the future
        utilities::export::{ImageParams, ImageElementBundle, TextParams, TextElementBundle, element_update},
        utilities::export::{GridParams, grid_generate, grid_generate_inside},
        utilities::export::{LunexDebugPlugin, Cursor, cursor_update},
    };
}
