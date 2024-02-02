use std::borrow::Borrow;

use bevy::{prelude::*, render::primitives::Aabb, sprite::Anchor, text::{Text2dBounds, TextLayoutInfo}};
use lunex_engine::prelude::*;

pub type UiStack = FlexBox;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct UiContent {
    pub size: Vec2,
}
impl UiContent {
    pub fn new(size: impl Into<Vec2>) -> Self {
        UiContent { size: size.into() }
    }
}


#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct MovableByCamera;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct Element;


#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct UiLink {
    pub path: String,
}
impl UiLink {
    pub fn path( path: impl Borrow<str>) -> Self {
        UiLink { path: path.borrow().to_string() }
    }
    pub fn add( &self, path: impl Borrow<str>) -> Self {
        UiLink { path: format!("{}/{}", self.path, path.borrow()) }
    }
    pub fn new( &self) -> Self {
        UiLink { path: format!("{}/", self.path) }
    }
}


#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct Dimension {
    pub size: Vec2,
}
impl Dimension {
    pub fn new(size: impl Into<Vec2>) -> Self {
        Dimension {
            size: size.into()
        }
    }
}





#[derive(Bundle, Debug, Default, Clone, PartialEq)]
pub struct UiTreeBundle <M: Default + Component, N: Default + Component, T: Component> {
    pub tree: UiTree<M, N>,
    pub marker: T,
    pub transform: Transform,
    pub dimension: Dimension,

    pub global_transform: GlobalTransform,
    pub inherited_visibility: InheritedVisibility,
}
impl <M: Default + Component, N: Default + Component, T: Component + Default> From<UiTree<M, N>> for UiTreeBundle<M, N, T> {
    fn from(value: UiTree<M, N>) -> Self {
        UiTreeBundle::<M, N, T> {
            tree: value,
            ..default()
        }
    }
}






#[derive(Bundle, Debug, Default, Clone)]
pub struct UiMaterial3dBundle {
    /// Marks this as node element.
    pub element: Element,
    /// Quad mesh that is generated every time node is changed.
    pub mesh: Handle<Mesh>,
    /// The material used for the quad.
    pub material: Handle<StandardMaterial>,
    /// Image boundary for culling.
    pub aabb: Aabb,
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// The transform of the quad.
    pub transform: Transform,
    /// The global transform of the quad.
    pub global_transform: GlobalTransform,
    /// The visibility properties of the quad.
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering.
    pub view_visibility: ViewVisibility,
}
impl From<Handle<StandardMaterial>> for UiMaterial3dBundle {
    fn from(value: Handle<StandardMaterial>) -> Self {
        UiMaterial3dBundle {
            material: value,
            ..default()
        }
    }
}
impl UiMaterial3dBundle {
    pub fn from_image(materials: &mut ResMut<'_, Assets<StandardMaterial>>, value: Handle<Image>) -> Self {
        UiMaterial3dBundle {
            material: materials.add(StandardMaterial { base_color_texture: Some(value), unlit: true, ..default() }),
            ..default()
        }
    }
    pub fn from_transparent_image(materials: &mut ResMut<'_, Assets<StandardMaterial>>, value: Handle<Image>) -> Self {
        UiMaterial3dBundle {
            material: materials.add(StandardMaterial { base_color_texture: Some(value), unlit: true, alpha_mode: AlphaMode::Blend, ..default() }),
            ..default()
        }
    }
}


#[derive(Bundle, Clone, Debug, Default)]
pub struct UiImage2dBundle {
    /// Marks this as node element.
    pub element: Element,
    /// Image properties.
    pub sprite: Sprite,
    /// Image texture.
    pub texture: Handle<Image>,
    /// Image boundary for culling.
    pub aabb: Aabb,
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// The transform of the image.
    pub transform: Transform,
    /// The global transform of the image.
    pub global_transform: GlobalTransform,
    /// The visibility properties of the image.
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering.
    pub view_visibility: ViewVisibility,
}
impl From<Handle<Image>> for UiImage2dBundle {
    fn from(value: Handle<Image>) -> Self {
        UiImage2dBundle {
            texture: value,
            ..default()
        }
    }
}


#[derive(Bundle, Clone, Debug, Default)]
pub struct UiText2dBundle {
    /// Marks this as node element.
    pub element: Element,
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// Contains the text.
    pub text: Text,
    /// How the text is positioned relative to its transform.
    pub text_anchor: Anchor,
    /// The maximum width and height of the text.
    pub text_2d_bounds: Text2dBounds,
    /// The transform of the text.
    pub transform: Transform,
    /// The global transform of the text.
    pub global_transform: GlobalTransform,
    /// The visibility properties of the text.
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering.
    pub view_visibility: ViewVisibility,
    /// Contains the size of the text and its glyph's position and scale data. Generated via [`TextPipeline::queue_text`]
    pub text_layout_info: TextLayoutInfo,
}
