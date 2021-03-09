use std::{iter, slice, cmp::{Reverse, Ordering}, collections::{binary_heap::BinaryHeap, HashMap}, io::{BufRead, Write}, fmt};

pub struct BinaryHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> FromIterator<T> for BinaryHeap<T> {
    fn from_iter(iter: impl IntoIterator<Item = T>
}

#[derive(PartialEq, Eq, Debug)]
pub enum Node<T> {
    // in our specific case, nodes never have only one child
    Branch(Box<[Node<T>; 2]>),
    Leaf(T),
}

struct NodeIterator<T>(Node<T>);
