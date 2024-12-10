use crate::search::{NoSeenSpace, Search, SeenSpace};

pub mod bfs;
pub mod dfs;
pub mod dijkstra;

pub trait Order<S>: Sized {
    fn reset(&mut self);
    fn push(&mut self, state: S);
    fn next(&mut self) -> Option<S>;

    fn with_seen_space<SEEN>(self, seen: SEEN) -> Search<S, SEEN, Self>
    where
        SEEN: SeenSpace<S>,
    {
        Search {
            seen,
            order: self,
            spooky_ghost: Default::default(),
        }
    }

    fn without_seen_space(self) -> Search<S, NoSeenSpace, Self> {
        self.with_seen_space(NoSeenSpace)
    }
}
