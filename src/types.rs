//! Concrete node and edge types used in the registry.

/// The type of a node.
pub enum NodeType {
    /// A user, eg. contributor, project member etc.
    User,
    /// A project with users as members and contributors.
    Project,
}

/// The type of an edge.
pub enum EdgeType {
    /// Contribution, user -> project, with strength metric.
    ContributionToProject(u32),
    /// Contribution, project -> user, with strength metric.
    ContributionFromUser(u32),
    /// Membership of a user in a project, project <-> user.
    ProjectMembership,
    /// One-way dependency between two projects, project -> project.
    ProjectDependency,
}

/// The rank or "osrank" of a node, normalized to `1.0`.
pub type NodeRank = f64;

/// Global parameters used by the graph algorithm.
pub struct HyperParameters {
    /// Threshold below which nodes are pruned in the first
    /// phase of the algorithm.
    pruning_threshold: f64,
    /// How often do random walks return to the seed set.
    dampening_factor: f64,
    /// 'R' value.
    r_value: f64,
}
