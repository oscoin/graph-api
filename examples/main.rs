#![allow(dead_code)]
use oscoin_graph_api as oscoin;
use oscoin_graph_api::types;

use std::collections::BTreeMap;

/// Id shared by all objects.
type Id = u64;

/// Byte string.
type Bytes = Vec<u8>;

/// Data stored in nodes.
type NodeData = Vec<(&'static str, Bytes)>;

/// Data stored in edges.
type EdgeData = Bytes;

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
    id: Id,
    data: NodeData,
    node_type: types::NodeType,
}

impl oscoin::Node<NodeData> for Node {
    fn node_type(&self) -> &types::NodeType {
        &self.node_type
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Edge {
    id: Id,
    from: Id,
    to: Id,
    data: EdgeData,
    weight: f64,
    edge_type: types::EdgeType,
}

impl oscoin::Edge<f64, Id, EdgeData> for Edge {
    fn weight(&self) -> f64 {
        self.weight
    }

    fn source(&self) -> &Id {
        &self.from
    }

    fn target(&self) -> &Id {
        &self.to
    }

    fn edge_type(&self) -> &types::EdgeType {
        &self.edge_type
    }
}

impl oscoin::GraphObject for Edge {
    type Id = Id;
    type Data = EdgeData;

    fn id(&self) -> &Id {
        &self.id
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

    fn id(&self) -> &Id {
        &self.id
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

impl oscoin::Graph for Network {
    type Node = self::Node;
    type Edge = self::Edge;

    type NodeData = self::NodeData;
    type EdgeData = self::EdgeData;

    type Weight = f64;

    fn get_node(&self, id: &oscoin::Id<Node>) -> Option<&Self::Node> {
        self.nodes.get(id)
    }

    fn get_edge(&self, id: &<Self::Edge as oscoin::GraphObject>::Id) -> Option<&Self::Edge> {
        self.edges.get(id)
    }

    fn nodes(&self) -> oscoin::Nodes<Self::Node> {
        let vec: Vec<&Node> = self.nodes.values().collect();
        oscoin::Nodes {
            range: vec.into_iter(),
        }
    }

    fn neighbors(
        &self,
        node: &<Self::Node as oscoin::GraphObject>::Id,
    ) -> oscoin::Nodes<Self::Node> {
        let mut ns: Vec<&Node> = Vec::new();

        for e in self.edges.values() {
            if e.from == *node {
                ns.push(self.nodes.get(&e.to).unwrap());
            } else if e.to == *node {
                ns.push(self.nodes.get(&e.from).unwrap());
            }
        }

        oscoin::Nodes {
            range: ns.into_iter(),
        }
    }

    fn edges(&self, node: &<Self::Node as oscoin::GraphObject>::Id) -> oscoin::Edges<Self::Edge> {
        let mut edges = Vec::new();

        for e in self.edges.values() {
            if e.from == *node || e.to == *node {
                edges.push(e);
            }
        }
        oscoin::Edges {
            range: edges.into_iter(),
        }
    }

    fn edges_directed(
        &self,
        node: &<Self::Node as oscoin::GraphObject>::Id,
        dir: oscoin::Direction,
    ) -> oscoin::EdgeRefs<oscoin::Id<Self::Node>, oscoin::Id<Self::Edge>> {
        let mut refs = Vec::new();

        for e in self.edges.values() {
            if dir == oscoin::Direction::Outgoing && e.from == *node {
                refs.push(oscoin::EdgeRef {
                    from: &e.from,
                    to: &e.to,
                    id: &e.id,
                    edge_type: &e.edge_type,
                })
            } else if dir == oscoin::Direction::Incoming && e.to == *node {
                refs.push(oscoin::EdgeRef {
                    from: &e.from,
                    to: &e.to,
                    id: &e.id,
                    edge_type: &e.edge_type,
                })
            }
        }

        refs
    }
}

impl oscoin::GraphWriter for Network {
    fn add_node(&mut self, id: oscoin::Id<Node>, data: NodeData) {
        // For the sake of the example it doesn't matter which node type we
        // pick.
        self.nodes.insert(
            id,
            Node {
                id,
                data,
                node_type: types::NodeType::Project,
            },
        );
    }

    fn remove_node(&mut self, id: oscoin::Id<Node>) {
        self.nodes.remove(&id);
    }

    fn add_edge(
        &mut self,
        id: oscoin::Id<Edge>,
        from: &oscoin::Id<Node>,
        to: &oscoin::Id<Node>,
        data: EdgeData,
    ) {
        // In this example we are modelling the `EdgeData` as a bytes blob,
        // but in practice we should be able to extract a weight out of that.
        self.edges.insert(
            id,
            Edge {
                id,
                from: *from,
                to: *to,
                weight: 0.0,
                data,
                edge_type: types::EdgeType::ContributionFromUser,
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
        id: &<Self::Edge as oscoin::GraphObject>::Id,
    ) -> Option<&mut EdgeData> {
        self.edges.get_mut(id).map(|e| &mut e.data)
    }

    fn node_data_mut(
        &mut self,
        id: &<Self::Node as oscoin::GraphObject>::Id,
    ) -> Option<&mut NodeData> {
        self.nodes.get_mut(id).map(|n| &mut n.data)
    }
}

fn main() {
    use oscoin::{Graph, GraphDataWriter, GraphWriter};

    let mut g = Network::default();
    g.add_node(0x1, vec![("A", Vec::new())]);
    g.add_node(0x2, vec![("B", Vec::new())]);
    g.add_edge(0x3, &0x1, &0x2, vec![]);

    assert_eq!(
        g.neighbors(&0x1).collect::<Vec<&Node>>(),
        vec![g.get_node(&0x2).unwrap()]
    );

    assert_eq!(
        g.edges_directed(&0x1, oscoin::Direction::Outgoing)
            .into_iter()
            .map(|eref| g.get_edge(eref.id))
            .collect::<Vec<Option<&Edge>>>(),
        vec![g.get_edge(&0x3)]
    );

    *g.node_data_mut(&0x1).unwrap() = vec![("AA", Vec::new())];
    assert_eq!(g.get_node(&0x1).unwrap().data, vec![("AA", Vec::new())]);
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
        T: oscoin::GraphAPI<Graph = super::Network>,
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
                        &node_id,
                        &d.node_id,
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
                    &node_id,
                    &c.node_id,
                    vec![EdgeType::ContributionFrom as u8],
                );
                // Add `contribution -> project` link.
                graph.add_edge(
                    self::edge_id(node_id, c.node_id),
                    &c.node_id,
                    &node_id,
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
