#[macro_use]
mod attributes;
mod order;
mod seen;

use std::marker::PhantomData;
use std::ops::Add;

use order::bfs::BFS;
use order::dfs::DFS;
use order::dijkstra::{AStarBinaryHeap, DijkstraBinaryHeap};

use crate::search::attributes::Heuristic;
use crate::utils::GatherTarget;
pub use attributes::{Cost, Key, OnlyKey};
pub use order::Order;
pub use seen::{NoSeenSpace, SeenSpace};

pub struct Search<S, SEEN, ORDER>
where
    SEEN: SeenSpace<S>,
    ORDER: Order<S>,
{
    seen: SEEN,
    order: ORDER,
    spooky_ghost: PhantomData<S>,
}

impl<S, SEEN, ORDER> Search<S, SEEN, ORDER>
where
    SEEN: SeenSpace<S>,
    ORDER: Order<S>,
    S: Copy,
{
    pub fn dissolve(self) -> (SEEN, ORDER) {
        (self.seen, self.order)
    }

    pub fn reset(&mut self) {
        self.order.reset();
        self.seen.reset();
    }

    pub fn push(&mut self, s: S) {
        if self.seen.try_mark_seen(s) {
            self.order.push(s);
        }
    }

    pub fn find<F, T>(&mut self, f: F) -> Option<T>
    where
        F: Fn(&mut Self, S) -> Option<T>,
    {
        while let Some(step) = self.order.next() {
            if let Some(res) = f(self, step) {
                return Some(res);
            }
        }

        None
    }

    pub fn gather<G, F, T>(&mut self, f: F) -> G
    where
        F: Fn(&mut Self, S) -> Option<T>,
        G: GatherTarget<T>,
    {
        let mut target = G::init_gather_target(0);
        let mut i = 0;
        while let Some(res) = self.find(|s, state| f(s, state)) {
            if !target.gather(i, res) {
                break;
            }
            i += 1;
        }

        target
    }
}

pub fn bfs<S>() -> impl Order<S> {
    BFS::new()
}
pub fn dfs<S>() -> impl Order<S> {
    DFS::new()
}
pub fn dijkstra<C, S>() -> impl Order<S>
where
    S: Cost<C>,
    C: Eq + Ord,
{
    DijkstraBinaryHeap::new()
}
pub fn a_star<C, S>() -> impl Order<S>
where
    S: Cost<C> + Heuristic<C>,
    C: Eq + Ord + Add<Output = C>,
{
    AStarBinaryHeap::new()
}
