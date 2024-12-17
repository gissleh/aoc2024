use arrayvec::ArrayVec;
use common::graph::Graph;
use common::grid::Grid;
use common::runner::{BothParts, Runner};
use common::search::{bfs, dijkstra, Cost, Key, Order, ReEntrantSeenMap};
use common::utils::CardinalDirection;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::minmax;
use std::collections::HashSet;

const WALL: u8 = b'#';
const OPEN: u8 = b'.';

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

type MazeGraph = Graph<((u8, u8), CardinalDirection), u32, 6>;

fn build_graph(maze: &Maze) -> MazeGraph {
    let mut search = dijkstra().with_seen_space(FxHashMap::with_capacity_and_hasher(
        1024,
        Default::default(),
    ));
    search.push(GraphBuildStep {
        pos: maze.start_pos,
        last_intersection: maze.start_pos,
        last_direction: CardinalDirection::East,
        direction: CardinalDirection::East,
        score: 0,
        last_score: 0,
    });

    let mut max_score = 0;

    search.fold(
        MazeGraph::with_capacity(192),
        |search,
         GraphBuildStep {
             pos,
             last_intersection,
             last_direction,
             direction,
             score,
             last_score,
         }| {
            if max_score != 0 && score > max_score {
                return None;
            }

            if pos == maze.end_pos {
                max_score = score;

                return Some((
                    last_intersection,
                    pos,
                    last_direction,
                    direction,
                    score - last_score,
                ));
            }

            let directions = [
                direction,
                direction.turn_clockwise(),
                direction.turn_anticlockwise(),
            ];

            let open = directions.map(|dir| maze.grid[dir.next_pos(&pos)] == OPEN);
            let open_count = open.iter().filter(|v| **v).count();

            let (last_score, last_intersection) = if open_count > 1 {
                (score,
                pos)
            } else {
                (last_score,
                last_intersection)
            };

            for (i, next_direction) in directions.iter().enumerate() {
                let next_pos = next_direction.next_pos(&pos);
                if open[i] {
                    let next_score = if *next_direction != direction {
                        score + 1001
                    } else {
                        score + 1
                    };

                    //search.push((next_pos, last_intersection, next_score, next_score, last_score));
                    search.push(GraphBuildStep {
                        pos: next_pos,
                        direction: *next_direction,
                        score: next_score,
                        last_score,
                        last_intersection,
                        last_direction: if open_count > 1 { *next_direction } else { last_direction },
                    });
                }
            }

            // If it's about to reach a split
            let next_pos = direction.next_pos(&pos);
            if score != last_score && maze.grid[next_pos] == OPEN {
                let open = directions.map(|dir| maze.grid[dir.next_pos(&next_pos)] == OPEN);
                if open.iter().filter(|o| **o).count() > 1 {
                    if score == last_score {
                        return None;
                    }

                    return Some((
                        last_intersection,
                        next_pos,
                        last_direction,
                        direction,
                        score - last_score,
                    ));
                }
            }

            None
        },
        |mut graph, (pos_a, pos_b, dir_a, dir_b, cost)| {
            #[cfg(test)]
            println!("{pos_a:?}({dir_a:?}) -> {pos_b:?}({dir_b:?}) {cost}");

            let a = graph.ensure_node((pos_a, dir_a));
            let b = graph.ensure_node((pos_b, dir_b));

            if graph.edge(a, b).is_none() {
                graph.connect(a, b, cost);
            }

            if graph.edges(a).len() == 1 {
                let al = graph.ensure_node((pos_a, dir_a.turn_anticlockwise()));
                let ar = graph.ensure_node((pos_a, dir_a.turn_clockwise()));
                graph.connect(a, al, 1000);
                graph.connect(a, ar, 1000);
            }

            if graph.edges(b).len() == 0 {
                let bl = graph.ensure_node((pos_b, dir_b.turn_anticlockwise()));
                let br = graph.ensure_node((pos_b, dir_b.turn_clockwise()));
                graph.connect(b, bl, 1000);
                graph.connect(b, br, 1000);
            }

            graph
        },
    )
}

fn part_1_graph(maze: &Maze, graph: &MazeGraph) -> u32 {
    let mut search = dijkstra().with_seen_space(vec![0u32; graph.len()]);
    search.push((
        graph
            .node_index(&(maze.start_pos, CardinalDirection::East))
            .unwrap(),
        1,
    ));

    search
        .find(|search, (index, score)| {
            if graph.node(index).0 == maze.end_pos {
                return Some(score);
            }

            for (next_index, cost) in graph.edges(index) {
                search.push((*next_index, *cost + score));
            }

            None
        })
        .unwrap() - 1
}

fn part_2_graph(maze: &Maze, graph: &MazeGraph) -> u32 {
    let mut search = dijkstra().with_seen_space(ReEntrantSeenMap::with_capacity(256));
    search.push(GraphedReindeer{
        index: graph.node_index(&(maze.start_pos, CardinalDirection::East)).unwrap(),
        traced_path: [0; 192],
        traced_path_len: 0,
        score: 1,
    });
    let start_index = graph.node_index(&(maze.start_pos, CardinalDirection::East)).unwrap() as u16;

    let mut best_score = 0;

    search.fold(HashSet::with_capacity(64), move |search, r| {
        if best_score != 0 && r.score > best_score {
            return None;
        }

        if graph.node(r.index).0 == maze.end_pos {
            best_score = r.score;

            return Some(ArrayVec::from_iter(r.traced_path[..r.traced_path_len as usize]
                .iter()
                .copied()));
        }

        for (next_index, cost) in graph.edges(r.index) {
            let mut next_traced_path = r.traced_path;
            let mut next_traced_path_len = r.traced_path_len;
            next_traced_path[r.traced_path_len as usize] = *next_index as u16;
            next_traced_path_len += 1;

            search.push(GraphedReindeer{
                index: *next_index,
                score: r.score + *cost,
                traced_path: next_traced_path,
                traced_path_len: next_traced_path_len,
            });
        }

        None
    }, |mut seen_segments, path: ArrayVec<_, 192>| {
        let mut prev = start_index;
        #[cfg(test)]
        println!("{path:?}");

        for pos in path.iter() {
            #[cfg(test)]
            println!("{prev} -> {pos}: {}", *graph.edge(prev as usize, *pos as usize).unwrap() % 1000);

            seen_segments.insert((prev, *pos));
            prev = *pos;
        }

        seen_segments
    }).iter().map(|(a, b)| *graph.edge(*a as usize, *b as usize).unwrap() % 1000).sum::<u32>()
}

#[derive(Clone, Copy)]
struct GraphBuildStep {
    pos: (u8, u8),
    last_intersection: (u8, u8),
    last_direction: CardinalDirection,
    direction: CardinalDirection,
    score: u32,
    last_score: u32,
}

impl Key<(u8, u8, CardinalDirection)> for GraphBuildStep {
    fn key(&self) -> (u8, u8, CardinalDirection) {
        (self.pos.0, self.pos.1, self.direction)
    }
}

impl Cost<u32> for GraphBuildStep {
    fn cost(&self) -> u32 {
        self.score
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

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct GraphedReindeer {
    index: usize,
    score: u32,
    traced_path: [u16; 192],
    traced_path_len: u32,
}

impl Cost<u32> for GraphedReindeer {
    fn cost(&self) -> u32 {
        self.score
    }
}

impl Key<usize> for GraphedReindeer {
    fn key(&self) -> usize {
        self.index
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
    fn build_graph_works_on_example_1() {
        let maze = Maze::parse(EXAMPLE_1);
        let graph = build_graph(&maze);

        println!("{:?}", graph.edges(0));

        assert_eq!(
            graph.edge(
                graph
                    .node_index(&((9, 7), CardinalDirection::East))
                    .unwrap(),
                graph
                    .node_index(&((9, 7), CardinalDirection::North))
                    .unwrap(),
            ),
            Some(&1000)
        );
        assert_eq!(
            graph.edge(
                graph
                    .node_index(&((9, 7), CardinalDirection::North))
                    .unwrap(),
                graph
                    .node_index(&((9, 5), CardinalDirection::North))
                    .unwrap(),
            ),
            Some(&2)
        );
        assert_eq!(
            graph.edge(
                graph
                    .node_index(&((9, 5), CardinalDirection::North))
                    .unwrap(),
                graph
                    .node_index(&((9, 5), CardinalDirection::East))
                    .unwrap(),
            ),
            Some(&1000)
        );
        assert_eq!(
            graph.edge(
                graph
                    .node_index(&((9, 5), CardinalDirection::East))
                    .unwrap(),
                graph
                    .node_index(&((13, 1), CardinalDirection::East))
                    .unwrap(),
            ),
            Some(&4012)
        );
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
