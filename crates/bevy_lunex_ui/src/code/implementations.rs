use bevy::prelude::*;
use bevy_lunex_core::{UiTree, Size};

pub trait UiTreeUi {
    fn bundle(self) -> (Self, Transform, Size) where Self: Sized;
}

impl <T: Default> UiTreeUi for UiTree<T> {
    fn bundle(self) -> (Self, Transform, Size) where Self: Sized {
        (self, Transform::default(), Size::default())
    }
}