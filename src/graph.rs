use crate::node;
use crate::relation;
use std::path::Path;

pub struct Graph<'a> {
    name: &'a str,
    nodes: Vec<node::Node<'a>>,
    relations: Vec<relation::Relation<'a>>,
}

impl<'a> Graph<'a> {
    pub fn new_from_file(path: &Path) -> Self {
        Self {
            name: "Unnamed",
            nodes: Vec::new(),
            relations: Vec::new(),
        }
    }

    pub fn to_file(&self, path: &Path) {}

    pub fn add_relation(&self, a: node::Node, b: node::Node, relation: relation::Relation) {}

    pub fn new_node(&self, name: &'a str) -> node::Node {
        node::Node::new(name)
    }
}
