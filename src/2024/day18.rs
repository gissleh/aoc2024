use arrayvec::ArrayVec;
use bit_vec::BitVec;
use common::grid::{Grid, GridCoordinate};
use common::parser;
use common::parser::Parser;
use common::runner::Runner;
use common::search::{bfs, Key, Order, Search, SeenSpace};
use common::utils::UnionFind;

const START: (u8, u8) = (0, 0);
const END: (u8, u8) = (70, 70);

type ByteGrid = Grid<(u8, u8), [u16; 71 * 71], u16>;

pub fn main(r: &mut Runner, input: &[u8]) {
    let (grid, points) = r.prep("Parse", || parser(71, 71).parse_value(input).unwrap());

    r.part("Part 1", || part_1(&grid));
    r.part("Part 2", || part_2(&grid, points.len() as u16));

    r.set_tail("Part 1");
    r.part("Part 2 (Union Set)", || {
        part_2_union_find(*grid.size(), &points)
    });

    r.info("Bytes", &points.len());
}

fn part_1(grid: &ByteGrid) -> u32 {
    let mut search = bfs().with_seen_space(SeenGrid::new());
    run_pathfinding(grid, 1024, &mut search).unwrap()
}

type GridSearch<O> = Search<((u8, u8), u32), SeenGrid, O>;

fn run_pathfinding<O>(grid: &ByteGrid, limit: u16, search: &mut GridSearch<O>) -> Option<u32>
where
    O: Order<((u8, u8), u32)>,
{
    search.push((START, 0));
    search.find(|search, ((x, y), cost)| {
        if grid[(x, y)] > 0 && grid[(x, y)] <= limit {
            return None;
        }

        if (x, y) == END {
            return Some(cost);
        }

        if y > 0 {
            search.push(((x, y - 1), cost + 1));
        }
        if x > 0 {
            search.push(((x - 1, y), cost + 1));
        }
        if x < grid.size().0 - 1 {
            search.push(((x + 1, y), cost + 1));
        }
        if y < grid.size().1 - 1 {
            search.push(((x, y + 1), cost + 1));
        }

        None
    })
}

fn part_2(grid: &ByteGrid, max_byte: u16) -> String {
    let mut step_size = (max_byte - 1024) / 2;
    let mut current = 1024 + step_size;
    let mut search = bfs().with_seen_space(SeenGrid::new());

    step_size /= 2;

    while current > 1024 && current < max_byte {
        if run_pathfinding(&grid, current, &mut search).is_none() {
            if step_size == 1 {
                let (x, y) = grid
                    .iter()
                    .find(|(_, cell)| **cell == current)
                    .map(|((x, y), _)| (x, y))
                    .unwrap();

                return format!("{},{}", x, y);
            } else {
                current -= step_size;
            }
        } else {
            current += step_size;
        }

        if step_size > 1 {
            step_size /= 2;
        }

        search.reset();
    }

    ":(".to_owned()
}

fn part_2_union_find(size: (u8, u8), order: &[(u8, u8)]) -> String {
    let area = size.area();
    let top_right = area;
    let bottom_left = area + 1;
    let row_offset = size.0 as usize;
    let r = size.0 - 1;
    let b = size.1 - 1;

    let mut uf = UnionFind::new(area + 2);
    let mut seen = BitVec::from_elem(area, false);
    let mut allowed_neighbors = ArrayVec::<_, 8>::new();

    for (x, y) in order.iter().copied() {
        let index = (x, y).index(&size);

        allowed_neighbors.clear();
        if x != 0 && y != 0 {
            allowed_neighbors.push(index - row_offset - 1);
        }
        if y != 0 {
            allowed_neighbors.push(index - row_offset);
        }
        if y != 0 && x != r {
            allowed_neighbors.push(index - row_offset + 1);
        }
        if x != 0 {
            allowed_neighbors.push(index - 1);
        }
        if x != 0 && y != b {
            allowed_neighbors.push(index + row_offset - 1);
        }
        if y != b {
            allowed_neighbors.push(index + row_offset);
        }
        if x != r && y != b {
            allowed_neighbors.push(index + row_offset + 1);
        }
        if x != r {
            allowed_neighbors.push(index + 1);
        }

        for neigh in allowed_neighbors.iter() {
            if seen[*neigh] {
                uf.union(index, *neigh);
            }
        }

        if x == r || y == 0 {
            uf.union(top_right, index);
        }
        if x == 0 || y == b {
            uf.union(bottom_left, index);
        }

        seen.set(index, true);

        if uf.find(top_right) == uf.find(bottom_left) {
            return format!("{x},{y}");
        }
    }

    ":(".to_owned()
}

fn parser<'i>(w: u8, h: u8) -> impl Parser<'i, (ByteGrid, Vec<(u8, u8)>)> {
    parser::uint::<u8>()
        .and_discard(b',')
        .and(parser::uint::<u8>())
        .and_discard(b'\n')
        .repeat_fold_mut(
            move || {
                (
                    ByteGrid::new_with_default((w, h), [0u16; 71 * 71], 0),
                    Vec::with_capacity(4096),
                )
            },
            |(grid, points), (x, y)| {
                points.push((x, y));
                grid[(x, y)] = points.len() as u16;
            },
        )
}

struct SeenGrid {
    data: [u32; ((71 * 71) / 32) + 1],
}

impl SeenGrid {
    fn new() -> Self {
        Self {
            data: [0; ((71 * 71) / 32) + 1],
        }
    }

    #[inline]
    fn index_of(&self, pos: (u8, u8)) -> (usize, u32) {
        let index = pos.index(&(71, 71));
        (index / 32, (index % 32) as u32)
    }
}

impl<S> SeenSpace<S> for SeenGrid
where
    S: Key<(u8, u8)>,
{
    fn reset(&mut self) {
        self.data.fill(0);
    }

    fn has_seen(&self, state: &S) -> bool {
        let (index, bit) = self.index_of(state.key());
        self.data[index] & (1 << bit) != 0
    }

    fn try_mark_seen(&mut self, state: S) -> bool {
        let (index, bit) = self.index_of(state.key());

        if self.data[index] & (1 << bit) == 0 {
            self.data[index] |= 1 << bit;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_2_uf_works_on_example() {
        assert_eq!(part_2_union_find((7, 7), EXAMPLE_POINTS), "6,1",)
    }

    const EXAMPLE_POINTS: &[(u8, u8)] = &[
        (5, 4),
        (4, 2),
        (4, 5),
        (3, 0),
        (2, 1),
        (6, 3),
        (2, 4),
        (1, 5),
        (0, 6),
        (3, 3),
        (2, 6),
        (5, 1),
        (1, 2),
        (5, 5),
        (2, 5),
        (6, 5),
        (1, 4),
        (0, 4),
        (6, 4),
        (1, 1),
        (6, 1),
        (1, 0),
        (0, 5),
        (1, 6),
        (2, 0),
    ];
}
