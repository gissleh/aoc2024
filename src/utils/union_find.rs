pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<u32>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let parents = (0..size).collect();
        let ranks = vec![1; size];

        Self { parents, ranks }
    }

    #[inline]
    pub fn find(&self, x: usize) -> usize {
        let mut current = self.parents[x];
        while current != self.parents[current] {
            current = self.parents[current];
        }

        current
    }

    #[inline]
    pub fn group_count(&mut self) -> usize {
        self.parents
            .iter()
            .enumerate()
            .filter(|(i, p)| **p == *i)
            .count()
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);

        if px != py {
            if self.ranks[py] <= self.ranks[px] {
                self.parents[px] = py;
                self.ranks[py] += 1;
            } else {
                self.parents[py] = px;
                self.ranks[px] += 1;
            }

            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example_works() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(4, 2);
        uf.union(5, 4);

        assert_eq!(uf.group_count(), 2);
        assert_eq!(uf.find(0), uf.find(1));
        assert_eq!(uf.find(1), uf.find(2));
        assert_eq!(uf.find(0), uf.find(2));
        assert_eq!(uf.find(0), uf.find(5));
        assert_ne!(uf.find(2), uf.find(3));
    }

    #[test]
    fn ram_run_example() {}
}
