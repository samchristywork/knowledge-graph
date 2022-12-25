pub mod graph;
pub mod image;
pub mod markdown;
pub mod node;
pub mod relation;
pub mod serial;
pub mod ui;

use std::path::Path;

fn main() {
    let path = Path::new("in");

    let g = graph::Graph::new_from_file(path);

    g.add_relation(
        g.new_node("main"),
        g.new_node("sub"),
        relation::Relation::new("child"),
    );

    let path = Path::new("out");
    g.to_file(path);
}
