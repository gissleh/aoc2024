use rustc_hash::FxHashSet;
use common::grid::Grid;
use common::point::CardinalNeighbors;
use common::runner::Runner;
use common::search::{dfs, NoSeenSpace, OnlyKey, Order, SeenSpace};

pub fn main(r: &mut Runner, input: &[u8]) {
    let map = r.prep("Parse", || TopographicalMap::parse(input));

    r.part("Part 1", || part_1(&map));
    r.part("Part 2", || part_2(&map));

    r.info_debug("Grid Size", map.grid.size());
    r.info("Trailheads", &map.trailheads.len());
}

fn part_1(map: &TopographicalMap) -> usize {
    part_common(map, FxHashSet::with_capacity_and_hasher(64, Default::default()))
}

fn part_2(map: &TopographicalMap) -> usize {
    part_common(map, NoSeenSpace)
}

fn part_common<SEEN: SeenSpace<OnlyKey<(u8, u8)>>>(map: &TopographicalMap, seen: SEEN) -> usize {
    let mut search = dfs().with_seen_space(seen);
    let mut total = 0;

    for trailhead in map.trailheads.iter() {
        search.reset();
        search.push(OnlyKey(*trailhead));
        let score = search.gather::<usize, _, ()>(|search, OnlyKey(pos)| {
            let current_height = map.grid[pos];
            if current_height == 9 {
                return Some(());
            }

            for neigh in pos.cardinal_neighbors() {
                if map.grid[neigh] == current_height + 1 {
                    search.push(OnlyKey(neigh));
                }
            }

            None
        });

        total += score;
    }

    total
}
struct TopographicalMap {
    grid: Grid<(u8, u8), Vec<u8>, u8>,
    trailheads: Vec<(u8, u8)>,
}

impl TopographicalMap {
    fn parse(input: &[u8]) -> Self {
        let width = input.iter().position(|b| *b == b'\n').unwrap();
        let height = input.len() / (width+1);
        let mut grid = Grid::new_with_default(((width+2) as u8, (height+2) as u8), vec![255; (width+2) * (height+2)], 255);
        let mut trailheads = Vec::with_capacity(64);

        let mut pos = (1, 1);
        for v in input.iter() {
            if v == &b'\n' {
                pos = (1, pos.1 + 1);
                continue;
            }
            if *v == b'0' {
                trailheads.push(pos);
            }

            grid[pos] = *v - b'0';
            pos.0 += 1;
        }

        Self{grid, trailheads}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &[u8] = b"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn parses_correctly() {
        let g = TopographicalMap::parse(SAMPLE_1);
        assert_eq!(g.grid.size().0, 8+2);
        assert_eq!(g.grid.size().1, 8+2);
        assert_eq!(g.trailheads.len(), 9);
        assert_eq!(g.trailheads[0], (3, 1));
        assert_eq!(g.trailheads[1], (5, 1));
        assert_eq!(g.trailheads[2], (5, 3));
    }

    #[test]
    fn part1_works_on_example() {
        assert_eq!(part_1(&TopographicalMap::parse(SAMPLE_1)), 36);
    }
}