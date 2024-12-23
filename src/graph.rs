use arrayvec::ArrayVec;

pub struct Graph<N, E, const CAP: usize> {
    nodes: Vec<N>,
    edges: Vec<ArrayVec<(usize, E), CAP>>,
}

impl<N, E, const CAP: usize> Graph<N, E, CAP>
where
    N: Eq,
{
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Graph {
            nodes: Vec::with_capacity(cap),
            edges: Vec::with_capacity(cap),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn ensure_node(&mut self, node: N) -> usize {
        match self.nodes.iter().position(|n| n == &node) {
            Some(index) => index,
            None => {
                self.nodes.push(node);
                self.edges.push(ArrayVec::new());
                self.nodes.len() - 1
            }
        }
    }

    pub fn add_node(&mut self, node: N) -> usize {
        self.nodes.push(node);
        self.edges.push(ArrayVec::new());
        self.nodes.len() - 1
    }

    #[inline]
    pub fn node(&self, index: usize) -> &N {
        &self.nodes[index]
    }

    #[inline]
    pub fn node_mut(&mut self, index: usize) -> &mut N {
        &mut self.nodes[index]
    }

    #[inline]
    pub fn node_index(&self, node: &N) -> Option<usize> {
        self.nodes.iter().position(|n| n == node)
    }

    #[inline]
    pub fn node_index_by_ref<K>(&self, key: &K) -> Option<usize>
    where
        K: Ord + ?Sized,
        N: AsRef<K>,
    {
        self.nodes.iter().position(|n| n.as_ref() == key)
    }

    #[inline]
    pub fn nodes(&self) -> &[N] {
        &self.nodes
    }

    pub fn roots(&self) -> impl Iterator<Item = (usize, &N)> {
        self.nodes.iter().enumerate().filter(|(i, _)| {
            self.edges
                .iter()
                .find(|e| e.iter().find(|(dst, _)| *dst == *i).is_some())
                .is_none()
        })
    }

    #[inline]
    pub fn connect(&mut self, src: usize, dst: usize, edge: E) {
        self.edges[src].push((dst, edge));
    }

    #[inline]
    pub fn edges(&self, src: usize) -> &[(usize, E)] {
        self.edges[src].as_slice()
    }

    #[inline]
    pub fn edge(&self, src: usize, dst: usize) -> Option<&E> {
        self.edges[src]
            .iter()
            .find(|(d, _)| *d == dst)
            .map(|(_, e)| e)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn edge_count(&self) -> usize {
        self.edges.iter().map(|l| l.len()).sum()
    }
}

impl<N, E, const CAP: usize> Graph<N, E, CAP>
where
    E: Copy,
{
    pub fn connect_mutual(&mut self, src: usize, dst: usize, edge: E) {
        self.edges[src].push((dst, edge));
        self.edges[dst].push((src, edge));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_graph() {
        let mut graph = Graph::<String, u8, 16>::new();
        assert_eq!(graph.ensure_node(String::from("hello")), 0);
        assert_eq!(graph.ensure_node(String::from("world")), 1);
        assert_eq!(graph.ensure_node(String::from("world")), 1);
        assert_eq!(graph.ensure_node(String::from("blurg")), 2);

        assert_eq!(graph.node_index_by_ref("blurg"), Some(2));
        assert_eq!(graph.node_index_by_ref("hello"), Some(0));
        assert_eq!(graph.node_index_by_ref("world"), Some(1));
        assert_eq!(graph.node_index_by_ref("welt"), None);
    }

    #[test]
    fn test_u32_graph() {
        let mut graph = Graph::<u32, u8, 16>::new();
        assert_eq!(graph.ensure_node(16), 0);
        assert_eq!(graph.ensure_node(32), 1);
        assert_eq!(graph.ensure_node(24), 2);
        assert_eq!(graph.ensure_node(32), 1);

        assert_eq!(graph.node_index(&16), Some(0));
        assert_eq!(graph.node_index(&24), Some(2));
        assert_eq!(graph.node_index(&32), Some(1));
        assert_eq!(graph.node_index(&33), None);
    }

    #[test]
    fn test_struct_graph() {
        #[derive(Debug, Eq, PartialEq)]
        struct CustomNode(u32, String);

        impl AsRef<u32> for CustomNode {
            fn as_ref(&self) -> &u32 {
                &self.0
            }
        }

        let mut graph = Graph::<CustomNode, u8, 16>::new();
        assert_eq!(graph.add_node(CustomNode(16, String::from("Hello"))), 0);
        assert_eq!(graph.add_node(CustomNode(32, String::from("World"))), 1);

        assert_eq!(graph.node_index_by_ref(&16), Some(0));
        assert_eq!(graph.node_index_by_ref(&32), Some(1));
        assert_eq!(graph.node_index_by_ref(&33), None);
    }
}
