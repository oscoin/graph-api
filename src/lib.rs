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
pub type Metadata<T> = <T as GraphObject>::Data;

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
pub trait Edge: GraphObject {
    /// Get the edge weight.
    fn weight(&self) -> f64;
}

/// The Graph API
pub trait GraphAPI<G: GraphWriter + GraphAnnotator> {
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

pub trait GraphWriter: Graph {
    /// Add a node to the graph on the specified layer.
    fn add_node(
        &mut self,
        id: <Self::Node as GraphObject>::Id,
        data: <Self::Node as GraphObject>::Data,
    );

    /// Remove a node from the graph.
    fn remove_node(&mut self, id: <Self::Node as GraphObject>::Id);

    /// Link two nodes.
    fn add_edge(&mut self, edge: Self::Edge, data: <Self::Edge as GraphObject>::Data);

    /// Unlink two nodes.
    fn remove_edge(&mut self, id: <Self::Edge as GraphObject>::Id);

    /// Mutable iterator over nodes.
    fn nodes_mut(&mut self) -> NodesMut<Self::Node>;
}

pub trait GraphAnnotator: Graph {
    /// Annotate a node with data.
    fn annotate_node(
        &mut self,
        node: <Self::Node as GraphObject>::Id,
        data: &<Self::Node as GraphObject>::Data,
    );

    /// Annotate an edge with data.
    fn annotate_edge(
        &mut self,
        edge: <Self::Edge as GraphObject>::Id,
        data: &<Self::Edge as GraphObject>::Data,
    );
}

/// A read-only graph of nodes and edges.
pub trait Graph {
    /// A graph node.
    type Node: Node;

    /// A graph edge between nodes.
    type Edge: Edge;

    /// Get a node.
    fn get_node(&self, id: <Self::Node as GraphObject>::Id) -> Option<&Self::Node>;

    /// Get an edge.
    fn get_edge(&self, id: <Self::Edge as GraphObject>::Id) -> Option<&Self::Edge>;

    /// Iterator over nodes.
    fn nodes(&self) -> Nodes<Self::Node>;

    /// Get a node's neighbors.
    fn neighbors(&self, node: <Self::Node as GraphObject>::Id) -> Nodes<Self::Node>;

    /// Get a node's inbound and outbound edges.
    fn edges(&self, node: <Self::Node as GraphObject>::Id) -> Vec<Self::Edge>;
}

/// A graph algorithm over a graph.
pub trait GraphAlgorithm<G: GraphAnnotator> {
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

pub struct Nodes<'a, N: 'a> {
    pub range: std::vec::IntoIter<&'a N>,
}

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
