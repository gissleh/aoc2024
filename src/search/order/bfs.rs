use super::Order;
use std::collections::VecDeque;

pub struct BFS<S> {
    stack: VecDeque<S>,
}

impl<S> BFS<S> {
    pub fn new() -> Self {
        BFS {
            stack: VecDeque::with_capacity(64),
        }
    }
}

impl<S> Order<S> for BFS<S> {
    #[inline]
    fn reset(&mut self) {
        self.stack.clear();
    }

    #[inline]
    fn push(&mut self, state: S) {
        self.stack.push_back(state)
    }

    #[inline]
    fn next(&mut self) -> Option<S> {
        self.stack.pop_front()
    }
}
