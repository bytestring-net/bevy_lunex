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
        textgrid,
        textrow,

        core::widget::Widget,
        core::container::layout,
        core::container::SolidScale,
        core::container::LayoutPackage,

        core::util::AsLunexVec2,
        core::util::tween,
        core::util::{tween_color_rgba, tween_color_hsla_short, tween_color_hsla_long},


        utilities::export::{ImageParams, ImageElementBundle, TextParams, TextElementBundle, element_update},
        utilities::export::{GridParams, grid_generate, grid_generate_inside},

        core::tree::{UiTree, BranchError, Data, hierarchy_update},
        utilities::export::{
            LunexDebugPlugin,
            Cursor,
            cursor_update,
        },
    };
}
