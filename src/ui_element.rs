#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;

use crate::prelude::Widget;

#[derive(Component, Clone, Debug, Default)]
pub struct Element {
    pub relative: Vec2,
    pub size: Vec2,
}

#[derive(Bundle, Clone, Debug, Default)]
pub struct ElementBundle {
    pub widget: Widget,
    pub element: Element,
    pub transform: Transform,
    pub visibility: Visibility,
    pub global_transform: GlobalTransform,
    pub computed_visibility: ComputedVisibility,
}