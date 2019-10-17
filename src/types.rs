//! Concrete node and edge types used in the registry.

use std::collections::HashMap;

/// The type of a node.
pub enum NodeType {
    /// A user, eg. contributor, project member etc.
    User,
    /// A project with users as members and contributors.
    Project,
}

/// Node data.
pub struct NodeData {
    /// The total contributions by this user, to *all* projects, if any.
    pub total_contributions: Option<u32>,
}

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
pub struct EdgeData<W> {
    /// The type of edge.
    pub edge_type: EdgeType,
    /// The weight of this specific edge. Can be used to weight for eg.
    /// edges with more contributions higher, or weigh certain dependencies
    /// higher than others.
    pub weight: W,
    /// The contributions of the user towards a project, if any.
    pub contributions: Option<u32>,
}

/// The rank or "osrank" of a node, normalized to `1.0`.
pub struct NodeRank<W> {
    pub rank: W,
}

/// Global parameters used by the graph algorithm.
pub struct HyperParameters<W> {
    /// Also `tau`. Threshold below which nodes are pruned in the first
    /// phase of the algorithm.
    pub pruning_threshold: W,
    /// Probability that a random walk on a project node continues.
    pub project_damping_factor: W,
    /// Probability that a random walk on a user node continues.
    pub user_damping_factor: W,
    /// 'R' value.
    pub r_value: u32,
    /// Weights for the different edge types.
    pub edge_weights: HashMap<EdgeType, W>,
}
