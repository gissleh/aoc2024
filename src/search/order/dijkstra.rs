use crate::search::attributes::Heuristic;
use crate::search::{Cost, Order};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::Add;

pub struct DijkstraBinaryHeap<C, S>
where
    S: Cost<C>,
    C: Eq + Ord,
{
    heap: BinaryHeap<OrderedByCost<C, S>>,
}

impl<C, S> DijkstraBinaryHeap<C, S>
where
    S: Cost<C>,
    C: Eq + Ord,
{
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::with_capacity(64),
        }
    }
}

impl<C, S> Order<S> for DijkstraBinaryHeap<C, S>
where
    S: Cost<C>,
    C: Eq + Ord,
{
    #[inline]
    fn reset(&mut self) {
        self.heap.clear();
    }

    #[inline]
    fn push(&mut self, state: S) {
        self.heap.push(OrderedByCost(state.cost(), state))
    }

    #[inline]
    fn next(&mut self) -> Option<S> {
        self.heap.pop().map(|e| e.1)
    }
}

pub struct AStarBinaryHeap<C, S>
where
    S: Cost<C> + Heuristic<C>,
    C: Eq + Ord + Add<Output = C>,
{
    heap: BinaryHeap<OrderedByCost<C, S>>,
}

impl<C, S> AStarBinaryHeap<C, S>
where
    S: Cost<C> + Heuristic<C>,
    C: Eq + Ord + Add<Output = C>,
{
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::with_capacity(64),
        }
    }
}

impl<C, S> Order<S> for AStarBinaryHeap<C, S>
where
    S: Cost<C> + Heuristic<C>,
    C: Eq + Ord + Add<Output = C>,
{
    #[inline]
    fn reset(&mut self) {
        self.heap.clear();
    }

    #[inline]
    fn push(&mut self, state: S) {
        self.heap
            .push(OrderedByCost(state.cost() + state.heuristic(), state))
    }

    #[inline]
    fn next(&mut self) -> Option<S> {
        self.heap.pop().map(|e| e.1)
    }
}

struct OrderedByCost<C, S>(C, S);

impl<C, S> Eq for OrderedByCost<C, S> where C: Eq + Ord {}

impl<C, S> PartialEq<Self> for OrderedByCost<C, S>
where
    C: Eq + Ord,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<C, S> PartialOrd<Self> for OrderedByCost<C, S>
where
    C: Eq + Ord,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<C, S> Ord for OrderedByCost<C, S>
where
    C: Ord + Eq,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
