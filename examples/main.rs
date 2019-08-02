#![allow(dead_code)]
use oscoin_graph_api as oscoin;

use std::collections::BTreeMap;

type Id = char;

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
    id: Id,
    data: String,
}

impl oscoin::Node for Node {}

#[derive(PartialEq, Debug, Clone)]
pub struct Edge {
    id: Id,
    from: Id,
    to: Id,
    data: String,
    weight: f64,
}

impl oscoin::Edge<f64> for Edge {
    fn weight(&self) -> f64 {
        self.weight
    }
}

impl oscoin::GraphObject for Edge {
    type Id = Id;
    type Data = String;

    fn id(&self) -> Id {
        self.id
    }

    fn data(&self) -> &Self::Data {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Self::Data {
        &mut self.data
    }
}

impl oscoin::GraphObject for Node {
    type Id = Id;
    type Data = String;

    fn id(&self) -> Id {
        self.id
    }

    fn data(&self) -> &Self::Data {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Self::Data {
        &mut self.data
    }
}

struct Network {
    edges: BTreeMap<Id, Edge>,
    nodes: BTreeMap<Id, Node>,
}

impl Network {
    fn new() -> Self {
        Self {
            edges: BTreeMap::new(),
            nodes: BTreeMap::new(),
        }
    }
}

impl oscoin::Graph for Network {
    type Edge = Edge;
    type Node = Node;
    type Weight = f64;

    fn get_node(&self, id: oscoin::Id<Node>) -> Option<&Self::Node> {
        self.nodes.get(&id)
    }

    fn get_edge(&self, id: <Self::Edge as oscoin::GraphObject>::Id) -> Option<&Self::Edge> {
        self.edges.get(&id)
    }

    fn nodes(&self) -> oscoin::Nodes<Self::Node> {
        let vec: Vec<&Node> = self.nodes.values().collect();
        oscoin::Nodes {
            range: vec.into_iter(),
        }
    }

    fn neighbors(
        &self,
        node: <Self::Node as oscoin::GraphObject>::Id,
    ) -> oscoin::Nodes<Self::Node> {
        let mut ns: Vec<&Node> = Vec::new();

        for e in self.edges.values() {
            if e.from == node {
                ns.push(self.nodes.get(&e.to).unwrap());
            } else if e.to == node {
                ns.push(self.nodes.get(&e.from).unwrap());
            }
        }
        oscoin::Nodes {
            range: ns.into_iter(),
        }
    }

    fn edges(&self, node: <Self::Node as oscoin::GraphObject>::Id) -> oscoin::Edges<Self::Edge> {
        let mut edges = Vec::new();

        for e in self.edges.values() {
            if e.from == node || e.to == node {
                edges.push(e);
            }
        }
        oscoin::Edges {
            range: edges.into_iter(),
        }
    }
}

impl oscoin::GraphWriter for Network {
    fn add_node(&mut self, id: oscoin::Id<Node>, data: oscoin::Data<Node>) {
        self.nodes.insert(id, Node { id, data });
    }

    fn remove_node(&mut self, id: oscoin::Id<Node>) {
        self.nodes.remove(&id);
    }

    fn add_edge(
        &mut self,
        id: oscoin::Id<Edge>,
        from: oscoin::Id<Node>,
        to: oscoin::Id<Node>,
        weight: f64,
        data: oscoin::Data<Edge>,
    ) {
        self.edges.insert(
            id,
            Edge {
                id,
                from,
                to,
                weight,
                data,
            },
        );
    }

    fn remove_edge(&mut self, id: <Self::Edge as oscoin::GraphObject>::Id) {
        self.edges.remove(&id);
    }

    fn nodes_mut(&mut self) -> oscoin::NodesMut<Self::Node> {
        let vec: Vec<&mut Node> = self.nodes.values_mut().collect();
        oscoin::NodesMut {
            range: vec.into_iter(),
        }
    }
}

impl oscoin::GraphDataWriter for Network {
    fn edge_data_mut(
        &mut self,
        id: <Self::Edge as oscoin::GraphObject>::Id,
    ) -> Option<&mut <Self::Edge as oscoin::GraphObject>::Data> {
        self.edges.get_mut(&id).map(|e| &mut e.data)
    }

    fn node_data_mut(
        &mut self,
        id: <Self::Node as oscoin::GraphObject>::Id,
    ) -> Option<&mut <Self::Node as oscoin::GraphObject>::Data> {
        self.nodes.get_mut(&id).map(|n| &mut n.data)
    }
}

fn main() {
    use oscoin::{Graph, GraphDataWriter, GraphWriter};

    let mut g = Network::new();
    g.add_node('a', "A".to_owned());
    g.add_node('b', "B".to_owned());
    g.add_edge('e', 'a', 'b', 1.0, "Dependency".to_owned());

    assert_eq!(
        g.neighbors('a').collect::<Vec<&Node>>(),
        vec![g.get_node('b').unwrap()]
    );

    *g.node_data_mut('a').unwrap() = "AA".to_owned();
    assert_eq!(g.get_node('a').unwrap().data, "AA".to_owned());
}
