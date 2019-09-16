//! Concrete node and edge types used in the registry.

/// The type of a node.
pub enum NodeType {
    /// A user, eg. contributor, project member etc.
    User,
    /// A project with users as members and contributors.
    Project,
}

/// Node data.
type NodeData = NodeType;

/// The type of an edge.
pub enum EdgeType {
    /// Contribution from a user to a project. This edge type is
    /// used bi-directionally, between project and user.
    Contribution,
    /// Membership of a user in a project, project <-> user.
    ProjectMembership,
    /// One-way dependency between two projects, project -> project.
    ProjectDependency,
}

/// Edge data.
pub struct EdgeData {
    /// The type of edge.
    edge_type: EdgeType,
    /// The weight of this specific edge. Can be used to weight for eg.
    /// edges with more contributions higher, or weigh certain dependencies
    /// higher than others. Defaults to `1.0`.
    weight: f64,
}

/// The rank or "osrank" of a node, normalized to `1.0`.
pub type NodeRank = f64;

/// Global parameters used by the graph algorithm.
pub struct HyperParameters {
    /// Also `tau`. Threshold below which nodes are pruned in the first
    /// phase of the algorithm.
    pruning_threshold: f64,
    /// Probability that a random walk on a project node continues.
    project_damping_factor: f64,
    /// Probability that a random walk on a user node continues.
    user_damping_factor: f64,
    /// 'R' value.
    r_value: f64,
    /// Weights for the different edge types.
    edge_weights: HashMap<EdgeType, f64>,
}
