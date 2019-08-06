#![allow(dead_code)]
use oscoin_graph_api as oscoin;

use std::collections::BTreeMap;

type Id = u64;

/// Byte string.
type Bytes = Vec<u8>;

type NodeData = Vec<(&'static str, Bytes)>;

type EdgeData = Bytes;

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
    id: Id,
    data: NodeData,
}

impl oscoin::Node<NodeData> for Node {}

#[derive(PartialEq, Debug, Clone)]
pub struct Edge {
    id: Id,
    from: Id,
    to: Id,
    data: EdgeData,
    weight: f64,
}

impl oscoin::Edge<f64, EdgeData> for Edge {
    fn weight(&self) -> f64 {
        self.weight
    }
}

impl oscoin::GraphObject for Edge {
    type Id = Id;
    type Data = EdgeData;

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
    type Data = NodeData;

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

impl Default for Network {
    fn default() -> Self {
        Self {
            edges: BTreeMap::new(),
            nodes: BTreeMap::new(),
        }
    }
}

impl oscoin::Graph<NodeData, EdgeData> for Network {
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

impl oscoin::GraphWriter<NodeData, EdgeData> for Network {
    fn add_node(&mut self, id: oscoin::Id<Node>, data: NodeData) {
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
        data: EdgeData,
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

impl oscoin::GraphDataWriter<NodeData, EdgeData> for Network {
    fn edge_data_mut(
        &mut self,
        id: <Self::Edge as oscoin::GraphObject>::Id,
    ) -> Option<&mut EdgeData> {
        self.edges.get_mut(&id).map(|e| &mut e.data)
    }

    fn node_data_mut(
        &mut self,
        id: <Self::Node as oscoin::GraphObject>::Id,
    ) -> Option<&mut NodeData> {
        self.nodes.get_mut(&id).map(|n| &mut n.data)
    }
}

fn main() {
    use oscoin::{Graph, GraphDataWriter, GraphWriter};

    let mut g = Network::default();
    g.add_node(0x1, vec![("A", Vec::new())]);
    g.add_node(0x2, vec![("B", Vec::new())]);
    g.add_edge(0x3, 0x1, 0x2, 1.0, vec![]);

    assert_eq!(
        g.neighbors(0x1).collect::<Vec<&Node>>(),
        vec![g.get_node(0x2).unwrap()]
    );

    *g.node_data_mut(0x1).unwrap() = vec![("AA", Vec::new())];
    assert_eq!(g.get_node(0x1).unwrap().data, vec![("AA", Vec::new())]);
}

mod ledger {
    use oscoin_graph_api as oscoin;
    use oscoin_graph_api::GraphWriter;

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    type H256 = [u8; 32];

    struct Dep {
        node_id: super::Id,
        is_added: bool,
    }

    struct Contrib {
        node_id: super::Id,
    }

    enum EdgeType {
        Dependency = 0x01,
        ContributionFrom = 0x02,
        ContributionTo = 0x03,
    }

    impl EdgeType {
        pub fn weight(&self) -> f64 {
            match self {
                EdgeType::Dependency => 0.7,
                EdgeType::ContributionFrom => 0.3,
                EdgeType::ContributionTo => 0.3,
            }
        }
    }

    // An example ledger implementation that uses the APIs defined.
    struct Ledger<T> {
        api: T,
    }

    impl<T> Ledger<T>
    where
        T: oscoin::GraphAPI<super::NodeData, super::EdgeData, Graph = super::Network>,
    {
        fn checkpoint(
            &mut self,
            id: super::Id,
            version: &[u8],
            hash: H256,
            deps: &[Dep],
            contribs: &[Contrib],
        ) {
            // The node-id must be unique for all (project-id, project-hash) pairs.
            // TODO
            let node_id = id;

            // Get a mutable ref to the osrank graph.
            let graph = self.api.graph_mut(&oscoin::Layer("osrank")).unwrap();

            // Add the new checkpoint node to the graph. We annotate it with the project
            // version and project hash.
            graph.add_node(
                node_id,
                vec![("version", version.to_vec()), ("hash", hash.to_vec())],
            );

            for d in deps.iter() {
                let edge_id = self::edge_id(node_id, d.node_id);

                // If we're adding a dependency, add a `project -> project` link.
                // If we're removing one, remove the link.
                if d.is_added {
                    graph.add_edge(
                        edge_id,
                        node_id,
                        d.node_id,
                        1.0,
                        vec![EdgeType::Dependency as u8],
                    );
                } else {
                    graph.remove_edge(edge_id);
                }
            }

            for c in contribs.iter() {
                // Add `project -> contribution` link.
                graph.add_edge(
                    self::edge_id(node_id, c.node_id),
                    node_id,
                    c.node_id,
                    EdgeType::ContributionFrom.weight(),
                    vec![EdgeType::ContributionFrom as u8],
                );
                // Add `contribution -> project` link.
                graph.add_edge(
                    self::edge_id(node_id, c.node_id),
                    c.node_id,
                    node_id,
                    EdgeType::ContributionTo.weight(),
                    vec![EdgeType::ContributionTo as u8],
                );
            }
        }
    }

    fn edge_id(from: super::Id, to: super::Id) -> super::Id {
        let mut hasher = DefaultHasher::new();
        from.hash(&mut hasher);
        to.hash(&mut hasher);
        hasher.finish()
    }
}
