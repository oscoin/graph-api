//! Concrete node and edge types used in the registry.

extern crate num_traits;
#[cfg(feature = "quickcheck")]
extern crate quickcheck;

use num_traits::Zero;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

#[cfg(feature = "quickcheck")]
use quickcheck::{Arbitrary, Gen};

/// The type of a node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
    /// A user, eg. contributor, project member etc.
    User,
    /// A project with users as members and contributors.
    Project,
}

#[cfg(feature = "quickcheck")]
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

#[cfg(feature = "quickcheck")]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EdgeType {
    /// Contribution from a project to a user. Corresponds to `contrib` from the paper.
    ContributionFromUser,
    /// Contribution from a user to a project. Corresponds to `contribᵒ` from the paper.
    ContributionToProject,
    /// Membership of a user in a project. Corresponds to `maintain` from the paper.
    MaintenanceFromUser,
    /// Membership of a user in a project. Correspond to `maintainᵒ` from the paper.
    Maintenance,
    /// One-way dependency between two projects. Correspond to `depend` from the paper.
    Dependency,
}

/// Edge data.
#[derive(Debug, Clone, PartialEq)]
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

impl<W: Add<Output = W>> Add for NodeRank<W> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        NodeRank {
            rank: self.rank + other.rank,
        }
    }
}

impl<W> Zero for NodeRank<W>
where
    W: Zero,
{
    fn zero() -> Self {
        NodeRank { rank: W::zero() }
    }

    fn is_zero(&self) -> bool {
        self.rank.is_zero()
    }
}

#[cfg(feature = "quickcheck")]
// TODO(adn) If we really want precise *bounded* ranks, then we need to
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

/// Global DampingFactors used by the graph algorithm.
#[derive(Clone, Debug)]
pub struct DampingFactors {
    /// Probability that a random walk on a project node continues.
    pub project: f64,
    /// Probability that a random walk on a user node continues.
    pub account: f64,
}

/// Global parameters used by the graph algorithm.
#[derive(Clone, Debug)]
pub struct HyperParameters<W> {
    /// Also `tau`. Threshold below which nodes are pruned in the first
    /// phase of the algorithm.
    pub pruning_threshold: W,
    pub damping_factors: DampingFactors,
    /// 'R' value.
    pub r_value: u32,
    /// Weights for the different edge types.
    pub edge_weights: HashMap<EdgeType, W>,
}

impl<W> HyperParameters<W> {
    /// Get the hyper value associated to the input `EdgeType`. It panics at
    /// runtime if the value cannot be found.
    pub fn get_param(&self, edge_type: &EdgeType) -> &W {
        self.edge_weights
            .get(edge_type)
            .unwrap_or_else(|| panic!("hyperparam value for {:#?} not found.", edge_type))
    }
}
