#[deny(clippy::all)]
///! Graph API Traits
///
use rand::SeedableRng;

#[allow(dead_code)]
/// A graph layer name.
pub struct Layer(&'static str);

/// A handy type alias.
pub type Id<T> = <T as GraphObject>::Id;

/// A handy type alias.
pub type Data<T> = <T as GraphObject>::Data;

/// Abstract object in a graph, eg. node or edge.
pub trait GraphObject {
    /// Identifier of all graph objects.
    type Id: Clone;

    /// Local graph object data.
    type Data;

    /// Return the edge id.
    fn id(&self) -> Self::Id;

    /// Get local data.
    fn data(&self) -> &Self::Data;

    /// Get mutable local data.
    fn data_mut(&mut self) -> &mut Self::Data;
}

/// A graph  node.
pub trait Node: GraphObject {}

/// A graph edge between two nodes.
pub trait Edge<W>: GraphObject {
    /// Get the edge weight.
    fn weight(&self) -> W;
}

/// The Graph API
pub trait GraphAPI<G: GraphWriter> {
    /// Entropy used to seed the CPRNG.
    type Entropy;

    /// Add a graph layer.
    fn add_layer(&mut self, layer: Layer);

    /// Remove a graph layer. The layer must not contain any nodes.
    fn remove_layer(&mut self, layer: &Layer);

    /// Return an immutable graph of the given layer.
    fn graph(&self, layer: &Layer) -> Option<&G>;

    /// Return the mutable graph of the given layer.
    fn graph_mut(&mut self, layer: &Layer) -> Option<&mut G>;

    /// Given initial entropy, returns a random seed.
    fn seed<R: SeedableRng>(&mut self, entropy: Self::Entropy) -> R::Seed;
}

pub trait GraphWriter: Graph + GraphDataWriter {
    /// Add a node to the graph on the specified layer.
    fn add_node(&mut self, id: Id<Self::Node>, data: Data<Self::Node>);

    /// Remove a node from the graph.
    fn remove_node(&mut self, id: Id<Self::Node>);

    /// Link two nodes.
    fn add_edge(
        &mut self,
        id: Id<Self::Edge>,
        from: Id<Self::Node>,
        to: Id<Self::Node>,
        weight: f64,
        data: Data<Self::Edge>,
    );

    /// Unlink two nodes.
    fn remove_edge(&mut self, id: Id<Self::Edge>);

    /// Mutable iterator over nodes.
    fn nodes_mut(&mut self) -> NodesMut<Self::Node>;
}

pub trait GraphDataWriter: Graph {
    /// Return a mutable reference to an edge's data, to annotate the edge.
    fn edge_data_mut(&mut self, id: Id<Self::Edge>) -> Option<&mut Data<Self::Edge>>;

    /// Return a mutable reference to a node's data, to annotate the node.
    fn node_data_mut(&mut self, id: Id<Self::Node>) -> Option<&mut Data<Self::Node>>;
}

/// A read-only graph of nodes and edges.
pub trait Graph {
    /// A graph node.
    type Node: Node;

    /// A graph edge between nodes.
    type Edge: Edge<Self::Weight>;

    /// An edge weight.
    type Weight;

    /// Get a node.
    fn get_node(&self, id: Id<Self::Node>) -> Option<&Self::Node>;

    /// Get an edge.
    fn get_edge(&self, id: Id<Self::Edge>) -> Option<&Self::Edge>;

    /// Iterator over nodes.
    fn nodes(&self) -> Nodes<Self::Node>;

    /// Get a node's neighbors.
    fn neighbors(&self, node: Id<Self::Node>) -> Nodes<Self::Node>;

    /// Get a node's inbound and outbound edges.
    fn edges(&self, node: Id<Self::Node>) -> Edges<Self::Edge>;
}

/// A graph algorithm over a graph.
pub trait GraphAlgorithm<G: GraphDataWriter> {
    /// Some input state to the execution.
    type Input;

    /// The output of the execution.
    type Output;

    /// An execution error.
    type Error;

    /// Execute an algorithm over an immutable input and mutable graph.
    fn execute<R: SeedableRng>(
        &self,
        input: &Self::Input,
        graph: &mut G,
        rng: R,
    ) -> Result<Self::Output, Self::Error>;
}

/// Iterator over edges.
pub struct Edges<'a, E: 'a> {
    pub range: std::vec::IntoIter<&'a E>,
}

impl<'a, N: 'a> Iterator for Edges<'a, N> {
    type Item = &'a N;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next()
    }
}

/// Iterator over nodes.
pub struct Nodes<'a, N: 'a> {
    pub range: std::vec::IntoIter<&'a N>,
}

/// Iterator over mutable nodes.
pub struct NodesMut<'a, N: 'a> {
    pub range: std::vec::IntoIter<&'a mut N>,
}

impl<'a, N: 'a> Iterator for Nodes<'a, N> {
    type Item = &'a N;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next()
    }
}

impl<'a, N: 'a> Iterator for NodesMut<'a, N> {
    type Item = &'a mut N;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next()
    }
}
