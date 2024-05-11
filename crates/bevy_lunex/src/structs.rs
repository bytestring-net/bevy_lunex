use std::{borrow::Borrow, marker::PhantomData};

use bevy::{prelude::*, render::primitives::Aabb, sprite::Anchor, text::{Text2dBounds, TextLayoutInfo}};
use lunex_engine::prelude::*;



// #==================#
// #=== COMPONENTS ===#

/// This struct marks [`UiTree`] entity to receive piped [`Camera`] size and position to its [`Dimension`] and [`Transform`] component.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct MovableByCamera;

/// This struct is used to mark linked UI entities as elements for easier rendering.
/// They are picked up by different systems, that ensure their piped [`Transform`] is centered,
/// instead of being aligned in a top-left corner like the normal UI entities.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct Element;


/// # WIP - used for Div layout
#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct UiContent {
    pub size: Vec2,
}
impl UiContent {
    pub fn new(size: impl Into<Vec2>) -> Self {
        UiContent { size: size.into() }
    }
}


/// This struct is a string reference to a specific node in a parent [`UiTree`].
/// Lunex uses this component to locate what data this entity should be working with.
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct UiLink<T> {
    pub path: String,
    marker: PhantomData<T>,
}
impl <T> UiLink<T> {
    pub fn path( path: impl Borrow<str>) -> Self {
        UiLink {
            path: path.borrow().to_string(),
            marker: PhantomData,
        }
    }
    pub fn add( &self, path: impl Borrow<str>) -> Self {
        UiLink {
            path: format!("{}/{}", self.path, path.borrow()),
            marker: PhantomData,
        }
    }
    pub fn new() -> Self {
        UiLink {
            path: format!("/"),
            marker: PhantomData,
        }
    }
}


/// This struct holds rectangular data. If the component covers some kind of 2D area, it should be stored in this component.
/// Lunex uses this component to mirror node size in & out from parent [`UiTree`].
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


// #====================#
// #=== MAIN BUNDLES ===#

/// Main bundle for spawning `UiTree` entity
#[derive(Bundle, Debug, Default, Clone, PartialEq)]
pub struct UiTreeBundle <M: Default + Component, N: Default + Component, T: Component> {
    /// The ui layout data of the entity and it's children.
    pub tree: UiTree<M, N>,
    /// The marker component for the ui system.
    pub marker: T,
    /// The transform of the entity.
    pub transform: Transform,
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
    /// The visibility properties of the entity.
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering.
    pub view_visibility: ViewVisibility,
}
impl <M: Default + Component, N: Default + Component, T: Component + Default> From<UiTree<M, N>> for UiTreeBundle<M, N, T> {
    fn from(value: UiTree<M, N>) -> Self {
        UiTreeBundle::<M, N, T> {
            tree: value,
            ..default()
        }
    }
}

/// Main bundle for spawning `UiNode` entity as a child of `UiTree` entity
#[derive(Bundle, Debug, Clone, PartialEq)]
pub struct UiNodeBundle<T: Component> {
    /// The marker component for the ui system.
    pub marker: T,
    /// The corresponding path that leads to the node data in parent UiTree entity.
    pub link: UiLink<T>,
    /// The layout to use when computing this node.
    pub layout: UiLayout,
}
impl <T: Component + Default> Default for UiNodeBundle<T> {
    fn default() -> Self {
        UiNodeBundle {
            marker: T::default(),
            link: UiLink::default(),
            layout: UiLayout::default(),
        }
    }
}

/// Additional bundle for `UiNode` entity that provides required components to be renderable.
#[derive(Bundle, Debug, Clone, PartialEq, Default)]
pub struct UiElementBundle {
    /// Marks this as node element.
    pub element: Element,
    /// The transform of the entity.
    pub transform: Transform,
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
    /// The visibility properties of the entity.
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering.
    pub view_visibility: ViewVisibility,
}

/// Additional bundle for `UiNode` entity that provides required components to exist in a 3D world, but not as an element.
#[derive(Bundle, Debug, Clone, PartialEq, Default)]
pub struct UiSpacialBundle {
    /// The transform of the entity.
    pub transform: Transform,
    /// Contains the ui node size.
    pub dimension: Dimension,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
    /// The visibility properties of the entity.
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering.
    pub view_visibility: ViewVisibility,
}

// #=======================#
// #=== SPECIAL BUNDLES ===#

/// Additional bundle for `UiNode` entity that provides 3D material.
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

/// Additional bundle for `UiNode` entity that provides 2D texture.
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

/// Additional bundle for `UiNode` entity that provides 2D text.
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
