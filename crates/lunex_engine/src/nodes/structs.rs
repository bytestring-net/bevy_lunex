use bevy::ecs::component::Component;
use crate::import::*;
use crate::NiceDisplay;
use super::{NodeGeneralTrait, NodeCreationTrait, NodeDataTrait, NodeTopDataTrait, NodeInitTrait, NodeTreeInitTrait, NodeDisplayTrait};


// #==================#
// #=== ERROR TYPE ===#

/// ## Node error
/// Error type indicating something went wrong.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum NodeError {

    /// Error that happens when merging nodes. Two subnodes share the same name thus cannot be merged.
    #[error("Duplicate name conflict for '{0:}' when trying to merge nodes")]
    DuplicateName (String),

    /// Error that happens when attempting to create a node with a name that is already in use.
    #[error("Name '{0:}' is already in use")]
    NameInUse (String),

    /// Error that happens when the path you provided is not allowed.
    #[error("Path '{0:}' is not allowed")]
    InvalidPath (String),

    /// Error that happens when you try to locate a node that doesn't exist.
    #[error("Unable to locate '{0:}' node")]
    NoNode (String),
}


// #================#
// #=== NODETREE ===#

/// A hashmap-like data structure for organizing general data into recursive subnodes.
/// Data is indexed and traversed using `paths`. It retains the order of insertion.
/// ## 📏 Structure
/// All data is stored inside inner tree-like hierarchy. Each node can store users data and multiple subnodes.
/// ```text
/// > NodeTree
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
/// If you want to access `Node_4`, you use path `"Node_1/Node_3/Node_4"` on `NodeTree` struct.
/// You can also use `"Node_3/Node_4"` on `Node_1` struct to get the same result.
/// 
/// Whitespaces are allowed in paths, but are not encouraged.
/// Putting a dot as first symbol like this `".name"` will hide the node from the tree.
/// Just `"."` will refer to the same node. `".."` is not supported and is actually a valid name.
/// 
/// You can also not specify the name when creating a node. That means the name will be generated.
/// The format is as follows `".||#:N"` with `N` being the `.len()` of the `nodes` hashmap.
/// ## 📦 Types
/// * Generic `(D)` - Master data schema struct defining what surface data can be stored in [`NodeTree`] for all nodes to share.
/// * Generic `(N)` - Node data schema struct defining what node-specific data can be stored in [`Node`]
/// ## ⚠️ Warning
/// Please refrain from manually using `".||#:0"`, `".||#:1"`, `".||#:2"`, ... as names or [`NodeGeneralTrait::add_node`] will return errors.
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct NodeTree<D, T> {
    /// ## Top-level data
    /// This top-level data is meant to be shared for every node. Example usage is storing `theme` and other surface data.
    pub data: Option<D>,

    /// ## Node
    /// The starting root node.
    pub node: Node<T>,
}
impl <D, T> NodeTopDataTrait<D> for NodeTree<D, T> {
    fn add_topdata(&mut self, data: D) -> Option<D> {
        core::mem::replace(&mut self.data, Some(data))
    }

    fn take_topdata(&mut self) -> Option<D> {
        core::mem::replace(&mut self.data, None)
    }

    fn obtain_topdata(&self) -> Option<&D> {
        match &self.data {
            Some(value) => Some(value),
            None => None,
        }
    }

    fn obtain_topdata_mut(&mut self) -> Option<&mut D> {
        match &mut self.data {
            Some(value) => Some(value),
            None => None,
        }
    }
}
impl <D, T> NodeTreeInitTrait for NodeTree<D, T> {
    fn new(name: impl Borrow<str>) -> Self {
        let mut node = Node::new();
        node.name = name.borrow().into();
        node.path = "".into();
        NodeTree { data: None, node }
    }
}
impl <D, T> NodeGeneralTrait<T> for NodeTree<D, T> {
    fn add_node(&mut self, name: impl Borrow<str>, node: impl Into<Node<T>>) -> Result<String, NodeError>{
        self.node.add_node(name, node)
    }

    fn insert_node(&mut self, path: impl Borrow<str>, node: impl Into<Node<T>>) -> Result<String, NodeError>{
        self.node.insert_node(path, node)
    }

    fn take_node(&mut self, name: impl Borrow<str>) -> Result<Node<T>, NodeError> {
        self.node.take_node(name)
    }

    fn remove_node(&mut self, path: impl Borrow<str>) -> Result<Node<T>, NodeError> {
        self.node.remove_node(path)
    }

    fn obtain_node(&self, name: impl Borrow<str>) -> Result<&Node<T>, NodeError> {
        self.node.obtain_node(name)
    }

    fn obtain_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Node<T>, NodeError> {
        self.node.obtain_node_mut(name)
    }

    fn borrow_node(&self, path: impl Borrow<str>) -> Result<&Node<T>, NodeError> {
        self.node.borrow_node(path)
    }

    fn borrow_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Node<T>, NodeError> {
        self.node.borrow_node_mut(path)
    }

    fn merge(&mut self, node: impl Into<Node<T>>) -> Result<(), NodeError> {
        self.node.merge(node.into())
    }

    fn crawl(&self) -> Vec<&Node<T>> {
        self.node.crawl()
    }

    fn tree_node(&self, params: impl Borrow<str>) -> String {
        self.node.tree_node(params)
    }

    fn get_name(&self) -> &String {
        &self.node.get_name()
    }

    fn get_path(&self) -> &String {
        &self.node.get_path()
    }

    fn get_depth(&self) -> f32 {
        self.node.get_depth()
    }
}
impl <D, T> NodeCreationTrait<T> for NodeTree<D, T> {
    fn make_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError>{
        self.node.make_node(name)
    }

    fn create_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError>{
        self.node.create_node(path)
    }

    fn obtain_or_create_node(&mut self, name: impl Borrow<str>) -> Result<&Node<T>, NodeError> {
        self.node.obtain_or_create_node(name)
    }

    fn obtain_or_create_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Node<T>, NodeError> {
        self.node.obtain_or_create_node_mut(name)
    }

    fn borrow_or_create_node(&mut self, path: impl Borrow<str>) -> Result<&Node<T>, NodeError> {
        self.node.borrow_or_create_node(path)
    }

    fn borrow_or_create_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Node<T>, NodeError> {
        self.node.borrow_or_create_node_mut(path)
    }
}
impl <D, T> NodeDataTrait<T> for NodeTree<D, T> {
    fn add_data(&mut self, data: T) -> Option<T> {
        self.node.add_data(data)
    }

    fn insert_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeError> {
        self.node.insert_data(path, data)
    }

    fn take_data(&mut self) -> Option<T> {
        self.node.take_data()
    }

    fn remove_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeError> {
        self.node.remove_data(path)
    }

    fn obtain_data(&self) -> Option<&T> {
        self.node.obtain_data()
    }
    
    fn obtain_data_mut(&mut self) -> Option<&mut T> {
        self.node.obtain_data_mut()
    }

    fn borrow_data(&self, path: impl Borrow<str>) -> Result<Option<&T>, NodeError> {
        self.node.borrow_data(path)
    }
    
    fn borrow_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, NodeError> {
        self.node.borrow_data_mut(path)
    }
}
impl <D, T: NiceDisplay> NodeDisplayTrait<T> for NodeTree<D, T> {
    fn tree(&self, params: impl Borrow<str>) -> String {
        self.node.tree(params)
    }
}
impl <D, T> Into<Node<T>> for NodeTree<D, T>{
    fn into(self) -> Node<T> {
        self.node
    }
}


// #============#
// #=== NODE ===#

/// A struct representing organized data in [`NodeTree`].
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct Node<T> {
    /// ## Name
    /// Name of the node. `Cached` & `Read-only`.
    name: String,
    /// ## Path
    /// Full path without the name. `Cached` & `Read-only`.
    path: String,
    /// ## Depth
    /// Depth within the hierarchy. `Cached` & `Read-only`.
    depth: f32,

    /// ## Data
    /// Optional data this node can have. Example usage is storing `node layout` and other specific data.
    pub data: Option<T>,
    /// ## Nodes
    /// All subnodes this node contains. Treat is as `Read-only` unless you know what you are doing.
    /// Use the struct methods to manipulate the values inside.
    pub nodes: HashMap<String, Node<T>>,
}
impl <T> Node<T> {
    /// Generate overview of the inner tree and write the mapped output to the given string with data formatted to a certain level depth
    pub(crate) fn cascade_tree(&self, mut string: String, level: u32, param: &str) -> String {
        for (name, node) in &self.nodes {
            if !param.contains("show-hidden") && name.starts_with('.') {continue;}
            let mut text = String::from("\n  ");
            for _ in 0..level { text += "|    " }
            text += "|-> ";
            string = format!("{}{}{}", string, text.black(), name.bold().yellow());
            string = node.cascade_tree(string, level + 1, param);
        }
        string
    }
}
impl <T:NiceDisplay> Node<T> {
    /// Generate overview of the inner tree and write the mapped output to the given string with data formatted to a certain level depth
    pub(crate) fn cascade_tree_display(&self, mut string: String, level: u32, param: &str) -> String {
        if !param.contains("no-data") {
            if let Some(data) = &self.data {
                let text = String::from(" == ");
                string = format!("{}{}{}", string, text.black(), data.to_nicestr());
            }
        }
        for (name, node) in &self.nodes {
            if !param.contains("show-hidden") && name.starts_with('.') {continue;}
            let mut text = String::from("\n  ");
            for _ in 0..level { text += "|    " }
            text += "|-> ";
            string = format!("{}{}{}", string, text.black(), name.bold().yellow());
            string = node.cascade_tree_display(string, level + 1, param);
        }
        string
    }
}
impl <T> NodeInitTrait for Node<T> {
    fn new() -> Self {
        Node {
            name: "".into(),
            path: "".into(),
            depth: 0.0,

            data: None,
            nodes: HashMap::new(),
        }
    }
}
impl <T> NodeGeneralTrait<T> for Node<T> {
    fn add_node(&mut self, name: impl Borrow<str>, node: impl Into<Node<T>>) -> Result<String, NodeError>{
        let mut node = node.into();
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Err(NodeError::NameInUse("The special symbol '.' is used to refer to 'self' and is not available for use".to_owned())) }
            if self.nodes.contains_key(name.borrow()) == false {
                node.name = name.borrow().to_owned();
                node.path = if self.path.is_empty() { name.borrow().to_owned() } else { self.path.to_owned() + "/" + name.borrow() };
                node.depth = self.depth + 1.0;
                self.nodes.insert(name.borrow().to_owned(), node);
                Ok(name.borrow().to_owned())
            } else {
                Err(NodeError::NameInUse(name.borrow().to_owned()))
            }
        } else {
            let mut generated_name = format!(".||#:{}", self.nodes.len());
            let mut i = 0;
            while self.nodes.contains_key(&generated_name) == true {
                generated_name = format!(".||#:{}", self.nodes.len()+i);
                i += 1;
                if i > 100 { return Err(NodeError::InvalidPath("Failed to generate name, max threshold reached!".to_owned())); }
            }
            node.name = generated_name.to_owned();
            node.path = if self.path.is_empty() { generated_name.to_owned() } else { self.path.to_owned() + "/" + &generated_name };
            node.depth = self.depth + 1.0;
            self.nodes.insert(generated_name.to_owned(), node);
            Ok(generated_name)
        }
    }

    fn insert_node(&mut self, path: impl Borrow<str>, node: impl Into<Node<T>>) -> Result<String, NodeError>{
        match path.borrow().rsplit_once('/'){
            None => self.add_node(path, node),
            Some((rempath, name)) => self.borrow_node_mut(rempath)?.add_node(name, node),
        }
    }

    fn take_node(&mut self, name: impl Borrow<str>) -> Result<Node<T>, NodeError> {
        match self.nodes.shift_remove(name.borrow()) {
            Some(node) => Ok(node),
            None => Err(NodeError::NoNode(name.borrow().to_owned())),
        }
    }

    fn remove_node(&mut self, path: impl Borrow<str>) -> Result<Node<T>, NodeError> {
        match path.borrow().rsplit_once('/') {
            None => self.take_node(path),
            Some((rempath, name)) => self.borrow_node_mut(rempath)?.remove_node(name),
        }
    }

    fn obtain_node(&self, name: impl Borrow<str>) -> Result<&Node<T>, NodeError> {
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Ok(self) }
            match self.nodes.get(name.borrow()) {
                Some(node) => Ok(node),
                None => Err(NodeError::NoNode(name.borrow().into())),
            }
        } else {
            Err(NodeError::InvalidPath(name.borrow().into()))
        }
    }

    fn obtain_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Node<T>, NodeError> {
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Ok(self) }
            match self.nodes.get_mut(name.borrow()) {
                Some(node) => Ok(node),
                None => Err(NodeError::NoNode(name.borrow().into())),
            }
        } else {
            Err(NodeError::InvalidPath(name.borrow().into()))
        }
    }

    fn borrow_node(&self, path: impl Borrow<str>) -> Result<&Node<T>, NodeError> {
        match path.borrow().split_once('/') {
            None => self.obtain_node(path),
            Some((name, rempath)) => self.obtain_node(name)?.borrow_node(rempath),
        }
    }

    fn borrow_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Node<T>, NodeError> {
        match path.borrow().split_once('/') {
            None => self.obtain_node_mut(path),
            Some((name, rempath)) => self.obtain_node_mut(name)?.borrow_node_mut(rempath),
        }
    }

    fn merge(&mut self, node: impl Into<Node<T>>) -> Result<(), NodeError> {
        let node = node.into();
        //if let Some(_) = node.data { return Err(NodeError::DataConflict); }
        for (name, _) in &node.nodes {
            if self.nodes.contains_key(name) { return Err(NodeError::DuplicateName(name.to_owned())); }
        }
        for (name, dir) in node.nodes {
            self.insert_node(name, dir)?;
        }
        Ok(())
    }

    fn crawl(&self) -> Vec<&Node<T>> {
        let mut vector = Vec::new();
        for (_, node) in &self.nodes{
            vector.push(node);
            let mut content = node.crawl();
            vector.append(&mut content);
        }
        vector
    }

    fn tree_node(&self, params: impl Borrow<str>) -> String {
        let text = String::new();
        format!(
            "{} {}{}",
            ">".black(),
            self.name.purple().bold().underline(),
            self.cascade_tree(text, 0, params.borrow())
        )
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_path(&self) -> &String {
        &self.path
    }

    fn get_depth(&self) -> f32 {
        self.depth
    }
}
impl <T> NodeCreationTrait<T> for Node<T> {
    fn make_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError> {
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Err(NodeError::NameInUse("The special symbol '.' is used to refer to 'self' and is not available for use".to_owned())) }
            if self.nodes.contains_key(name.borrow()) == false {
                let mut node = Node::new();
                node.name = name.borrow().to_owned();
                node.path = if self.path.is_empty() { name.borrow().to_owned() } else { self.path.to_owned() + "/" + name.borrow() };
                node.depth = self.depth + 1.0;
                self.nodes.insert(name.borrow().to_owned(), node);
                Ok(name.borrow().to_owned())
            } else {
                Err(NodeError::NameInUse(name.borrow().to_owned()))
            }
        } else {
            let mut generated_name = format!(".||#:{}", self.nodes.len());
            let mut i = 0;
            while self.nodes.contains_key(&generated_name) == true {
                generated_name = format!(".||#:{}", self.nodes.len()+i);
                i += 1;
                if i > 100 { return Err(NodeError::InvalidPath("Failed to generate name, max threshold reached!".to_owned())); }
            }
            let mut node = Node::new();
            node.name = generated_name.to_owned();
            node.path = if self.path.is_empty() { generated_name.to_owned() } else { self.path.to_owned() + "/" + &generated_name };
            node.depth = self.depth + 1.0;
            self.nodes.insert(generated_name.to_owned(), node);
            Ok(generated_name)
        }
    }

    fn create_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError> {
        self.insert_node(path, Node::new())
    }

    fn obtain_or_create_node(&mut self, name: impl Borrow<str>) -> Result<&Node<T>, NodeError> {
        let _ = self.make_node(name.borrow());
        self.obtain_node(name)
    }

    fn obtain_or_create_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Node<T>, NodeError> {
        let _ = self.make_node(name.borrow());
        self.obtain_node_mut(name)
    }

    fn borrow_or_create_node(&mut self, path: impl Borrow<str>) -> Result<&Node<T>, NodeError> {
        match path.borrow().split_once('/') {
            None => self.obtain_or_create_node(path),
            Some((name, rempath)) => self.obtain_or_create_node_mut(name)?.borrow_or_create_node(rempath),
        }
    }

    fn borrow_or_create_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Node<T>, NodeError> {
        match path.borrow().split_once('/') {
            None => self.obtain_or_create_node_mut(path),
            Some((name, rempath)) => self.obtain_or_create_node_mut(name)?.borrow_or_create_node_mut(rempath),
        }
    }
}
impl <T> NodeDataTrait<T> for Node<T> {
    fn add_data(&mut self, data: T) -> Option<T> {
        core::mem::replace(&mut self.data, Some(data))
    }

    fn insert_data(&mut self, path: impl Borrow<str>, data: T) -> Result<Option<T>, NodeError>{
        Ok(self.borrow_node_mut(path)?.add_data(data))
    }

    fn take_data(&mut self) -> Option<T> {
        core::mem::replace(&mut self.data, None)
    }

    fn remove_data(&mut self, path: impl Borrow<str>) -> Result<Option<T>, NodeError> {
        Ok(self.borrow_node_mut(path)?.take_data())
    }

    fn obtain_data(&self) -> Option<&T> {
        self.data.as_ref()
    }
    
    fn obtain_data_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }

    fn borrow_data(&self, path: impl Borrow<str>) -> Result<Option<&T> , NodeError> {
        Ok(self.borrow_node(path)?.obtain_data())
    }
    
    fn borrow_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T> , NodeError> {
        Ok(self.borrow_node_mut(path)?.obtain_data_mut())
    }
}
impl <T:NiceDisplay> NodeDisplayTrait<T> for Node<T> {
    fn tree(&self, params: impl Borrow<str>) -> String {
        let text = String::new();
        format!(
            "{} {}{}",
            ">".black(),
            self.name.purple().bold().underline(),
            self.cascade_tree_display(text, 0, params.borrow())
        )
    }
}
