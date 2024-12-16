use arrayvec::ArrayVec;
use common::grid::Grid;
use common::runner::{BothParts, Runner};
use common::search::{dijkstra, Cost, Key, Order, ReEntrantSeenMap};
use common::utils::CardinalDirection;
use rustc_hash::FxHashSet;
use std::cmp::minmax;

const WALL: u8 = b'#';
const OPEN: u8 = b'.';

pub fn main(r: &mut Runner, input: &[u8]) {
    let maze = r.prep("Parse", || Maze::parse(input));

    r.part("Both Parts", || both_parts(&maze));

    r.info_debug("Maze Size", maze.grid.size());
    r.info_debug("Start Pos", &maze.start_pos);
    r.info_debug("End Pos", &maze.end_pos);
}

fn both_parts(maze: &Maze) -> BothParts<u32, u32> {
    let mut search = dijkstra().with_seen_space(ReEntrantSeenMap::with_capacity(1024));

    search.push(Reindeer {
        position: maze.start_pos,
        direction: CardinalDirection::East,
        score: 0,
        turn_positions: [(0, 0); 128],
        turn_positions_len: 0,
    });

    let mut best_score = 0;
    let best_score = &mut best_score;

    let scores_and_turns =
        search.gather::<Vec<_>, _, (u32, ArrayVec<(u8, u8), 128>)>(move |search, reindeer| {
            if *best_score > 0 && reindeer.score > *best_score {
                return None;
            }

            if reindeer.position == maze.end_pos {
                let mut next_traced_path = reindeer.turn_positions;
                next_traced_path[reindeer.turn_positions_len as usize] = reindeer.position;

                *best_score = reindeer.score;

                return Some((
                    reindeer.score,
                    ArrayVec::from_iter(
                        next_traced_path[..(reindeer.turn_positions_len + 1) as usize]
                            .iter()
                            .copied(),
                    ),
                ));
            }

            let next_pos = reindeer.direction.next_pos(&reindeer.position);
            if maze.grid[next_pos] == OPEN {
                search.push(Reindeer {
                    position: next_pos,
                    direction: reindeer.direction,
                    score: reindeer.score + 1,
                    turn_positions: reindeer.turn_positions,
                    turn_positions_len: reindeer.turn_positions_len,
                });
            }

            for next_direction in [
                reindeer.direction.turn_clockwise(),
                reindeer.direction.turn_anticlockwise(),
            ] {
                let next_pos = next_direction.next_pos(&reindeer.position);
                if maze.grid[next_pos] == OPEN {
                    let mut next_traced_path = reindeer.turn_positions;
                    next_traced_path[reindeer.turn_positions_len as usize] = reindeer.position;

                    search.push(Reindeer {
                        position: next_pos,
                        direction: next_direction,
                        score: reindeer.score + 1001,
                        turn_positions: next_traced_path,
                        turn_positions_len: reindeer.turn_positions_len + 1,
                    });
                }
            }

            None
        });

    let mut seen_tiles: FxHashSet<(u8, u8)> =
        FxHashSet::with_capacity_and_hasher(128, Default::default());
    for (_, path) in scores_and_turns.iter() {
        let mut prev = maze.start_pos;
        for pos in path.iter() {
            let [min_x, max_x] = minmax(prev.0, pos.0);
            let [min_y, max_y] = minmax(prev.1, pos.1);

            if min_x == max_x {
                for y in min_y..=max_y {
                    seen_tiles.insert((min_x, y));
                }
            } else {
                for x in min_x..=max_x {
                    seen_tiles.insert((x, min_y));
                }
            }

            prev = *pos;
        }
    }

    #[cfg(test)]
    for tile in seen_tiles.iter() {
        println!("{tile:?}");
    }

    BothParts(scores_and_turns[0].0, seen_tiles.len() as u32)
}

struct Maze {
    grid: Grid<(u8, u8), Vec<u8>, u8>,
    start_pos: (u8, u8),
    end_pos: (u8, u8),
}

impl Maze {
    fn parse(input: &[u8]) -> Self {
        let width = input.iter().position(|&c| c == b'\n').unwrap();
        let height = input.len() / (width + 1);
        let mut grid = Grid::new_vec((width as u8, height as u8));

        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);

        let mut x = 0;
        let mut y = 0;
        for ch in input.iter() {
            match *ch {
                WALL => {
                    grid[(x, y)] = WALL;
                    x += 1;
                }
                OPEN => {
                    grid[(x, y)] = OPEN;
                    x += 1;
                }
                b'S' => {
                    grid[(x, y)] = OPEN;
                    start_pos = (x, y);
                    x += 1;
                }
                b'E' => {
                    grid[(x, y)] = OPEN;
                    end_pos = (x, y);
                    x += 1;
                }
                b'\n' => {
                    x = 0;
                    y += 1;
                }

                _ => unreachable!(),
            }
        }

        Self {
            grid,
            start_pos,
            end_pos,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Reindeer {
    position: (u8, u8),
    direction: CardinalDirection,
    score: u32,
    turn_positions: [(u8, u8); 128],
    turn_positions_len: u8,
}

impl Cost<u32> for Reindeer {
    fn cost(&self) -> u32 {
        self.score
    }
}

impl Key<(u8, u8, CardinalDirection)> for Reindeer {
    fn key(&self) -> (u8, u8, CardinalDirection) {
        (self.position.0, self.position.1, self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &[u8] = b"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const EXAMPLE_2: &[u8] = b"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    fn both_parts_works_on_example_1() {
        let BothParts(p1_res, p2_res) = both_parts(&Maze::parse(EXAMPLE_1));
        assert_eq!(p1_res, 7036);
        assert_eq!(p2_res, 45);
    }

    #[test]
    fn both_parts_works_on_example_2() {
        let BothParts(p1_res, p2_res) = both_parts(&Maze::parse(EXAMPLE_2));
        assert_eq!(p1_res, 11048);
        assert_eq!(p2_res, 64);
    }
}
