pub mod model;

use model::{Graph, Node, Target, Relationship};

pub enum ExampleNodeName {
    A,
    B,
    C,
    D,
}

fn ExampleGraph() -> Graph<ExampleNodeName> {
    Graph { nodes: vec![
        Node{ typ: ExampleNodeName::A, targets: vec![
            Target{ typ: ExampleNodeName::B, rel: Relationship::OneToMany }
        ]},
        Node{ typ: ExampleNodeName::C, targets: vec![
            Target{ typ: ExampleNodeName::B, rel: Relationship::OneToOne }
        ]},
    ] }
}