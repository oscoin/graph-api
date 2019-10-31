//! Concrete node and edge types used in the registry.

extern crate num_traits;
#[cfg(feature = "quickcheck")]
extern crate quickcheck;

use num_traits::Zero;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

use super::{Graph, Id};

#[cfg(feature = "quickcheck")]
use quickcheck::{Arbitrary, Gen};

/// The type of a node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
    /// A user, eg. contributor, project member etc.
    User { contributions_to_all_projects: u32 },
    /// A project with users as members and contributors.
    Project { contributions_from_all_users: u32 },
}

impl NodeType {
    /// Increments the current contributions for this `NodeType` by 'c'.
    pub fn add_contributions(&mut self, c: u32) {
        match self {
            NodeType::User {
                contributions_to_all_projects,
            } => {
                *contributions_to_all_projects += c;
            }
            NodeType::Project {
                contributions_from_all_users,
            } => {
                *contributions_from_all_users += c;
            }
        }
    }

    /// Set the contributions to the given value.
    pub fn set_contributions(&mut self, c: u32) {
        match self {
            NodeType::User {
                contributions_to_all_projects,
            } => {
                *contributions_to_all_projects = c;
            }
            NodeType::Project {
                contributions_from_all_users,
            } => {
                *contributions_from_all_users = c;
            }
        }
    }

    pub fn total_contributions(&self) -> u32 {
        match self {
            NodeType::User {
                contributions_to_all_projects,
            } => *contributions_to_all_projects,
            NodeType::Project {
                contributions_from_all_users,
            } => *contributions_from_all_users,
        }
    }
}

#[cfg(feature = "quickcheck")]
impl Arbitrary for NodeType {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let contribs = Arbitrary::arbitrary(g);
        if g.next_u32() % 2 == 0 {
            Self::User {
                contributions_to_all_projects: contribs,
            }
        } else {
            Self::Project {
                contributions_from_all_users: contribs,
            }
        }
    }
}

/// Node data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeData<W> {
    /// The type for this node.
    pub node_type: NodeType,
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
            rank: Arbitrary::arbitrary(g),
        }
    }
}

/// The type of an edge. When allowed, it bundles together the number of
/// contributions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EdgeType {
    /// Contribution from a project to a user. Corresponds to `contrib` from the paper.
    ProjectToUserContribution(u32),
    /// Contribution from a user to a project. Corresponds to `contribᵒ` from the paper.
    UserToProjectContribution(u32),
    /// Membership relation from a project to a user. Corresponds to `maintain` from the paper.
    ProjectToUserMembership(u32),
    /// Membership relation from a user to a project. Correspond to `maintainᵒ` from the paper.
    UserToProjectMembership(u32),
    /// One-way dependency between two projects. Correspond to `depend` from the paper.
    Dependency,
}

/// A companion tag for an `EdgeType`, to allow the former to be used as a key
/// in a `HashMap`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EdgeTypeTag {
    ProjectToUserContribution,
    UserToProjectContribution,
    ProjectToUserMembership,
    UserToProjectMembership,
    Dependency,
}

impl EdgeType {
    pub fn to_tag(&self) -> EdgeTypeTag {
        match self {
            EdgeType::ProjectToUserContribution(_) => EdgeTypeTag::ProjectToUserContribution,
            EdgeType::UserToProjectContribution(_) => EdgeTypeTag::UserToProjectContribution,
            EdgeType::ProjectToUserMembership(_) => EdgeTypeTag::ProjectToUserMembership,
            EdgeType::UserToProjectMembership(_) => EdgeTypeTag::UserToProjectMembership,
            EdgeType::Dependency => EdgeTypeTag::Dependency,
        }
    }

    pub fn total_contributions(&self) -> u32 {
        match self {
            EdgeType::ProjectToUserContribution(c) => *c,
            EdgeType::UserToProjectContribution(c) => *c,
            EdgeType::ProjectToUserMembership(c) => *c,
            EdgeType::UserToProjectMembership(c) => *c,
            EdgeType::Dependency => 0,
        }
    }
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
    pub edge_weights: HashMap<EdgeTypeTag, W>,
}

impl<W> HyperParameters<W> {
    /// Get the hyper value associated to the input `EdgeTypeTag`. It panics at
    /// runtime if the value cannot be found.
    pub fn get_param(&self, edge_type_tag: &EdgeTypeTag) -> &W {
        self.edge_weights
            .get(&edge_type_tag)
            .unwrap_or_else(|| panic!("hyperparam value for {:#?} not found.", edge_type_tag))
    }
}

/* Concrete types for the incremental MonteCarlo algorithm. */

/// This is an enumeration of all the possible ways in which a `Graph` can be
/// mutated/transformed between two Osrank invocations.
pub enum GraphDiff<'a, G: 'a>
where
    G: Graph,
{
    /// A new node has been added to the network.
    NodeAdded(&'a Id<G::Node>),
    /// An existing node has been deleted from the network. We require full
    /// ownership over the `G::Node` because once the registry deleted it from
    /// the network we cannot query the network anymore to fetch more interesting
    /// data out of it.
    NodeDeleted(G::Node),
    /// A `Node` has been updated. For now updates are not relevant to `Osrank`,
    /// but they might in the future.
    NodeUpdated(&'a Id<G::Node>),
    /// A new edge has been added to the network.
    EdgeAdded {
        id: &'a Id<G::Edge>,
        source: &'a Id<G::Node>,
        target: &'a Id<G::Node>,
    },
    /// An existing edge has been deleted from the network. We require full
    /// ownership over the `G::Edge` for the same reasons of `NodeDeleted`.
    EdgeDeleted(G::Edge),
    // NOTE: There is no `EdgeUpdated` by design: this is because the only
    // reason why an edge might be updated is either to change "Direction"
    // (which seems unlikely and wrong to begin with) or to bump the number
    // of contributions. But in a "multi-version" world like this one, this is
    // *not* what happens. Rather, every time a new contributions contributes
    // you do *not* update an existing node but rather the next checkpoint a brand
    // new edge is added (with the new contributions) and a new *project version*
    // is released.
}

/// An Iterator over a collection of `GraphDiff`.
pub struct GraphDiffs<'a, G: 'a>
where
    G: Graph,
{
    pub range: std::vec::IntoIter<&'a GraphDiff<'a, G>>,
}

impl<'a, G> Iterator for GraphDiffs<'a, G>
where
    G: Graph,
{
    type Item = &'a GraphDiff<'a, G>;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next()
    }
}
