#[deny(clippy::all)]
///! Graph API Traits
///
use rand::SeedableRng;

#[allow(dead_code)]
/// A graph layer name.
struct Layer(&'static str);

/// Abstract object in a graph, eg. node or edge.
trait GraphObject {
    /// Identifier of all graph objects.
    type Id;

    /// Local graph object data.
    type Data;

    /// Return the edge id.
    fn id(&self) -> Self::Id;

    /// Get local data.
    fn data(&self) -> &Self::Data;

    /// Get mutable local data.
    fn data_mut(&self) -> &mut Self::Data;
}

/// A graph  node.
trait Node: GraphObject {
    /// The edge type connected to this node.
    type Edge: GraphObject;

    /// Get connected nodes.
    fn neighbors(&self) -> Iterator<Item = &Self>;

    /// Get incoming and outgoing edges.
    fn edges(&self) -> Iterator<Item = &Self::Edge>;
}

/// A graph edge between two nodes.
trait Edge: GraphObject {
    /// Get the edge weight.
    fn weight(&self) -> f64;

    /// The object from which the edge starts.
    fn from(&self) -> <Self as GraphObject>::Id;

    /// The object to which the edge points.
    fn to(&self) -> <Self as GraphObject>::Id;
}

/// The Graph API
trait GraphAPI<G: Graph> {
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

/// A graph of nodes and edges.
trait Graph {
    /// A graph node.
    type Node: Node<Edge = Self::Edge>;

    /// A graph edge between nodes.
    type Edge: Edge;

    /// Add a node to the graph on the specified layer.
    fn add_node<N: Node>(&mut self, id: N::Id, data: N::Data);

    /// Remove a node from the graph.
    fn remove_node<N: Node>(&mut self, id: N::Id);

    /// Get a node.
    fn get_node<N: Node>(&self, id: N::Id) -> Option<&Self::Node>;

    /// Link two nodes.
    fn add_edge(&mut self, edge: Self::Edge, data: <Self::Edge as GraphObject>::Data);

    /// Unlink two nodes.
    fn remove_edge<E: Edge>(&mut self, id: E::Id);

    /// Iterator over nodes.
    fn iter(&self) -> Iterator<Item = &Self::Node>;

    /// Mutable iterator over nodes.
    fn iter_mut(&mut self) -> Iterator<Item = &mut Self::Node>;

    /// Annotate a node with data.
    fn annotate_node<N: Node>(&mut self, node: N::Id, data: &N::Data);

    /// Annotate an edge with data.
    fn annotate_edge<E: Edge>(&mut self, edge: E::Id, data: &E::Data);
}

/// A graph algorithm over a graph.
trait GraphAlgorithm<G: Graph> {
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
