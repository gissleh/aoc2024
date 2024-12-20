use common::grid::{Grid, GridCoordinate};
use common::point::{CardinalNeighbors, CardinalNeighborsWrapping, ManhattanDistance};
use common::runner::Runner;
use common::search::{bfs, Order};
use rustc_hash::FxHashSet;

const WALL: u16 = u16::MAX;
const OPEN: u16 = 0;

pub fn main(r: &mut Runner, input: &[u8]) {
    let maze = r.prep("Parse", || Maze::parse(input));
    let maze = r.prep("Flood", || maze.with_distances());

    r.part("Part 1", || maze.count_cheats(100));
    r.part("Part 2", || maze.count_cheats_p2(100));
    r.set_tail("Part 1");
    r.part("Part 2 (Manhattan)", || maze.count_cheats_p2_manhattan(100));

    r.info_debug("Maze Size", &maze.grid.size());
    r.info_debug("Maze Start", &maze.start_pos);
    r.info_debug("Maze End", &maze.end_pos);
    r.info_debug(
        "Maze Max Cost",
        &maze
            .grid
            .iter()
            .map(|(_, v)| *v)
            .filter(|v| *v != WALL)
            .max()
            .unwrap(),
    );
}

#[derive(Clone)]
struct Maze {
    grid: Grid<(u8, u8), Vec<u16>, u16>,
    start_pos: (u8, u8),
    end_pos: (u8, u8),
}

impl Maze {
    fn count_cheats(&self, min: u16) -> usize {
        let mut count = 0;
        for (pos, distance) in self.grid.iter() {
            if *distance == WALL {
                continue;
            }

            for (neigh, skip) in pos
                .cardinal_neighbors_n(1)
                .iter()
                .zip(pos.cardinal_neighbors_n(2).iter())
            {
                let neigh_dist = self.grid[*neigh];
                let skip_dist = self.grid[*skip];

                if neigh_dist != WALL || skip_dist > *distance {
                    continue;
                }

                #[cfg(test)]
                println!("{:?}({}) -> {:?}({})", pos, *distance, *skip, skip_dist);

                if *distance - skip_dist > min {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_cheats_p2(&self, min: u16) -> usize {
        let mut count = 0;
        let mut search = bfs().with_seen_space(FxHashSet::default());

        #[cfg(test)]
        println!("Counting cheats min={min}");

        for (pos, distance) in self.grid.iter() {
            if *distance == WALL {
                continue;
            }

            search.reset();
            for neigh in pos.cardinal_neighbors() {
                search.push((neigh, 1));
            }

            count += search.gather::<usize, _, _>(|search, (current_pos, steps)| {
                let curr_dist = self.grid[current_pos];

                if steps < 20 {
                    for neigh in current_pos.cardinal_neighbors_wrapping() {
                        if !neigh.in_bounds(self.grid.size()) {
                            continue;
                        }

                        search.push((neigh, steps + 1));
                    }
                }

                if curr_dist != WALL
                    && curr_dist < *distance
                    && *distance - curr_dist >= (min + steps)
                {
                    #[cfg(test)]
                    println!(
                        "{}   {:?}({}) -> {:?}({})   {}",
                        steps,
                        pos,
                        *distance,
                        current_pos,
                        curr_dist,
                        (*distance - curr_dist - steps)
                    );

                    Some(())
                } else {
                    None
                }
            })
        }

        count
    }

    fn count_cheats_p2_manhattan(&self, min: u16) -> usize {
        let positions: Vec<((u16, u16), u16)> = self
            .grid
            .iter()
            .filter(|(_, v)| **v != WALL)
            .map(|((x, y), v)| ((x as u16, y as u16), *v))
            .collect();

        let mut count = 0;

        for (ap, ad) in positions.iter() {
            for (bp, bd) in positions.iter() {
                if *bd >= *ad || ap.1 > bp.1 + 20 {
                    continue;
                }
                if bp.1 > ap.1 + 20 {
                    break;
                }

                let cheat_gain = *ad - *bd;
                let cheat_distance = ap.manhattan_distance_to(bp);
                if cheat_distance <= 20 && cheat_gain - cheat_distance >= min {
                    count += 1;
                }
            }
        }

        count
    }

    fn with_distances(&self) -> Self {
        let mut search = bfs().with_seen_space(FxHashSet::default());
        search.push((self.end_pos, 0));

        let new_grid = search.fold(
            self.grid.clone(),
            |search, (pos, cost)| {
                if self.grid[pos] == WALL {
                    return None;
                }

                for neigh in pos.cardinal_neighbors() {
                    search.push((neigh, cost + 1));
                }

                Some((pos, cost))
            },
            |mut grid, (pos, cost)| {
                grid[pos] = cost;
                grid
            },
        );

        Self {
            grid: new_grid,
            start_pos: self.start_pos,
            end_pos: self.end_pos,
        }
    }

    fn parse(input: &[u8]) -> Self {
        let width = input.iter().position(|&c| c == b'\n').unwrap();
        let height = input.len() / (width + 1);
        let mut grid = Grid::new_vec_initial((width as u8 + 2, height as u8 + 2), WALL);

        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);

        let mut x = 1;
        let mut y = 1;
        for ch in input.iter() {
            match *ch {
                b'#' => {
                    grid[(x, y)] = WALL;
                    x += 1;
                }
                b'.' => {
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
                    x = 1;
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(Maze::parse(EXAMPLE).with_distances().count_cheats(10), 10)
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(Maze::parse(EXAMPLE).with_distances().count_cheats_p2(76), 3);
        assert_eq!(
            Maze::parse(EXAMPLE).with_distances().count_cheats_p2(74),
            4 + 3
        );
        assert_eq!(
            Maze::parse(EXAMPLE).with_distances().count_cheats_p2(72),
            22 + 4 + 3
        );
        assert_eq!(
            Maze::parse(EXAMPLE).with_distances().count_cheats_p2(70),
            12 + 22 + 4 + 3
        );
        assert_eq!(
            Maze::parse(EXAMPLE).with_distances().count_cheats_p2(50),
            285
        );
    }

    #[test]
    fn part_2_manhattan_works_on_example() {
        assert_eq!(
            Maze::parse(EXAMPLE)
                .with_distances()
                .count_cheats_p2_manhattan(76),
            3
        );
        assert_eq!(
            Maze::parse(EXAMPLE)
                .with_distances()
                .count_cheats_p2_manhattan(74),
            4 + 3
        );
        assert_eq!(
            Maze::parse(EXAMPLE)
                .with_distances()
                .count_cheats_p2_manhattan(72),
            22 + 4 + 3
        );
        assert_eq!(
            Maze::parse(EXAMPLE)
                .with_distances()
                .count_cheats_p2_manhattan(70),
            12 + 22 + 4 + 3
        );
        assert_eq!(
            Maze::parse(EXAMPLE)
                .with_distances()
                .count_cheats_p2_manhattan(50),
            285
        );
    }
}
