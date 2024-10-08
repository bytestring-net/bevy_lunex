use std::marker::PhantomData;

use crate::{import::*, NiceDisplay, UiStack};
use bevy::ecs::component::Component;
use bevy::math::FloatExt;
use colored::Colorize;

use crate::nodes::prelude::*;
use crate::layout::Layout;


// #==================#
// #=== ERROR TYPE ===#

/// **Ui error** - Type returned when there is an error.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum UiError {
    /// Error that occurs when something went wrong with NodeTree.
    #[error("NodeTree error: {0}")]
    NodeError(NodeError),
}
impl From<NodeError> for UiError {
    fn from(value: NodeError) -> Self {
        UiError::NodeError(value)
    }
}


// #============================#
// #=== COMPUTED LAYOUT TYPE ===#

/// **Rectangle 2D** - Contains computed values from node layout.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rectangle2D {
    pub pos : Vec2,
    pub size: Vec2,
}
impl Rectangle2D {
    pub fn lerp(self, rhs: Self, lerp: f32) -> Self {
        Rectangle2D {
            pos: self.pos.lerp(rhs.pos, lerp),
            size: self.size.lerp(rhs.size, lerp),
        }
    }
}
impl Rectangle2D {
    /// A new empty [`Rectangle2D`]. Has `0` size. 
    pub const EMPTY: Rectangle2D = Rectangle2D { pos : Vec2::ZERO, size: Vec2::ZERO };
    /// Creates new empty Window layout.
    pub const fn new() -> Self {
        Rectangle2D::EMPTY
    }
    /// Replaces the position with the new value.
    pub fn with_pos(mut self, pos: impl Into<Vec2>) -> Self {
        self.pos = pos.into();
        self
    }
    /// Replaces the x position with the new value.
    pub fn with_x(mut self, width: f32) -> Self {
        self.pos.x = width;
        self
    }
    /// Replaces the y position with the new value.
    pub fn with_y(mut self, height: f32) -> Self {
        self.pos.y = height;
        self
    }
    /// Replaces the size with the new value.
    pub fn with_size(mut self, size: impl Into<Vec2>) -> Self {
        self.size = size.into();
        self
    }
    /// Replaces the width with the new value.
    pub fn with_width(mut self, width: f32) -> Self {
        self.size.x = width;
        self
    }
    /// Replaces the height with the new value.
    pub fn with_height(mut self, height: f32) -> Self {
        self.size.y = height;
        self
    }    
}
impl From<Rectangle2D> for Rectangle3D {
    fn from(val: Rectangle2D) -> Self {
        Rectangle3D {
            pos: val.pos.extend(0.0),
            size: val.size,
            ..Default::default()
        }
    }
}


/// **Rectangle 3D** - Contains computed values from node layout.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rectangle3D {
    pub pos : Vec3,
    pub size: Vec2,
    pub roll: f32,
    pub yaw : f32,
    pub tilt: f32,
}
impl Rectangle3D {
    pub fn lerp(self, rhs: Self, lerp: f32) -> Self {
        Rectangle3D {
            pos: self.pos.lerp(rhs.pos, lerp),
            size: self.size.lerp(rhs.size, lerp),
            roll: self.roll.lerp(rhs.roll, lerp),
            yaw: self.yaw.lerp(rhs.yaw, lerp),
            tilt: self.tilt.lerp(rhs.tilt, lerp),
        }
    }
}
impl From<Rectangle3D> for Rectangle2D {
    fn from(val: Rectangle3D) -> Self {
        Rectangle2D {
            pos: val.pos.truncate(),
            size: val.size,
        }
    }
}
impl NiceDisplay for Rectangle3D {
    fn to_nicestr(&self) -> String {
        let text = format!("[pos: {} size: {}]", self.pos, self.size);
        format!("{} {}", "Computed".bright_magenta(), text.black())
    }
}


// #============================================#
// #=== TYPE ABSTRACTION OVER NODE PRIMITIVE ===#


/// A struct, `"Document"` / `"DOM"` in web-terms, that contains all layout data. If you want any entity to have it's own ui layout,
/// add this component to it. The subjects of this layout structure are the entities children.
/// 
/// This type wraps around [`NodeTree`] with generics set to [`MasterData`] & [`NodeData`]. [`NodeTree`] is used as the primitive and
/// ui logic is implemented on top of this type.
/// ## 📏 Structure
/// All data is stored inside inner tree-like hierarchy. Each node can store `custom user data` together with `layout` and multiple `subnodes`.
/// ```text
/// > UiTree
///  |-> Node_1
///  |    |-> Node_2
///  |    |-> Node_3
///  |    |    |-> Node_4
///  |-> Node_5
///  |    |-> Node_6
/// ```
/// ## ⚙️ Paths
/// Paths are strings that are passed to the methods to retrive and mutate data.
/// For example `"foo/bar/bar"` is a valid path syntax. You need to construct paths always
/// from the point of view of the struct we pass them to. For example on the previous hierarchy:
/// 
/// If you want to access `Node_4`, you use path `"Node_1/Node_3/Node_4"` on `UiTree` struct.
/// You can also use `"Node_3/Node_4"` on `Node_1` struct to get the same result.
/// 
/// Whitespaces are allowed in paths, but are not encouraged.
/// Putting a dot as first symbol like this `".name"` will hide the node from the tree.
/// Just `"."` will refer to the same node. `".."` is not supported and is actually a valid name.
/// 
/// You can also not specify the name when creating a node. That means the name will be generated.
/// The format is as follows `".||#:N"` with `N` being the `.len()` of the `nodes` hashmap.
/// ## 📦 Types
/// * Generic `(T)` - Marker for this UiTree
/// * Generic `(N)` - Node data schema struct defining what node-specific data can be stored in [`Node`]
/// ## ⚠️ Warning
/// Please refrain from manually using `".||#:0"`, `".||#:1"`, `".||#:2"`, _and so on.._ as names or [`NodeGeneralTrait::add_node`] will return errors.
pub type UiTree<T = MainUi, N = NoData> = NodeTree<MasterData<T>, NodeData<N>>;

/// A struct representing organized data in [`UiTree`].
pub type UiNode<N = NoData> = Node<NodeData<N>>;


// #====================================#
// #=== DIFFERENT DATA TYPE GENERICS ===#

#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct MainUi;

/// Empty type to tell the compiler that there is no data stored in the node.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct NoData;


/// A struct holding all data appended to [`UiTree`]. Responsible for storing settings, scaling, theme, etc.
/// Every [`UiTree`] needs to have this to work properly.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct MasterData<T> {
    /// Marker for query filtering
    pub marker: PhantomData<T>,
    /// Scale of the [`crate::Abs`] unit.
    pub abs_scale: f32,
    /// Default font size for all subnodes to use (Rem unit scaling).
    pub font_size: f32,
}
impl <T> Default for MasterData<T> {
    fn default() -> Self {
        MasterData {
            marker: PhantomData,
            abs_scale: 1.0,
            font_size: 16.0,
        }
    }
}
impl <T> NiceDisplay for MasterData<T> {
    fn to_nicestr(&self) -> String {
        format!("{}", self.abs_scale)
    }
}


/// A struct holding all data appended to [`UiNode`]. Responsible for storing layout, custom data, cache, etc.
/// Every [`UiNode`] needs to have this to work properly.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct NodeData<N:Default + Component> {
    /// Optional data the user can append.
    pub data: Option<N>,
    /// Calculated rectangle from layout.
    pub rectangle: Rectangle3D,
    /// Layouts of this node.
    pub layout: HashMap<usize, Layout>,
    pub layout_index: [usize; 2],
    pub layout_tween: f32,

    /// Layout of subnodes and how to stack them.
    pub stack: UiStack,
    /// Optional font size to overwrite the inherited master font size.
    pub font_size: Option<f32>,
    /// Value that will be relatively applied to Z after layout compute.
    pub depth_bias: f32,
    /// Size of the content to wrap around. Affects this node's size only if the layout is parametric (Div).
    pub content_size: Vec2,
}
impl <N:Default + Component> Default for NodeData<N> {
    fn default() -> Self {
        NodeData {
            data: Default::default(),
            rectangle: Default::default(),
            layout: HashMap::from([(0, Layout::default())]),
            layout_index: Default::default(),
            layout_tween: Default::default(),
            stack: Default::default(),
            font_size: Default::default(),
            depth_bias: Default::default(),
            content_size: Default::default(),
        }
    }
}
impl <N:Default + Component> NodeData<N> {
    pub fn new() -> NodeData<N> {
        NodeData::default()
    }
}
impl <N:Default + Component> NiceDisplay for NodeData<N> {
    fn to_nicestr(&self) -> String {
        let mut st = String::new();
        for l in &self.layout {
            st += &l.1.to_nicestr();
            st += " | "
        }
        format!("{} {} {}", st, "|||".black(), self.rectangle.to_nicestr())
    }
}