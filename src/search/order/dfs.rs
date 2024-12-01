use super::Order;

pub struct DFS<S> {
    stack: Vec<S>,
}

impl<S> DFS<S> {
    pub fn new() -> Self {
        DFS {
            stack: Vec::with_capacity(64),
        }
    }
}

impl<S> Order<S> for DFS<S> {
    #[inline]
    fn reset(&mut self) {
        self.stack.clear();
    }

    #[inline]
    fn push(&mut self, state: S) {
        self.stack.push(state)
    }

    #[inline]
    fn next(&mut self) -> Option<S> {
        self.stack.pop()
    }
}
