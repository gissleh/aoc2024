use std::fmt::{Debug, Formatter};
use common::grid::Grid;
use common::runner::Runner;

pub fn main(r: &mut Runner, input: &[u8]) {
    let (warehouse, moves) = r.prep("Parse", || Warehouse::parse(input));

    r.part("Part 1", || part_1(&warehouse, &moves));
    r.part("Part 2", || part_2(&warehouse, &moves));

    r.info_debug("Robot Position", &warehouse.robot_pos);
    r.info_debug("Warehouse Size", &warehouse.grid.size());
    r.info_debug("Move Count", &moves.len());
    r.info_debug("Initial Grid", &warehouse);
    r.info_debug("Initial Grid 2", &warehouse.to_wide());
}

fn part_1(warehouse: &Warehouse, moves: &[Move]) -> u32 {
    let mut warehouse = warehouse.clone();
    warehouse.run(moves);
    warehouse.total_gps()
}

fn part_2(warehouse: &Warehouse, moves: &[Move]) -> u32 {
    let mut warehouse = warehouse.to_wide();
    warehouse.run(moves);
    warehouse.total_gps()
}

#[derive(Clone)]
struct Warehouse {
    grid: Grid<(u8, u8), Vec<Cell>, Cell>,
    robot_pos: (u8, u8),
}

impl Warehouse {
    fn push_box(&mut self, box_pos: (u8, u8), m: Move) -> bool{
        let next_pos = m.next_pos(&box_pos);

        match self.grid[next_pos] {
            Cell::Empty => {
                self.grid[next_pos] = Cell::Box;
                self.grid[box_pos] = Cell::Empty;
                true
            },
            Cell::Box => {
                if self.push_box(next_pos, m) {
                    self.grid[next_pos] = Cell::Box;
                    self.grid[box_pos] = Cell::Empty;
                    true
                } else {
                    false
                }
            }
            Cell::Wall => {
                false
            }
        }
    }

    fn run_move(&mut self, m: Move) {
        let next_pos = m.next_pos(&self.robot_pos);

        match self.grid[next_pos] {
            Cell::Empty => { self.robot_pos = next_pos; }
            Cell::Box => {
                if self.push_box(next_pos, m) {
                    self.robot_pos = next_pos;
                }
            }
            Cell::Wall => { }
        }
    }

    fn run(&mut self, moves: &[Move]) {
        for m in moves.iter() {
            self.run_move(*m);
        }
    }

    fn total_gps(&self) -> u32 {
        let mut total = 0;
        for ((x, y), cell) in self.grid.iter() {
            if let Cell::Box = cell {
                total += ((y as u32) * 100) + (x as u32);
            }
        }

        total
    }

    fn to_wide(&self) -> WideWarehouse {
        WideWarehouse::from_warehouse(self)
    }

    fn parse(input: &[u8]) -> (Self, Vec<Move>) {
        let grid_end_index = input
            .array_windows::<2>()
            .position(|v| v == b"\n\n")
            .unwrap();
        let width = input.iter().position(|&c| c == b'\n').unwrap();
        let height = (grid_end_index + 1) / (width + 1);

        let mut robot_pos = (0, 0);
        let mut grid = Grid::new_vec((width as u8, height as u8));
        let mut x = 0;
        let mut y = 0;
        for ch in input[..grid_end_index].iter() {
            match ch {
                b'#' => {
                    grid[(x, y)] = Cell::Wall;
                    x += 1;
                }
                b'.' => {
                    grid[(x, y)] = Cell::Empty;
                    x += 1;
                }
                b'O' => {
                    grid[(x, y)] = Cell::Box;
                    x += 1;
                }
                b'@' => {
                    grid[(x, y)] = Cell::Empty;
                    robot_pos = (x, y);
                    x += 1;
                }
                b'\n' => {
                    x = 0;
                    y += 1;
                }

                _ => unreachable!(),
            }
        }

        let moves = input[grid_end_index..]
            .iter()
            .filter_map(|ch| match *ch {
                b'^' => Some(Move::Up),
                b'<' => Some(Move::Left),
                b'>' => Some(Move::Right),
                b'v' => Some(Move::Down),
                _ => None,
            })
            .collect::<Vec<_>>();

        (Self {
            robot_pos,
            grid,
        }, moves)
    }
}

impl Debug for Warehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "   ")?;
        for n in 0..self.grid.size().0 {
            if n % 10 == 0 {
                write!(f, "{}", (n/10) % 10)?;
            } else {
                write!(f, " ")?;
            }
        }
        writeln!(f)?;
        write!(f, "   ")?;
        for n in 0..self.grid.size().0 {
            write!(f, "{}", n % 10)?;
        }
        writeln!(f)?;

        for ((x, y), cell) in self.grid.iter() {
            if x == 0 && y > 0 {
                writeln!(f)?;
            }

            if x == 0 {
                write!(f, "{y:02} ")?;
            }

            if (x, y) == self.robot_pos {
                write!(f, "@")?;
            } else {
                match cell {
                    Cell::Empty => write!(f, ".")?,
                    Cell::Wall => write!(f, "#")?,
                    Cell::Box => write!(f, "O")?,
                }
            }
        }

        writeln!(f)
    }
}

#[derive(Clone)]
struct WideWarehouse {
    grid: Grid<(u8, u8), Vec<WideCell>, WideCell>,
    robot_pos: (u8, u8),
}

impl WideWarehouse {
    fn box_at(&self, pos: (u8, u8)) -> Option<[(u8, u8); 2]> {
        match self.grid[pos] {
            WideCell::BoxLeft => Some([ pos, (pos.0 + 1, pos.1) ]),
            WideCell::BoxRight => Some([ (pos.0 - 1, pos.1), pos ]),
            _ => None,
        }
    }

    fn push_box(&mut self, box_pos: [(u8, u8); 2], m: Move, dry: bool) -> bool{
        // The space above is clear
        match m.side_of_box(&box_pos) {
            BoxSide::NS(next_pos) => {
                if let WideCell::Empty = self.grid[next_pos[0]] {
                    if let WideCell::Empty = self.grid[next_pos[1]] {
                        if !dry {
                            self.grid[box_pos[0]] = WideCell::Empty;
                            self.grid[box_pos[1]] = WideCell::Empty;
                            self.grid[next_pos[0]] = WideCell::BoxLeft;
                            self.grid[next_pos[1]] = WideCell::BoxRight;
                        }
                        return true;
                    }
                }

                // Stop if either is a wall
                if let WideCell::Wall = self.grid[next_pos[0]] {
                    return false;
                }
                if let WideCell::Wall = self.grid[next_pos[1]] {
                    return false;
                }

                // The above/below space is boxed
                if let Some(box1) = self.box_at(next_pos[0]) {
                    if let Some(box2) = self.box_at(next_pos[1]) {
                        if box1 == box2 {
                            #[cfg(test)]
                            if dry {
                                println!("Indirectly Pushing {box1:?}");
                            }
                            if self.push_box(box1, m, dry) {
                                if !dry {
                                    self.grid[box_pos[0]] = WideCell::Empty;
                                    self.grid[box_pos[1]] = WideCell::Empty;
                                    self.grid[next_pos[0]] = WideCell::BoxLeft;
                                    self.grid[next_pos[1]] = WideCell::BoxRight;
                                }

                                return true
                            }
                        } else {
                            #[cfg(test)]
                            if dry {
                                println!("Indirectly Pushing {box1:?} and {box2:?}");
                            }
                            if self.push_box(box1, m, dry) && self.push_box(box2, m, dry) {
                                if !dry {
                                    self.grid[box_pos[0]] = WideCell::Empty;
                                    self.grid[box_pos[1]] = WideCell::Empty;
                                    self.grid[next_pos[0]] = WideCell::BoxLeft;
                                    self.grid[next_pos[1]] = WideCell::BoxRight;
                                }

                                return true;
                            }
                        }
                    } else {
                        #[cfg(test)]
                        if dry {
                            println!("Indirectly Pushing {box1:?}");
                        }
                        if self.push_box(box1, m, dry) {
                            if !dry {
                                self.grid[box_pos[0]] = WideCell::Empty;
                                self.grid[box_pos[1]] = WideCell::Empty;
                                self.grid[next_pos[0]] = WideCell::BoxLeft;
                                self.grid[next_pos[1]] = WideCell::BoxRight;
                            }

                            return true
                        }
                    }
                } else if let Some(box2) = self.box_at(next_pos[1]) {
                    #[cfg(test)]
                    if dry {
                        println!("Indirectly Pushing {box2:?}");
                    }
                    if self.push_box(box2, m, dry) {
                        if !dry {
                            self.grid[box_pos[0]] = WideCell::Empty;
                            self.grid[box_pos[1]] = WideCell::Empty;
                            self.grid[next_pos[0]] = WideCell::BoxLeft;
                            self.grid[next_pos[1]] = WideCell::BoxRight;
                        }

                        return true
                    }
                }
            }
            BoxSide::WE(check_pos, next_pos) => {
                // The next spot is empty
                if let WideCell::Empty = self.grid[check_pos] {
                    if !dry {
                        self.grid[box_pos[0]] = WideCell::Empty;
                        self.grid[box_pos[1]] = WideCell::Empty;
                        self.grid[next_pos[0]] = WideCell::BoxLeft;
                        self.grid[next_pos[1]] = WideCell::BoxRight;
                    }
                    return true;
                }

                // There is a box there
                if let Some(other_box) = self.box_at(check_pos) {
                    if self.push_box(other_box, m, dry) {
                        if !dry {
                            self.grid[box_pos[0]] = WideCell::Empty;
                            self.grid[box_pos[1]] = WideCell::Empty;
                            self.grid[next_pos[0]] = WideCell::BoxLeft;
                            self.grid[next_pos[1]] = WideCell::BoxRight;
                        }

                        return true;
                    }
                }

                // Failing both of those, there's a wall or the box is immovable
            }
        }

        false
    }

    fn run_move(&mut self, m: Move) {
        let next_pos = m.next_pos(&self.robot_pos);

        match self.grid[next_pos] {
            WideCell::Empty => { self.robot_pos = next_pos; }
            WideCell::BoxLeft => {
                let box_pos = [next_pos, (next_pos.0 + 1, next_pos.1)];
                #[cfg(test)]
                println!("Pushing {box_pos:?}");
                if self.push_box(box_pos, m, true) {
                    self.push_box(box_pos, m, false);
                    self.robot_pos = next_pos;
                }
            }
            WideCell::BoxRight => {
                let box_pos = [(next_pos.0 - 1, next_pos.1), next_pos];
                #[cfg(test)]
                println!("Pushing {box_pos:?}");
                if self.push_box(box_pos, m, true) {
                    self.push_box(box_pos, m, false);
                    self.robot_pos = next_pos;
                }
            }
            WideCell::Wall => { }
        }
    }

    pub fn run(&mut self, moves: &[Move]) {
        for m in moves.iter() {
            #[cfg(test)]
            println!("Move: {m:?}");
            self.run_move(*m);
            #[cfg(test)]
            println!("{:?}", &self);
        }
    }

    fn total_gps(&self) -> u32 {
        let mut total = 0;
        for ((x, y), cell) in self.grid.iter() {
            if let WideCell::BoxLeft = cell {
                total += ((y as u32) * 100) + (x as u32);
            }
        }

        total
    }

    fn from_warehouse(warehouse: &Warehouse) -> Self {
        let (w, h) = warehouse.grid.size();
        let mut grid = Grid::new_vec((w * 2, *h));
        for ((x, y), cell) in warehouse.grid.iter() {
            let x = x * 2;

            match cell {
                Cell::Empty => {
                    grid[(x, y)] = WideCell::Empty;
                    grid[(x + 1, y)] = WideCell::Empty;
                },
                Cell::Wall => {
                    grid[(x, y)] = WideCell::Wall;
                    grid[(x + 1, y)] = WideCell::Wall;
                }
                Cell::Box => {
                    grid[(x, y)] = WideCell::BoxLeft;
                    grid[(x + 1, y)] = WideCell::BoxRight;
                }
            }
        }

        Self{grid, robot_pos: (warehouse.robot_pos.0 * 2, warehouse.robot_pos.1)}
    }
}

impl Debug for WideWarehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "   ")?;
        for n in 0..self.grid.size().0 {
            if n % 10 == 0 {
                write!(f, "{}", (n/10) % 10)?;
            } else {
                write!(f, " ")?;
            }
        }
        writeln!(f)?;
        write!(f, "   ")?;
        for n in 0..self.grid.size().0 {
            write!(f, "{}", n % 10)?;
        }
        writeln!(f)?;

        for ((x, y), cell) in self.grid.iter() {
            if x == 0 && y > 0 {
                writeln!(f)?;
            }

            if x == 0 {
                write!(f, "{y:02} ")?;
            }

            if (x, y) == self.robot_pos {
                write!(f, "@")?;
            } else {
                match cell {
                    WideCell::Empty => write!(f, ".")?,
                    WideCell::Wall => write!(f, "#")?,
                    WideCell::BoxLeft => write!(f, "[")?,
                    WideCell::BoxRight => write!(f, "]")?,
                }
            }
        }

        writeln!(f)
    }
}


#[derive(Eq, PartialEq, Debug, Default, Clone, Copy)]
enum Cell {
    #[default]
    Empty,
    Wall,
    Box,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Copy)]
enum WideCell {
    #[default]
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Move {
    Up,
    Left,
    Right,
    Down,
}

impl Move {
    #[inline]
    fn side_of_box(&self, box_pos: &[(u8, u8); 2]) -> BoxSide {
        match self {
            Move::Up | Move::Down => BoxSide::NS([self.next_pos(&box_pos[0]), self.next_pos(&box_pos[1])]),
            Move::Left => BoxSide::WE((box_pos[0].0 - 1, box_pos[0].1), [
                (box_pos[0].0 - 1, box_pos[0].1),
                (box_pos[0].0, box_pos[0].1),
            ]),
            Move::Right => BoxSide::WE((box_pos[1].0 + 1, box_pos[0].1), [
                (box_pos[1].0, box_pos[1].1),
                (box_pos[1].0 + 1, box_pos[1].1),
            ]),
        }
    }

    #[inline]
    fn next_pos(&self, pos: &(u8, u8)) -> (u8, u8) {
        match self {
            Self::Up => (pos.0, pos.1 - 1),
            Self::Left => (pos.0 - 1, pos.1),
            Self::Right => (pos.0 + 1, pos.1),
            Self::Down => (pos.0, pos.1 + 1),
        }
    }
}

enum BoxSide {
    NS([(u8, u8); 2]),
    WE((u8, u8), [(u8, u8); 2]),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_LARGE: &[u8] = b"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    const EXAMPLE_P2_SMALL: &[u8] = b"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    const EXAMPLE_DIAGONALS: &[u8] = b"#############
#...........#
#......O@...#
#.....O.....#
#...........#
#.....O.....#
#.....O.....#
#...........#
#############

<>vvvv<<<>>>^^<<<<v^^
";

    #[test]
    fn part2_works_on_p2_small_example() {
        let (warehouse, moves) = Warehouse::parse(EXAMPLE_P2_SMALL);
        assert_eq!(part_2(&warehouse, &moves), 618)
    }

    #[test]
    fn part2_works_on_large_example() {
        let (warehouse, moves) = Warehouse::parse(EXAMPLE_LARGE);
        assert_eq!(part_2(&warehouse, &moves), 9021)
    }

    #[test]
    fn part2_works_on_diagonal_example() {
        let (warehouse, moves) = Warehouse::parse(EXAMPLE_DIAGONALS);
        assert_eq!(part_2(&warehouse, &moves), 1648)
    }
}