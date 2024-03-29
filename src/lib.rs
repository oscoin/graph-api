#[deny(clippy::all)]
///! Graph API Traits
pub mod types;

use crate::types::EdgeType;

/// Specifies a direction for an edge.
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Outgoing,
    Incoming,
}

/// A graph layer name.
pub struct Layer(pub &'static str);

/// A handy type alias.
pub type Id<T> = <T as GraphObject>::Id;

/// A handy type alias.
pub type Data<T> = <T as GraphObject>::Data;

/// Abstract object in a graph, eg. node or edge.
pub trait GraphObject {
    /// Identifier of all graph objects.
    type Id;

    /// Local graph object data.
    type Data;

    /// Return the edge id.
    fn id(&self) -> &Self::Id;

    /// Get local data.
    fn data(&self) -> &Self::Data;

    /// Get mutable local data.
    fn data_mut(&mut self) -> &mut Self::Data;
}

/// A graph  node.
pub trait Node<N>: GraphObject<Data = N> {
    /// Returns the type of this node.
    // NOTE(adinapoli) Here we are using the *concrete* `NodeType`, but
    // at this stage it's unclear whether is useful/necessary to depend on the
    // concrete type. That might be useful if the Registry & Osrank will agree
    // on this same type and would require this to be the "lowest common
    // denominator" when writing otherwise-polymorphic and abstract GraphAPI
    // code. An alternative design could be remove this method and have
    // library writers to use the `Data` here and require their functions to
    // "known" how to convert from `Data` into a `NodeType`.
    fn node_type(&self) -> &types::NodeType;
}

/// A graph edge between two nodes.
pub trait Edge<W, NodeId, E>: GraphObject<Data = E> {
    /// The source node.
    fn source(&self) -> &NodeId;
    /// The target node
    fn target(&self) -> &NodeId;
    /// Get the edge weight.
    fn weight(&self) -> W;
    /// Returns the type of this edge.
    // TODO(adinapoli) Same considerations as per `Node::node_type` apply.
    fn edge_type(&self) -> &types::EdgeType;
}

/// The Graph API
pub trait GraphAPI {
    /// The underlying graph.
    type Graph: GraphWriter;

    /// Add a graph layer.
    fn add_layer(&mut self, layer: Layer);

    /// Remove a graph layer. The layer must not contain any nodes.
    fn remove_layer(&mut self, layer: &Layer);

    /// Return an immutable graph of the given layer.
    fn graph(&self, layer: &Layer) -> Option<&Self::Graph>;

    /// Return the mutable graph of the given layer.
    fn graph_mut(&mut self, layer: &Layer) -> Option<&mut Self::Graph>;
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
        from: &Id<Self::Node>,
        to: &Id<Self::Node>,
        data: Data<Self::Edge>,
    );

    /// Unlink two nodes.
    fn remove_edge(&mut self, id: Id<Self::Edge>);

    /// Mutable iterator over nodes.
    fn nodes_mut(&mut self) -> NodesMut<Self::Node>;
}

/// A graph with read-only access to edge and node data.
pub trait GraphDataReader: Graph {
    /// Return an immutable reference to an edge's data.
    fn edge_data(&self, id: &Id<Self::Edge>) -> Option<&Data<Self::Edge>>;

    /// Return an immutable reference to a node's data.
    fn node_data(&self, id: &Id<Self::Node>) -> Option<&Data<Self::Node>>;
}

/// A graph with mutable access to edge and node data.
pub trait GraphDataWriter: Graph {
    /// Return a mutable reference to an edge's data, to annotate the edge.
    fn edge_data_mut(&mut self, id: &Id<Self::Edge>) -> Option<&mut Data<Self::Edge>>;

    /// Return a mutable reference to a node's data, to annotate the node.
    fn node_data_mut(&mut self, id: &Id<Self::Node>) -> Option<&mut Data<Self::Node>>;
}

/// An annotator for graphs.
pub trait GraphAnnotator {
    type Annotation;

    /// Annotate the graph with some data. This is left intentionally abstract,
    /// for the implementation to decide whether to expose a key/value like
    /// interface with eg. `Annotation = (Key, Val)` or a batched interface
    /// like `Annotation = Vec<(Key, Val)>`.
    fn annotate_graph(&mut self, note: Self::Annotation);
}

/// A read-only graph of nodes and edges.
pub trait Graph: Default {
    /// A graph node.
    type Node: Node<Self::NodeData>;

    /// A graph edge between nodes.
    type Edge: Edge<Self::Weight, <Self::Node as GraphObject>::Id, Self::EdgeData>;

    /// Data stored in graph nodes.
    type NodeData;

    /// Data stored in graph edges.
    type EdgeData;

    /// An edge weight.
    type Weight;

    /// Get a node.
    fn get_node(&self, id: &Id<Self::Node>) -> Option<&Self::Node>;

    /// Get an edge.
    fn get_edge(&self, id: &Id<Self::Edge>) -> Option<&Self::Edge>;

    /// Iterator over nodes.
    fn nodes(&self) -> Nodes<Self::Node>;

    /// Get a node's neighbors.
    fn neighbors(&self, node: &Id<Self::Node>) -> Nodes<Self::Node>;

    /// Get a node's inbound and outbound edges.
    fn edges(&self, node: &Id<Self::Node>) -> Edges<Self::Edge>;

    /// Get a node's *directed* edges by passing a `Direction` as input.
    /// This is a slightly more specialised version of `edges`.
    fn edges_directed(
        &self,
        node: &Id<Self::Node>,
        dir: Direction,
    ) -> EdgeRefs<Id<Self::Node>, Id<Self::Edge>>;
}

/// A graph algorithm over a graph.
pub trait GraphAlgorithm<G, A>
where
    G: Graph,
    A: GraphAnnotator<Annotation = Self::Annotation>,
{
    /// Mutable context of the execution.
    /// Can be used as a stateful cache.
    /// The first time `execute` is called,
    /// `Context::default()` is passed in.
    type Context: Default;

    /// The output of the execution.
    type Output;

    /// An execution error.
    type Error;

    /// A seed suitable for an RNG.
    type RngSeed;

    /// The type of annotation the algorithm will make
    /// on the graph.
    type Annotation;

    /// Execute an algorithm over a context and graph.
    /// Changes to the context will be persisted across
    /// executions of the algorithm.
    fn execute(
        &self,
        context: &mut Self::Context,
        graph: &G,
        annotator: &mut A,
        seed: Self::RngSeed,
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

/// Iterator over edge _references_, which keep track of the source and
/// target.
#[derive(Debug)]
pub struct EdgeRef<'a, NodeId, EdgeId> {
    pub from: &'a NodeId,
    pub to: &'a NodeId,
    pub id: &'a EdgeId,

    // TODO(adinapoli) Here we are using the *concrete* `EdgeType`, because
    // it's handy to get hold of the type of the edge for library consumers,
    // without having to assume anything about the underlying structure of the
    // edge's data. In this design we cannot use `G::EdgeData` directly, because
    // we don't have a `G` in scope. A perhaps better design would be either to
    // make `EdgeRef` polymorphic in `G` and use `Id<G::Node>` and `Id<G::Edge>`
    // and have `edge_data: &'a G::EdgeData` or simply parameterise the `EdgeRef`
    // from an additional `EdgeData/EdgeType` parameter.
    pub edge_type: &'a EdgeType,
}

pub type EdgeRefs<'a, N, E> = Vec<EdgeRef<'a, N, E>>;
