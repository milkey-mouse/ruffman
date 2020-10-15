use std::{cmp::{Reverse, Ordering}, collections::{binary_heap::BinaryHeap, HashMap}, io::{BufRead, Write}, fmt};

#[derive(PartialEq, Eq)]
pub enum Node<T> {
    // in our specific case, nodes never have only one child
    Branch(Box<(Node<T>, Node<T>)>),
    Leaf(T),
}

impl<T: fmt::Debug /*Display*/> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Branch(b) => write!(f, "({} {})", b.0, b.1),
            Self::Leaf(t) => write!(f, "{:?}", t), //t.fmt(f),
        }
    }
}

pub struct WeightedNode<T> {
    weight: usize,
    node: Node<T>,
}

impl<T> WeightedNode<T> {
    #[inline]
    pub fn leaf(data: T, weight: usize) -> Self {
        Self { weight, node: Node::Leaf(data) }
    }

    pub fn parent<I: Into<WeightedNode<T>>>(left: I, right: I) -> Self {
        let (l, r) = (left.into(), right.into());
        Self {
            weight: l.weight + r.weight,
            node: Node::Branch(Box::new((l.node, r.node))),
        }
    }
}

impl<T: fmt::Debug> fmt::Display for WeightedNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <weight {}>", self.node, self.weight)
    }
}

impl<T> PartialOrd for WeightedNode<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<T> Ord for WeightedNode<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

// TODO: should these be auto-derived (for actual equality)?
impl<T> PartialEq for WeightedNode<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<T> Eq for WeightedNode<T> {}

// we need a reverse-weighted node to turn the standard library's max-heap
// into a min-heap, but we can't just use std::cmp::Reverse because of non-
// forwarded traits...
#[derive(PartialEq, Eq)]
pub struct ReverseWeightedNode<T>(WeightedNode<T>);

impl<T> ReverseWeightedNode<T> {
    #[inline]
    pub fn leaf(data: T, weight: usize) -> Self {
        Self(WeightedNode::leaf(data, weight))
    }

    #[inline]
    pub fn parent<I: Into<WeightedNode<T>>>(left: I, right: I) -> Self {
        Self(WeightedNode::parent(left, right))
    }
}

impl<T: fmt::Debug> fmt::Display for ReverseWeightedNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: PartialOrd> PartialOrd for ReverseWeightedNode<T> {
    #[inline]
    fn partial_cmp(&self, other: &ReverseWeightedNode<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for ReverseWeightedNode<T> {
    #[inline]
    fn cmp(&self, other: &ReverseWeightedNode<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T> From<ReverseWeightedNode<T>> for WeightedNode<T> {
    #[inline]
    fn from(reverse: ReverseWeightedNode<T>) -> Self {
        reverse.0
    }
}

impl<T> From<WeightedNode<T>> for ReverseWeightedNode<T> {
    #[inline]
    fn from(obverse: WeightedNode<T>) -> Self {
        Self(obverse)
    }
}
