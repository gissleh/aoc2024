use common::grid::{Grid, GridCoordinate};
use common::runner::{BothParts, Runner};

pub fn main(r: &mut Runner, input: &[u8]) {
    let (grid, start_pos) = r.prep("Parse", || parse_grid(input));

    r.part("Both Parts", || both_parts(&grid, start_pos));

    r.info("Grid Size", &format!("{:?}", grid.size()));
    r.info("Start Position", &format!("{:?}", start_pos));
}

fn both_parts(grid: &Grid<(u16, u16), Vec<u8>, u8>, start_pos: (u16, u16)) -> BothParts<u32, u32> {
    let mut seen = [0u8; 256*256];
    let mut pos = start_pos;
    let mut dir = Direction::Up;
    let mut count = 0;
    let mut obstacles = 0;
    let size = grid.size();

    let mut next_pos = dir.next_pos(&pos);
    while let Some(cell) = grid.cell(&next_pos) {
        let index = pos.index(size);
        if seen[index] & dir.to_bits() == 0 {
            if seen[index] == 0 {
                count += 1;
            }

            seen[index] |= dir.to_bits();
        }

        match cell {
            b'.' => {
                let index = pos.index(size);
                let next_index = next_pos.index(size);

                if seen[index] & dir.turn_right().to_bits() == 99 {
                    #[cfg(test)]
                    println!("Obstacle at {:?} going {:?} (Cross)", dir.next_pos(&pos), dir);
                    obstacles += 1;
                } else if seen[next_index] == 0 {
                    let obstacle_pos = next_pos;
                    if grid.cell(&obstacle_pos) != Some(&b'#') {
                        let mut seen2 = seen;

                        let mut dir2 = dir.turn_right();
                        let mut pos2 = pos;
                        let mut next_pos2 = dir2.next_pos(&pos2);
                        'pos2_loop: while let Some(cell) = grid.cell(&next_pos2) {
                            let index = pos2.index(size);
                            seen2[index] |= dir2.to_bits();

                            match cell {
                                b'.' if next_pos2 == obstacle_pos => {
                                    dir2 = dir2.turn_right();
                                },
                                b'.' => {
                                    if seen2[next_pos2.index(&size)] & dir2.to_bits() != 0 {
                                        #[cfg(test)]
                                        println!("Obstacle at {:?} going {:?} (New Loop)",obstacle_pos, dir);

                                        obstacles += 1;
                                        break 'pos2_loop;
                                    }

                                    pos2 = next_pos2;
                                }
                                b'#' => {
                                    dir2 = dir2.turn_right();
                                }
                                _ => unreachable!()
                            }

                            next_pos2 = dir2.next_pos(&pos2);
                        }
                    }
                }

                pos = next_pos;
            }
            b'#' => {
                dir = dir.turn_right();
            }
            _ => {
                return BothParts(count, obstacles);
            }
        }

        next_pos = dir.next_pos(&pos);
    }

    // If it hasn't visited the exit before.
    if seen[pos.index(size)] == 0 {
        count += 1;
    }

    BothParts(count, obstacles)
}

fn parse_grid(input: &[u8]) -> (Grid<(u16, u16), Vec<u8>, u8>, (u16, u16)) {
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let height = input.len() / (width + 1);
    let vec = input
        .iter()
        .filter(|v| **v != b'\n')
        .copied()
        .collect::<Vec<_>>();

    let mut grid = Grid::new_with_default((width as u16, height as u16), vec, 0);
    let start = grid.iter()
        .find(|(_, v)| **v == b'^')
        .map(|(pos, _)| pos)
        .unwrap();

    *grid.cell_mut(&start).unwrap() = b'.';

    (grid, start)
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Left, Up, Right, Down
}

impl Direction {
    fn to_bits(&self) -> u8 {
        match *self {
            Direction::Left => 0b0001,
            Direction::Up => 0b0010,
            Direction::Right => 0b0100,
            Direction::Down => 0b1000,
        }
    }

    fn next_pos(&self, pos: &(u16, u16)) -> (u16, u16) {
        match self {
            Direction::Left => (pos.0.wrapping_sub(1), pos.1),
            Direction::Up => (pos.0, pos.1.wrapping_sub(1)),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Down => (pos.0, pos.1 + 1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use common::runner::BothParts;
    use crate::day06::{parse_grid, both_parts};

    const EXAMPLE: &[u8] = b"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    const EXAMPLE_2: &[u8] = b".....
..#..
...#.
..^..
..#..
";


    #[test]
    fn part_1_works_on_example() {
        let (grid, start_pos) = parse_grid(EXAMPLE);
        assert_eq!(both_parts(&grid, start_pos), BothParts(41, 6));
    }

    #[test]
    fn part_1_works_on_corner_case() {
        let (grid, start_pos) = parse_grid(EXAMPLE_2);
        assert_eq!(both_parts(&grid, start_pos), BothParts(4, 1));
    }
}