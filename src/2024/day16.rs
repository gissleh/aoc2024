use arrayvec::ArrayVec;
use common::graph::Graph;
use common::grid::Grid;
use common::point::CardinalNeighbors;
use common::runner::{BothParts, Runner};
use common::search::{dfs, dijkstra, Cost, Key, Order, ReEntrantSeenMap};
use common::utils::CardinalDirection;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::minmax;

const WALL: u8 = b'#';
const OPEN: u8 = b'.';
const JUNCTION: u8 = b'X';

pub fn main(r: &mut Runner, input: &[u8]) {
    let maze = r.prep("Parse", || Maze::parse(input));
    r.part("Both Parts", || both_parts(&maze));

    r.set_tail("Parse");
    let graph = r.prep("Build Graph", || build_graph(&maze));
    r.part("Part 1 (Graph)", || part_1_graph(&maze, &graph));
    r.part("Part 2 (Graph)", || part_2_graph(&maze, &graph));

    r.info_debug("Maze Size", maze.grid.size());
    r.info_debug("Start Pos", &maze.start_pos);
    r.info_debug("End Pos", &maze.end_pos);
    r.info_debug("Graph Nodes", &graph.nodes().len());
}

fn both_parts(maze: &Maze) -> BothParts<u32, u32> {
    let mut search = dijkstra().with_seen_space(ReEntrantSeenMap::with_capacity(1024));

    search.push(Reindeer {
        position: maze.start_pos,
        direction: CardinalDirection::East,
        score: 0,
        turn_positions: [(0, 0); 192],
        turn_positions_len: 0,
    });

    let mut best_score = 0;
    let best_score = &mut best_score;

    let scores_and_turns =
        search.gather::<Vec<_>, _, (u32, ArrayVec<(u8, u8), 192>)>(move |search, reindeer| {
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
        FxHashSet::with_capacity_and_hasher(192, Default::default());
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

type MazeGraph = Graph<(u8, u8), (u32, CardinalDirection, CardinalDirection), 4>;

fn build_graph(maze: &Maze) -> MazeGraph {
    let mut search = dfs().without_seen_space();
    let mut grid = maze.grid.clone();
    let mut junctions = Vec::with_capacity(64);
    let (w, h) = *grid.size();

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if (x, y) == maze.start_pos || (x, y) == maze.end_pos {
                grid[(x, y)] = JUNCTION;
                junctions.push((x, y));
                continue;
            }

            let c = grid[(x, y)];
            if c == WALL {
                continue;
            }

            let open_count = (x, y)
                .cardinal_neighbors()
                .iter()
                .filter(|n| grid[**n] != WALL)
                .count();
            if open_count > 2 {
                grid[(x, y)] = JUNCTION;
                junctions.push((x, y));
            }
        }
    }

    let mut graph = MazeGraph::with_capacity(junctions.len());
    for (x, y) in junctions {
        let a = graph.ensure_node((x, y));

        for init_dir in CardinalDirection::NWES {
            search.reset();
            search.push(((x, y), init_dir, 0u32));
            let first_next_pos = init_dir.next_pos(&(x, y));
            if grid[first_next_pos] == WALL {
                continue;
            }

            let res = search.find(|search, (pos, dir, score)| {
                let next_pos = dir.next_pos(&pos);
                let cell = grid[next_pos];

                match cell {
                    JUNCTION => Some((next_pos, dir, score + 1)),
                    OPEN => {
                        search.push((next_pos, dir, score + 1));
                        None
                    }
                    WALL => {
                        let left_dir = dir.turn_anticlockwise();
                        let next_pos_left = left_dir.next_pos(&pos);
                        match grid[next_pos_left] {
                            JUNCTION => Some((next_pos_left, left_dir, score + 1001)),
                            WALL => {
                                let right_dir = dir.turn_clockwise();
                                let next_pos_right = right_dir.next_pos(&pos);
                                match grid[next_pos_right] {
                                    JUNCTION => Some((next_pos_right, right_dir, score + 1001)),
                                    WALL => None,
                                    OPEN => {
                                        search.push((next_pos_right, right_dir, score + 1001));
                                        None
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            OPEN => {
                                search.push((next_pos_left, left_dir, score + 1001));
                                None
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            });

            if let Some((pos, new_dir, score)) = res {
                let b = graph.ensure_node(pos);
                graph.connect(a, b, (score, init_dir, new_dir));

                #[cfg(test)]
                println!("({x}, {y}) --{init_dir:?}--{new_dir:?}--> {pos:?}: {score}");
            }
        }
    }

    graph
}

fn part_1_graph(maze: &Maze, graph: &MazeGraph) -> u32 {
    let mut search =
        dijkstra().with_seen_space(FxHashMap::with_capacity_and_hasher(64, Default::default()));
    search.push(GraphedReindeerP1 {
        index: graph.node_index(&maze.start_pos).unwrap(),
        direction: CardinalDirection::East,
        score: 0,
    });

    let end_index = graph.node_index(&maze.end_pos).unwrap();

    search
        .find(
            |search,
             GraphedReindeerP1 {
                 index,
                 direction,
                 score,
             }| {
                if index == end_index {
                    return Some(score);
                }

                let dir_back = direction.turn_around();

                for (next_index, (cost, exit_dir, enter_dir)) in graph.edges(index) {
                    if *exit_dir == direction {
                        search.push(GraphedReindeerP1 {
                            index: *next_index,
                            direction: *enter_dir,
                            score: score + *cost,
                        });
                    } else if *exit_dir != dir_back {
                        search.push(GraphedReindeerP1 {
                            index: *next_index,
                            direction: *enter_dir,
                            score: score + 1000 + *cost,
                        });
                    }
                }

                None
            },
        )
        .unwrap()
}

#[derive(Clone, Copy, Debug)]
struct GraphedReindeerP1 {
    index: usize,
    direction: CardinalDirection,
    score: u32,
}

impl Cost<u32> for GraphedReindeerP1 {
    fn cost(&self) -> u32 {
        self.score
    }
}

impl Key<(usize, CardinalDirection)> for GraphedReindeerP1 {
    fn key(&self) -> (usize, CardinalDirection) {
        (self.index, self.direction)
    }
}

fn part_2_graph(maze: &Maze, graph: &MazeGraph) -> u32 {
    let mut search = dijkstra().with_seen_space(ReEntrantSeenMap::with_capacity(64));
    let start_index = graph.node_index(&maze.start_pos).unwrap();
    let end_index = graph.node_index(&maze.end_pos).unwrap();

    search.push(GraphedReindeerP2 {
        index: start_index,
        direction: CardinalDirection::East,
        score: 0,
        traced_path: [(0u16, 0u16); 192],
        traced_path_len: 0,
    });

    let mut max_score = 0;

    let map = search.fold(
        FxHashMap::<(u16, u16), u32>::with_capacity_and_hasher(128, Default::default()),
        |search,
         GraphedReindeerP2 {
             index,
             direction,
             score,
             traced_path,
             traced_path_len,
         }| {
            if max_score > 0 && score > max_score {
                return None;
            }

            #[cfg(test)]
            println!("At {:?} with {score}", graph.node(index));

            if index == end_index {
                max_score = score;
                return Some((traced_path, traced_path_len));
            }

            let dir_back = direction.turn_around();

            for (next_index, (cost, exit_dir, enter_dir)) in graph.edges(index) {
                if *exit_dir == dir_back {
                    continue;
                }

                let next_score = if direction != *exit_dir {
                    1000 + score + cost
                } else {
                    score + cost
                };

                let mut next_traced_path = traced_path;
                next_traced_path[traced_path_len as usize] =
                    (*next_index as u16, ((cost % 1000) - 1) as u16);

                search.push(GraphedReindeerP2 {
                    index: *next_index,
                    direction: *enter_dir,
                    score: next_score,
                    traced_path: next_traced_path,
                    traced_path_len: traced_path_len + 1,
                });
            }

            None
        },
        |mut map, (res, res_len)| {
            #[cfg(test)]
            println!("Path:");
            let mut prev = start_index as u16;
            for (index, score) in &res[..res_len as usize] {
                #[cfg(test)]
                println!(
                    "{:?}->{:?}",
                    graph.node(prev as usize),
                    graph.node(*index as usize)
                );

                map.insert((prev, *index), *score as u32);
                prev = *index;
            }

            map
        },
    );

    let mut unique_points = FxHashSet::with_capacity_and_hasher(map.len(), Default::default());
    for (a, b) in map.keys() {
        unique_points.insert(*a);
        unique_points.insert(*b);
    }

    unique_points.len() as u32 + map.values().sum::<u32>()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct GraphedReindeerP2 {
    index: usize,
    direction: CardinalDirection,
    score: u32,
    traced_path: [(u16, u16); 192],
    traced_path_len: u32,
}

impl Cost<u32> for GraphedReindeerP2 {
    fn cost(&self) -> u32 {
        self.score
    }
}

impl Key<(usize, CardinalDirection)> for GraphedReindeerP2 {
    fn key(&self) -> (usize, CardinalDirection) {
        (self.index, self.direction)
    }
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
    turn_positions: [(u8, u8); 192],
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
    fn part_1_graph_works_on_example_1() {
        let maze = Maze::parse(EXAMPLE_1);
        let graph = build_graph(&maze);

        assert_eq!(part_1_graph(&maze, &graph), 7036);
    }

    #[test]
    fn part_1_graph_works_on_example_2() {
        let maze = Maze::parse(EXAMPLE_2);
        let graph = build_graph(&maze);

        assert_eq!(part_1_graph(&maze, &graph), 11048);
    }

    #[test]
    fn part_2_graph_works_on_example_1() {
        let maze = Maze::parse(EXAMPLE_1);
        let graph = build_graph(&maze);

        assert_eq!(part_2_graph(&maze, &graph), 45);
    }

    #[test]
    fn part_2_graph_works_on_example_2() {
        let maze = Maze::parse(EXAMPLE_2);
        let graph = build_graph(&maze);

        assert_eq!(part_2_graph(&maze, &graph), 64);
    }

    #[test]
    fn both_parts_works_on_example_2() {
        let BothParts(p1_res, p2_res) = both_parts(&Maze::parse(EXAMPLE_2));
        assert_eq!(p1_res, 11048);
        assert_eq!(p2_res, 64);
    }
}
