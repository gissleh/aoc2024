use common::grid::Grid;
use common::runner::Runner;
use common::search::{dfs, NoSeenSpace, Order};

pub fn main(r: &mut Runner, input: &[u8]) {
    let xmas_grid = r.prep("Parse", || XmasGrid::parse(input));
    r.info("Grid Width", &xmas_grid.grid.size().0);
    r.info("Grid Height", &xmas_grid.grid.size().1);
    r.part("Part 1", || part_1(&xmas_grid));
    r.set_tail("Parse");
    r.part("Part 1 (DFS)", || part_1_dfs(&xmas_grid));
    r.part("Part 2", || part_2(&xmas_grid));
    r.connect("Part 1", "Part 2");
}

fn part_1(grid: &XmasGrid) -> u32 {
    let mut count = 0;
    let (width, height) = grid.size();
    for x in 0..width {
        for y in 0..height {
            let n = grid.check_xmas((x, y));
            if n > 0 {
                count += n;
            }
        }
    }

    count
}

fn part_1_dfs(grid: &XmasGrid) -> u32 {
    const WORD: &[u8] = b"XMAS";

    let mut search = dfs().with_seen_space(NoSeenSpace);
    let (width, height) = grid.size();
    for y in 0..height {
        for x in 0..width {
            if grid.grid[(x, y)] != b'X' {
                continue;
            }

            for dir in 0..8u8 {
                let next_pos = travel((x, y), dir, 1);
                if let Some(c) = grid.grid.cell(&next_pos) {
                    search.push((*c, next_pos, dir, 1usize));
                }
            }
        }
    }

    let mut total = 0;
    while let Some(_) = search.find(|s, (c, (x, y), dir, index)| {
        if c == WORD[index] {
            if index == 3 {
                Some(())
            } else {
                let next_pos = travel((x, y), dir, 1);
                if let Some(c) = grid.grid.cell(&next_pos) {
                    s.push((*c, next_pos, dir, index + 1));
                }
                None
            }
        } else {
            None
        }
    }) {
        total += 1;
    }

    total
}

fn part_2(grid: &XmasGrid) -> u32 {
    let mut count = 0;
    let (width, height) = grid.size();

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if grid.check_x_mas((x, y)) {
                count += 1;
            }
        }
    }

    count
}

struct XmasGrid {
    grid: Grid<(usize, usize), Vec<u8>, u8>,
}

impl XmasGrid {
    pub fn size(&self) -> (usize, usize) {
        *self.grid.size()
    }

    pub fn check_xmas(&self, pos: (usize, usize)) -> u32 {
        const WORD: &[u8] = b"XMAS";

        if Some(&b'X') == self.grid.cell(&pos) {
            let mut count = 0;
            for dir in 0..8 {
                for n in 1..4 {
                    if Some(&WORD[n]) == self.grid.cell(&travel(pos, dir, n)) {
                        if n == 3 {
                            count += 1;
                        }
                    } else {
                        break;
                    }
                }
            }

            count
        } else {
            0
        }
    }

    pub fn check_x_mas(&self, pos: (usize, usize)) -> bool {
        let (x, y) = pos;
        if Some(&b'A') == self.grid.cell(&pos) {
            let tl = self.grid[(x - 1, y - 1)];
            let tr = self.grid[(x + 1, y - 1)];
            let bl = self.grid[(x - 1, y + 1)];
            let br = self.grid[(x + 1, y + 1)];

            return ((tl == b'M' && br == b'S') || (tl == b'S' && br == b'M'))
                && ((tr == b'M' && bl == b'S') || (tr == b'S' && bl == b'M'));
        }

        false
    }

    pub fn parse(input: &[u8]) -> Self {
        let width = input.iter().position(|&c| c == b'\n').unwrap();
        let height = input.len() / (width + 1);
        let vec = input
            .iter()
            .filter(|v| **v != b'\n')
            .copied()
            .collect::<Vec<_>>();

        Self {
            grid: Grid::with_storage((width, height), vec),
        }
    }
}

fn travel(pos: (usize, usize), dir: u8, n: usize) -> (usize, usize) {
    let (x, y) = pos;
    match dir {
        0 => (x.wrapping_sub(n), y.wrapping_sub(n)),
        1 => (x, y.wrapping_sub(n)),
        2 => (x.wrapping_add(n), y.wrapping_sub(n)),
        3 => (x.wrapping_sub(n), y),
        4 => (x.wrapping_add(n), y),
        5 => (x.wrapping_sub(n), y.wrapping_add(n)),
        6 => (x, y.wrapping_add(n)),
        7 => (x.wrapping_add(n), y.wrapping_add(n)),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    const EXAMPLE_STAR: &[u8] = b".........
.S..S..S.
..A.A.A..
...MMM...
.SAMXMAS.
...MMM...
..A.A.A..
.S..S..S.
.........
";

    const EXAMPLE_STAR_INVERSE: &[u8] = b".........
.X..X..X.
..M.M.M..
...AAA...
.XMASAMX.
...AAA...
..M.M.M..
.X..X..X.
.........
";

    const EXAMPLE_EDGES: &[u8] = b"XMAS.SAMX
MM.....MM
A.A...A.A
S..S.S..S
.........
S..S.S..S
A.A...A.A
MM.....MM
XMAS.SAMX
";

    const EXAMPLE_EDGES_INVERSE: &[u8] = b"SAMX.XMAS
AA.....AA
M.M...M.M
X..X.X..X
.........
X..X.X..X
M.M...M.M
AA.....AA
SAMX.XMAS
";

    #[test]
    fn part1_works_on_example() {
        assert_eq!(part_1(&XmasGrid::parse(EXAMPLE)), 18);
        assert_eq!(part_1(&XmasGrid::parse(EXAMPLE_STAR)), 8);
        assert_eq!(part_1(&XmasGrid::parse(EXAMPLE_STAR_INVERSE)), 8);
        assert_eq!(part_1(&XmasGrid::parse(EXAMPLE_EDGES)), 12);
        assert_eq!(part_1(&XmasGrid::parse(EXAMPLE_EDGES_INVERSE)), 12);
    }

    #[test]
    fn part2_works_on_example() {
        assert_eq!(part_2(&XmasGrid::parse(EXAMPLE)), 9);
    }
}
