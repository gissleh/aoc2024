use common::grid::{Grid, GridCoordinate};
use common::parser;
use common::parser::Parser;
use common::runner::Runner;
use common::search::{bfs, Key, Order, SeenSpace};

const START: (u8, u8) = (0, 0);
const END: (u8, u8) = (70, 70);

type ByteGrid = Grid<(u8, u8), [u16; 71 * 71], u16>;

pub fn main(r: &mut Runner, input: &[u8]) {
    let grid = r.prep("Parse", || parser(71, 71).parse_value(input).unwrap());

    r.part("Part 1", || part_1(&grid, 1024).unwrap());
    r.part("Part 2", || part_2(&grid));

    r.info("Bytes", &grid.iter().map(|(_, v)| *v).max().unwrap());
}

fn part_1(grid: &ByteGrid, limit: u16) -> Option<u32> {
    let mut search = bfs().with_seen_space(SeenGrid::new());
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

fn part_2(grid: &ByteGrid) -> String {
    for limit in 1025..u16::MAX {
        if part_1(&grid, limit).is_none() {
            let (x, y) = grid
                .iter()
                .find(|(_, cell)| **cell == limit)
                .map(|((x, y), _)| (x, y))
                .unwrap();

            return format!("{},{}", x, y);
        }
    }

    ":(".to_owned()
}

fn parser<'i>(w: u8, h: u8) -> impl Parser<'i, ByteGrid> {
    parser::uint::<u8>()
        .and_discard(b',')
        .and(parser::uint::<u8>())
        .and_discard(b'\n')
        .repeat_fold(
            move || (ByteGrid::new_with_default((w, h), [0u16; 71 * 71], 0), 1u16),
            |(mut grid, next), (x, y)| {
                grid[(x, y)] = next;
                (grid, next + 1)
            },
        )
        .map(|(grid, _)| grid)
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
