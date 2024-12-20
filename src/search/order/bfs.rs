use super::Order;
use std::collections::VecDeque;

pub struct BFS<S> {
    queue: VecDeque<S>,
}

impl<S> BFS<S> {
    pub fn new() -> Self {
        BFS {
            queue: VecDeque::with_capacity(64),
        }
    }
}

impl<S> Order<S> for BFS<S> {
    #[inline]
    fn reset(&mut self) {
        self.queue.clear();
    }

    #[inline]
    fn push(&mut self, state: S) {
        self.queue.push_back(state)
    }

    #[inline]
    fn next(&mut self) -> Option<S> {
        self.queue.pop_front()
    }
}
