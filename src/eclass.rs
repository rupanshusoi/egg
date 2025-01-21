use std::fmt::{Debug, Display};
use std::iter::ExactSizeIterator;

use crate::*;

#[derive(Clone, PartialOrd, PartialEq, Eq, Ord, Debug, Hash)]
#[cfg_attr(feature = "serde-1", derive(serde::Serialize, serde::Deserialize))]
pub struct ENode<L> {
    pub node: L,
    pub version: usize,
}

impl<L: Display> Display for ENode<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.node.fmt(f)
    }
}

impl<L: Language> ENode<L> {
    pub fn discriminant(&self) -> L::Discriminant {
        self.node.discriminant()
    }

    pub fn matches(&self, other: &Self) -> bool {
        self.node.matches(&other.node)
    }

    pub fn children(&self) -> &[Id] {
        self.node.children()
    }

    pub fn children_mut(&mut self) -> &mut [Id] {
        self.node.children_mut()
    }
}

/// An equivalence class of enodes.
#[non_exhaustive]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-1", derive(serde::Serialize, serde::Deserialize))]
pub struct EClass<L, D> {
    /// This eclass's version.
    pub version: usize,
    /// This eclass's id.
    pub id: Id,
    /// The equivalent enodes in this equivalence class.
    pub nodes: Vec<ENode<L>>,
    /// The analysis data associated with this eclass.
    ///
    /// Modifying this field will _not_ cause changes to propagate through the e-graph.
    /// Prefer [`EGraph::set_analysis_data`] instead.
    pub data: D,
    /// The original Ids of parent enodes.
    pub(crate) parents: Vec<Id>,
}

impl<L, D> EClass<L, D> {
    /// Returns `true` if the `eclass` is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns the number of enodes in this eclass.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Iterates over the enodes in this eclass.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &ENode<L>> {
        self.nodes.iter()
    }

    /// Iterates over the non-canonical ids of parent enodes of this eclass.
    pub fn parents(&self) -> impl ExactSizeIterator<Item = Id> + '_ {
        self.parents.iter().copied()
    }
}

impl<L: Language, D> EClass<L, D> {
    /// Iterates over the childless enodes in this eclass.
    pub fn leaves(&self) -> impl Iterator<Item = &ENode<L>> {
        self.nodes.iter().filter(|&n| n.node.is_leaf())
    }

    /// Asserts that the childless enodes in this eclass are unique.
    pub fn assert_unique_leaves(&self)
    where
        L: Language,
    {
        let mut leaves = self.leaves();
        if let Some(first) = leaves.next() {
            assert!(
                leaves.all(|l| l == first),
                "Different leaves in eclass {}: {:?}",
                self.id,
                self.leaves().collect::<crate::util::HashSet<_>>()
            );
        }
    }
}
