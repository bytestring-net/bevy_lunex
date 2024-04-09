use std::borrow::Borrow;
#[allow(unused_imports)]
use super::{NodeTree, Node, NodeError};

// #=========================#
// #=== TRAIT DECLARATION ===#

/// Trait with all node management implementations.
pub trait NodeGeneralTrait<T> {
    /// Adds new subnode to this node and returns the new subnodes' name.
    /// ## 📌 Note
    /// * Use [`NodeGeneralTrait::insert_node`] for hierarchy insert `(supports path recursion)`
    fn add_node(&mut self, name: impl Borrow<str>, node: impl Into<Node<T>>) -> Result<String, NodeError>;
    /// ## 🚸 Recursive
    /// Inserts new subnode to this node or any other subnode and returns the new subnodes' name.
    /// ## 📌 Note
    /// * Use [`NodeGeneralTrait::add_node`] for direct insert on this node `(no recursion)`
    fn insert_node(&mut self, path: impl Borrow<str>, node: impl Into<Node<T>>) -> Result<String, NodeError>;
    /// Removes subnode from this node and returns it.
    /// ## 📌 Note
    /// * Use [`NodeGeneralTrait::take_node`] for hierarchy retrieval `(supports path recursion)`
    fn take_node(&mut self, name: impl Borrow<str>) -> Result<Node<T>, NodeError>;
    /// ## 🚸 Recursive
    /// Removes subnode from this node or any other subnode and returns it.
    /// ## 📌 Note
    /// * Use [`NodeGeneralTrait::take_node`] for direct retrieval on this node `(no recursion)`
    fn remove_node(&mut self, path: impl Borrow<str>) -> Result<Node<T>, NodeError>;
    /// Borrows subnode from this node.
    /// ## 📌 Note
    /// * Use [`NodeGeneralTrait::borrow_node`] for hierarchy retrieval `(supports path recursion)`
    fn obtain_node(&self, name: impl Borrow<str>) -> Result<&Node<T>, NodeError>;
    /// Borrows subnode from this node as mut.
    /// ## 📌 Note
    /// * Use [`NodeGeneralTrait::borrow_node_mut`] for hierarchy retrieval `(supports path recursion)`
    fn obtain_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Node<T>, NodeError>;
    /// ## 🚸 Recursive
    /// Borrows subnode from this node or any other subnode.
    /// ## 📌 Note
    /// * Use [`NodeGeneralTrait::obtain_node`] for direct retrieval on this node `(no recursion)`
    fn borrow_node(&self, path: impl Borrow<str>) -> Result<&Node<T>, NodeError>;
    /// ## 🚸 Recursive
    /// Borrows subnode from this node or any other subnode as mut.
    /// ## 📌 Note
    /// * Use [`NodeGeneralTrait::obtain_node_mut`] for direct retrieval on this node `(no recursion)`
    fn borrow_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Node<T>, NodeError>;
    /// Merges subnodes of supplied node or nodetree into this node.
    /// ## ⚠️ Warning
    /// * Any data that supplied node contains will be dropped.
    /// * Returns error if there is a name collision.
    fn merge(&mut self, node: impl Into<Node<T>>) -> Result<(), NodeError>;
    /// Recursively iterates over all subnodes and returns them in a single vector.
    fn crawl(&self) -> Vec<&Node<T>>;
    /// Generates overview of the inner structure of subnodes as a printable string.
    /// 
    /// You can supply additional parameters like `show-hidden`.
    /// ## 📌 Note
    /// * Prefer [`NodeDisplayTrait::tree`] method instad if (`T`) implements [`crate::NiceDisplay`]
    fn tree_node(&self, params: impl Borrow<str>) -> String;
    /// Returns name of the node. `Cached` & `Read-only`.
    /// ## ⚠️ Warning
    /// * Not guaranteed to be correct if node is not put inside the hierarchy correctly.
    fn get_name(&self) -> &String;
    /// Returns depth within the hierarchy. `Cached` & `Read-only`.
    /// ## ⚠️ Warning
    /// * Not guaranteed to be correct if node is not put inside the hierarchy correctly.
    fn get_path(&self) -> &String;
    /// Returns full path without the name. `Cached` & `Read-only`.
    /// ## ⚠️ Warning
    /// * Not guaranteed to be correct if node is not put inside the hierarchy correctly.
    fn get_depth(&self) -> f32;
}

/// Trait with all node creation implementations.
/// Lunex abstacts over this trait with another trait.
pub trait NodeCreationTrait<T> {
    /// Makes new subnode in this node and returns the new subnodes' name.
    /// ## 📌 Note
    /// * Use [`NodeCreationTrait::create_node`] for hierarchy creation `(supports path recursion)`
    fn make_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError>;
    /// ## 🚸 Recursive
    /// Creates new subnode in this node or any other subnode and returns the new subnodes' name.
    /// ## 📌 Note
    /// * Use [`NodeCreationTrait::make_node`] for direct creation on this node `(no recursion)`
    fn create_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError>;
    /// Borrows subnode from this node. If the node doesn't exist, it creates one.
    /// ## 📌 Note
    /// * Use [`NodeCreationTrait::borrow_or_create_node`] for hierarchy retrieval `(supports path recursion)`
    fn obtain_or_create_node(&mut self, name: impl Borrow<str>) -> Result<&Node<T>, NodeError>;
    /// Borrows subnode from this node as mut. If the node doesn't exist, it creates one.
    /// ## 📌 Note
    /// * Use [`NodeCreationTrait::borrow_or_create_node_mut`] for hierarchy retrieval `(supports path recursion)`
    fn obtain_or_create_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Node<T>, NodeError>;
    /// ## 🚸 Recursive
    /// Borrows subnode from this node or any other subnode. If a node in path doesn't exist, it creates one.
    /// ## 📌 Note
    /// * Use [`NodeCreationTrait::obtain_or_create_node`] for direct retrieval on this node `(no recursion)`
    fn borrow_or_create_node(&mut self, path: impl Borrow<str>) -> Result<&Node<T>, NodeError>;
    /// ## 🚸 Recursive
    /// Borrows subnode from this node or any other subnode as mut. If a node in path doesn't exist, it creates one.
    /// ## 📌 Note
    /// * Use [`NodeCreationTrait::obtain_or_create_node_mut`] for direct retrieval on this node `(no recursion)`
    fn borrow_or_create_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Node<T>, NodeError>;    
}

/// Trait with all node data management implementations.
/// Lunex abstacts over this trait with another trait.
pub trait NodeDataTrait<T> {
    /// Adds new data to this node and returns the previous data.
    /// ## 📌 Note
    /// * Use [`NodeDataTrait::insert_data`] for hierarchy insert `(supports path recursion)`
    fn add_data(&mut self, data: T) -> Option<T>;
    /// ## 🚸 Recursive
    /// Inserts new data to this node or any other subnode and returns the previous data.
    /// ## 📌 Note
    /// * Use [`NodeDataTrait::add_data`] for direct insert on this node `(no recursion)`
    fn insert_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeError>;
    /// Removes data from this node and returns them.
    /// ## 📌 Note
    /// * Use [`NodeDataTrait::remove_data`] for hierarchy retrieval `(supports path recursion)`
    fn take_data(&mut self) -> Option<T>;
    /// ## 🚸 Recursive
    /// Removes data from this node or any other subnode and returns them.
    /// ## 📌 Note
    /// * Use [`NodeDataTrait::take_data`] for direct retrieval on this node `(no recursion)`
    fn remove_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeError>;
    /// Borrows data from this node.
    /// ## 📌 Note
    /// * Use [`NodeDataTrait::borrow_data`] for hierarchy retrieval `(supports path recursion)`
    fn obtain_data(&self) -> Option<&T>;
    /// Borrows data from this node as mut.
    /// ## 📌 Note
    /// * Use [`NodeDataTrait::borrow_data_mut`] for hierarchy retrieval `(supports path recursion)`
    fn obtain_data_mut(&mut self) -> Option<&mut T>;
    /// ## 🚸 Recursive
    /// Borrows data from this node or any other subnode.
    /// ## 📌 Note
    /// * Use [`NodeDataTrait::obtain_data`] for direct retrieval on this node `(no recursion)`
    fn borrow_data(&self, path: impl Borrow<str>) -> Result<Option<&T>, NodeError>;
    /// ## 🚸 Recursive
    /// Borrows data from this node or any other subnode as mut.
    /// ## 📌 Note
    /// * Use [`NodeDataTrait::obtain_data_mut`] for direct retrieval on this node `(no recursion)`
    fn borrow_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, NodeError>;
}

/// Trait with all nodetree top-data management implementations.
/// Lunex SHOULD!!!! abstacts over this trait with another trait. (WIP)
pub trait NodeTopDataTrait<D> {
    /// Adds new top-level data and returns previous top-level data.
    fn add_topdata(&mut self, data: D) -> Option<D>;
    /// Removes top-level data and returns it.
    fn take_topdata(&mut self) -> Option<D>;
    /// Borrows top-level data.
    fn obtain_topdata(&self) -> Option<&D>;
    /// Borrows top-level data as mut.
    fn obtain_topdata_mut(&mut self) -> Option<&mut D>;
}

/// Trait with all init methods for empty nodes.
/// Lunex abstacts over this trait with another trait.
pub trait NodeInitTrait {
    /// Creates new [`Node`].
    fn new() -> Self;
}

/// Trait with init methods for [`NodeTree`].
/// Lunex abstacts over this trait with another trait.
pub trait NodeTreeInitTrait {
    /// Creates new [`NodeTree`].
    fn new(name: impl Borrow<str>) -> Self;
}

/// Trait with all node display implementations.
pub trait NodeDisplayTrait<T> {
    /// Generates overview of the inner structure of subnodes as a printable string.
    /// You can supply additional parameters like `show-hidden`.
    fn tree(&self, params: impl Borrow<str>) -> String;
}