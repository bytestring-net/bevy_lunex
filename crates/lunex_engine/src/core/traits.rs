use std::borrow::Borrow;

use bevy::ecs::component::Component;

use crate::nodes::prelude::*;
use crate::MasterData;
use crate::import::*;

use super::{UiNode, UiTree, NodeData};


// #===================#
// #=== GENERAL USE ===#

/// **Nice display** - Special trait for formatted console debug output with colors.
pub trait NiceDisplay {
    /// Output the data in a formatted string using the `colorise` crate.
    fn to_nicestr(&self) -> String;
}


/// **Y invert** - Trait for implementing Y value inversion function for `glam` types.
/// Required due to inverted coordinate system between Ui and Bevy.
pub trait YInvert {
    /// Multiplies the y value by `-1`
    fn invert_y(self) -> Self;
}
impl YInvert for Vec2 {
    fn invert_y(mut self) -> Self {
        self.y *= -1.0;
        self
    }
}
impl YInvert for Vec3 {
    fn invert_y(mut self) -> Self {
        self.y *= -1.0;
        self
    }
}
impl YInvert for Vec4 {
    fn invert_y(mut self) -> Self {
        self.y *= -1.0;
        self
    }
}


// #==========================#
// #=== ABSTRACTION TRAITS ===#

/// Trait that abstracts over [`NodeCreationTrait`] to provide tailored
/// implementations for the primitive in layouting context.
pub trait UiNodeCreationTrait<N:Default + Component> {
    /// Makes new subnode in this node and returns the new subnodes' name.
    /// ## 📌 Note
    /// * Use [`UiNodeCreationTrait::create_ui_node`] for hierarchy creation `(supports path recursion)`
    fn make_ui_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError>;
    /// ## 🚸 Recursive
    /// Creates new subnode in this node or any other subnode and returns the new subnodes' name.
    /// ## 📌 Note
    /// * Use [`UiNodeCreationTrait::make_ui_node`] for direct creation on this node `(no recursion)`
    fn create_ui_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError>;
    /// Borrows subnode from this node. If the node doesn't exist, it creates one.
    /// ## 📌 Note
    /// * Use [`UiNodeCreationTrait::borrow_or_create_ui_node`] for hierarchy retrieval `(supports path recursion)`
    fn obtain_or_create_ui_node(&mut self, name: impl Borrow<str>) -> Result<&UiNode<N>, NodeError>;
    /// Borrows subnode from this node as mut. If the node doesn't exist, it creates one.
    /// ## 📌 Note
    /// * Use [`UiNodeCreationTrait::borrow_or_create_ui_node_mut`] for hierarchy retrieval `(supports path recursion)`
    fn obtain_or_create_ui_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError>;
    /// ## 🚸 Recursive
    /// Borrows subnode from this node or any other subnode. If a node in path doesn't exist, it creates one.
    /// ## 📌 Note
    /// * Use [`UiNodeCreationTrait::obtain_or_create_ui_node`] for direct retrieval on this node `(no recursion)`
    fn borrow_or_create_ui_node(&mut self, path: impl Borrow<str>) -> Result<&UiNode<N>, NodeError>;
    /// ## 🚸 Recursive
    /// Borrows subnode from this node or any other subnode as mut. If a node in path doesn't exist, it creates one.
    /// ## 📌 Note
    /// * Use [`UiNodeCreationTrait::obtain_or_create_ui_node_mut`] for direct retrieval on this node `(no recursion)`
    fn borrow_or_create_ui_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError>;  
}
impl <T, N: Default + Component> UiNodeCreationTrait<N> for UiTree<T, N> {
    fn make_ui_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError>{
        self.node.make_ui_node(name)
    }
    fn create_ui_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError>{
        self.node.create_ui_node(path)
    }
    fn obtain_or_create_ui_node(&mut self, name: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        self.node.obtain_or_create_ui_node(name)
    }
    fn obtain_or_create_ui_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        self.node.obtain_or_create_ui_node_mut(name)
    }
    fn borrow_or_create_ui_node(&mut self, path: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        self.node.borrow_or_create_ui_node(path)
    }
    fn borrow_or_create_ui_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        self.node.borrow_or_create_ui_node_mut(path)
    }
}
impl <N: Default + Component> UiNodeCreationTrait<N> for UiNode<N> {
    fn make_ui_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError> {
        let n = self.make_node(name)?;
        self.insert_data(n.clone(), NodeData::default())?;
        Ok(n)
    }
    fn create_ui_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError> {
        let mut node: UiNode<N> = Node::new();
        node.add_data(NodeData::default());
        self.insert_node(path, Node::new())
    }
    fn obtain_or_create_ui_node(&mut self, name: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        if let Ok(n) = self.make_ui_node(name.borrow()) {
            return self.obtain_node(n)
        }
        self.obtain_node(name)
    }
    fn obtain_or_create_ui_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        if let Ok(n) = self.make_ui_node(name.borrow()) {
            return self.obtain_node_mut(n)
        }
        self.obtain_node_mut(name)
    }
    fn borrow_or_create_ui_node(&mut self, path: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        match path.borrow().split_once('/') {
            None => self.obtain_or_create_ui_node(path),
            Some((name, rempath)) => self.obtain_or_create_ui_node_mut(name)?.borrow_or_create_ui_node(rempath),
        }
    }
    fn borrow_or_create_ui_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        match path.borrow().split_once('/') {
            None => self.obtain_or_create_ui_node_mut(path),
            Some((name, rempath)) => self.obtain_or_create_ui_node_mut(name)?.borrow_or_create_ui_node_mut(rempath),
        }
    }
}

/// Trait that abstracts over [`NodeDataTrait`] to provide tailored
/// implementations for the primitive in layouting context.
pub trait UiNodeDataTrait<N> {
    /// Adds new data to this node and returns the previous data.
    /// ## 📌 Note
    /// * Use [`UiNodeDataTrait::insert_ui_data`] for hierarchy insert `(supports path recursion)`
    /// ## ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] struct that holds layout data + user data.
    ///   Wont happen unless somebody messed with internals using elevated access methods _(not in prelude)_.
    fn add_ui_data(&mut self, data: N) -> Option<N>;
    /// ## 🚸 Recursive
    /// Inserts new data to this node or any other subnode and returns the previous data.
    /// ## 📌 Note
    /// * Use [`UiNodeDataTrait::add_ui_data`] for direct insert on this node `(no recursion)`
    /// ## ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] struct that holds layout data + user data.
    ///   Wont happen unless somebody messed with internals using elevated access methods _(not in prelude)_.
    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: N) -> Result<Option<N>, NodeError>;
    /// Removes data from this node and returns them.
    /// ## 📌 Note
    /// * Use [`UiNodeDataTrait::remove_ui_data`] for hierarchy retrieval `(supports path recursion)`
    /// ## ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] struct that holds layout data + user data.
    ///   Wont happen unless somebody messed with internals using elevated access methods _(not in prelude)_.
    fn take_ui_data(&mut self) -> Option<N>;
    /// ## 🚸 Recursive
    /// Removes data from this node or any other subnode and returns them.
    /// ## 📌 Note
    /// * Use [`UiNodeDataTrait::take_ui_data`] for direct retrieval on this node `(no recursion)`
    /// ## ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] struct that holds layout data + user data.
    ///   Wont happen unless somebody messed with internals using elevated access methods _(not in prelude)_.
    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<N>, NodeError>;
    /// Borrows data from this node.
    /// ## 📌 Note
    /// * Use [`UiNodeDataTrait::borrow_ui_data`] for hierarchy retrieval `(supports path recursion)`
    /// ## ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] struct that holds layout data + user data.
    ///   Wont happen unless somebody messed with internals using elevated access methods _(not in prelude)_.
    fn obtain_ui_data(&self) -> Option<&N>;
    /// Borrows data from this node as mut.
    /// ## 📌 Note
    /// * Use [`UiNodeDataTrait::borrow_ui_data_mut`] for hierarchy retrieval `(supports path recursion)`
    /// ## ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] struct that holds layout data + user data.
    ///   Wont happen unless somebody messed with internals using elevated access methods _(not in prelude)_.
    fn obtain_ui_data_mut(&mut self) -> Option<&mut N>;
    /// ## 🚸 Recursive
    /// Borrows data from this node or any other subnode.
    /// ## 📌 Note
    /// * Use [`UiNodeDataTrait::obtain_ui_data`] for direct retrieval on this node `(no recursion)`
    /// ## ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] struct that holds layout data + user data.
    ///   Wont happen unless somebody messed with internals using elevated access methods _(not in prelude)_.
    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&N>, NodeError>;
    /// ## 🚸 Recursive
    /// Borrows data from this node or any other subnode as mut.
    /// ## 📌 Note
    /// * Use [`UiNodeDataTrait::obtain_ui_data_mut`] for direct retrieval on this node `(no recursion)`
    /// ## ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] struct that holds layout data + user data.
    ///   Wont happen unless somebody messed with internals using elevated access methods _(not in prelude)_.
    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut N>, NodeError>;
}
impl <T, N: Default + Component> UiNodeDataTrait<N> for UiTree<T, N> {
    fn add_ui_data(&mut self, data: N) -> Option<N> {
        self.node.add_ui_data(data)
    }
    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: N) -> Result<Option<N>, NodeError> {
        self.node.insert_ui_data(path, data)
    }
    fn take_ui_data(&mut self) -> Option<N> {
        self.node.take_ui_data()
    }
    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<N>, NodeError> {
        self.node.remove_ui_data(path)
    }
    fn obtain_ui_data(&self) -> Option<&N> {
        self.node.obtain_ui_data()
    }
    fn obtain_ui_data_mut(&mut self) -> Option<&mut N> {
        self.node.obtain_ui_data_mut()
    }
    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&N>, NodeError> {
        self.node.borrow_ui_data(path)
    }
    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut N>, NodeError> {
        self.node.borrow_ui_data_mut(path)
    }
}
impl <N: Default + Component> UiNodeDataTrait<N> for UiNode<N> {
    fn add_ui_data(&mut self, data: N) -> Option<N> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UiNode is missing Ui data!") };
        core::mem::replace(&mut container.data, Some(data))
    }
    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: N) -> Result<Option<N>, NodeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(core::mem::replace(&mut container.data, Some(data)))
    }
    fn take_ui_data(&mut self) -> Option<N> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UiNode is missing Ui data!") };
        container.data.take()
    }
    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<N>, NodeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(container.data.take())
    }
    fn obtain_ui_data(&self) -> Option<&N> {
        let Some(container) = self.obtain_data() else { panic!("This UiNode is missing Ui data!") };
        container.data.as_ref()
    }
    fn obtain_ui_data_mut(&mut self) -> Option<&mut N> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UiNode is missing Ui data!") };
        container.data.as_mut()
    }
    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&N>, NodeError> {
        let Some(container) = self.borrow_data(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(container.data.as_ref())
    }
    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut N>, NodeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(container.data.as_mut())
    }
}

/// Trait that abstracts over [`NodeTreeInitTrait`] to provide tailored
/// implementations for the primitive in layouting context.
pub trait UiNodeTreeInitTrait {
    /// Creates new [`UiTree`] configured for 2D enviroment.
    fn new2d(name: impl Borrow<str>) -> Self;
    /// Creates new [`UiTree`] configured for 3D enviroment.
    fn new3d(name: impl Borrow<str>) -> Self;
}
impl <T, N: Default + Component> UiNodeTreeInitTrait for UiTree<T, N> {
    fn new2d(name: impl Borrow<str>) -> Self {
        let mut tree: UiTree<T, N> = NodeTreeInitTrait::new(name);
        tree.add_topdata(MasterData::default());
        tree.add_data(NodeData::default());
        tree
    }
    fn new3d(name: impl Borrow<str>) -> Self {
        let mut tree: UiTree<T, N> = NodeTreeInitTrait::new(name);
        tree.add_topdata(MasterData {
            abs_scale : 0.001,
            ..Default::default()
        });
        tree.add_data(NodeData::default());
        tree
    }
}
