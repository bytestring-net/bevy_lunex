// ===========================================================
// === MOD ===

mod code;

// ===========================================================
// === EXPORT ===

pub use code::container::{WindowLayout, RelativeLayout, SolidLayout};
pub use code::container::SolidScale;
pub use code::container::LayoutPackage;
pub use code::container::{Position, Container};

pub use code::tree::{UiTree, UiBranch, UiT, UiD};

pub use code::types::LunexError;

//pub use code::widget::Widget;

// ===========================================================
// === CRATE SPECIFIC ===

pub(crate) use code::util::split_last;

// ===========================================================
// === PRELUDE ===

pub mod prelude {
    pub use super::{WindowLayout, RelativeLayout, SolidLayout};
    pub use super::{SolidScale, LayoutPackage};
    pub use super::LunexError;
    pub use super::UiTree;
    pub use super::Widget;
}

#[cfg(test)]
pub mod test {
    use super::*;
    use bevy::prelude::*;    
    #[test]
    fn run () {
        let mut tree = UiTree::new("Ui");
        tree.create_branch("Widget 1", RelativeLayout::new()).unwrap();

        tree.compute(Vec2::splat(0.0), 100.0, 100.0);

        let con = tree.borrow_branch("Widget 1").unwrap().get_container();
        assert_eq!(100.0, con.get_position().width);
    }
}
