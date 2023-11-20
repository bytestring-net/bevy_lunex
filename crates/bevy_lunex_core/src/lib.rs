// ===========================================================
// === MOD ===
pub const LEVEL_RENDER_DEPTH_START: f32 = 100.0;
pub const LEVEL_RENDER_DEPTH_DEFFERENCE: f32 = 10.0;

mod code;

// ===========================================================
// === EXPORT ===

pub use code::container::{WindowLayout, RelativeLayout, SolidLayout};
pub use code::container::{SolidScale, LayoutPackage};
pub use code::container::{Position, Container};
pub use code::tree::{UiTree, UiBranch, UiT, UiD};
pub use code::types::{LunexError, DataWrap, Size, Modifier};
pub use code::widget::Widget;


// ===========================================================
// === PRELUDE ===

pub mod prelude {
    pub use super::{WindowLayout, RelativeLayout, SolidLayout};
    pub use super::{SolidScale, LayoutPackage};
    pub use super::{UiTree, UiT, UiD};
    pub use super::{LunexError, Modifier};
    pub use super::Widget;
}

#[cfg(test)]
pub mod test {
    use super::*;
    use bevy::prelude::*; 
    #[test]
    fn run () {
        let mut tree:UiTree<f32> = UiTree::new("Ui");
        tree.create_branch("Widget 1", RelativeLayout::new()).unwrap();
        Widget::create(&mut tree, "Widget 2", RelativeLayout::new()).unwrap();

        Widget::create(&mut tree, "Widget 2/Widget 3", RelativeLayout::new()).unwrap();

        tree.compute(Vec2::splat(0.0), 100.0, 100.0);
        println!("{}", tree.tree());

        let con = tree.borrow_branch("Widget 2/Widget 3").unwrap().get_container();
        assert_eq!(100.0, con.width());
    }
}
