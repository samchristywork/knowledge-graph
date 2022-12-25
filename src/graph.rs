use crate::node;
use crate::relation;
use std::path::Path;

pub struct Graph<'a> {
    name: &'a str,
    nodes: Vec<node::Node<'a>>,
    relations: Vec<relation::Relation<'a>>,
}
