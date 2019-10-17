//! Concrete node and edge types used in the registry.

extern crate quickcheck;

use quickcheck::{Arbitrary, Gen};
use std::collections::HashMap;
use std::hash::Hash;

/// The type of a node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
    /// A user, eg. contributor, project member etc.
    User,
    /// A project with users as members and contributors.
    Project,
}

impl Arbitrary for NodeType {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        if g.next_u32() % 2 == 0 {
            Self::User
        } else {
            Self::Project
        }
    }
}

/// Node data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeData<W> {
    /// The type for this node.
    pub node_type: NodeType,
    /// The total contributions by this user, to *all* projects, if any.
    pub total_contributions: Option<u32>,
    pub rank: NodeRank<W>,
}

impl<W> Arbitrary for NodeData<W>
where
    W: Arbitrary,
{
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        NodeData {
            node_type: Arbitrary::arbitrary(g),
            total_contributions: Arbitrary::arbitrary(g),
            rank: Arbitrary::arbitrary(g),
        }
    }
}

/// The type of an edge.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EdgeType {
    /// Contribution from a project to a user.
    Contrib,
    /// Contribution from a user to a project.
    ContribStar,
    /// Membership of a user in a project, project -> user.
    Maintain,
    /// Membership of a user in a project, user -> project.
    MaintainStar,
    /// One-way dependency between two projects, project -> project.
    Depend,
}

/// Edge data.
#[derive(Debug, Clone)]
pub struct EdgeData<W> {
    /// The type for this edge.
    pub edge_type: EdgeType,
    /// The weight of this specific edge. Can be used to weight for eg.
    /// edges with more contributions higher, or weigh certain dependencies
    /// higher than others.
    pub weight: W,
    /// The contributions of the user towards a project, if any.
    pub contributions: Option<u32>,
}

/// The rank or "osrank" of a node, normalized to `1.0`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeRank<W> {
    pub rank: W,
}

// FIXME(adn) If we really want precise *bounded* ranks, then we need to
// pull the `num::Bounded` trait from the `num` crate.
impl<W> Arbitrary for NodeRank<W>
where
    W: Arbitrary,
{
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        NodeRank {
            rank: Arbitrary::arbitrary(g),
        }
    }
}

/// Global parameters used by the graph algorithm.
#[derive(Debug)]
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
