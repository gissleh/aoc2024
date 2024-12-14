use common::parser;
use common::parser::Parser;
use common::runner::Runner;

pub fn main(r: &mut Runner, input: &[u8]) {
    let robots = r.prep("Parse", || Robot::parse_list(input));

    r.part("Part 1", || part_1::<101, 103>(&robots));
    r.part("Part 2", || part_2::<101, 103>(&robots));

    r.info("Robots", &robots.len());
}

fn part_1<const W: i32, const H: i32>(robots: &[Robot]) -> u32 {
    let mut robots = robots.iter().copied().collect::<Vec<_>>();

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.run_move::<W, H>()
        }
    }

    let mut quadrants = [0u32; 4];
    for robot in robots.iter() {
        let mut qi = 0usize;
        let (px, py) = robot.p;
        if px == (W / 2) || py == (H / 2) {
            continue;
        }

        if px > (W / 2) {
            qi |= 1;
        }
        if py > (H / 2) {
            qi |= 2;
        }

        quadrants[qi] += 1;
    }

    quadrants.iter().product::<u32>()
}

fn part_2<const W: i32, const H: i32>(robots: &[Robot]) -> u32 {
    let mut robots = robots.iter().copied().collect::<Vec<_>>();
    let mut grid = vec![0u128; H as usize];

    for seconds in 1.. {
        grid.fill(0);
        for robot in robots.iter_mut() {
            robot.run_move::<W, H>();
            grid[robot.p.1 as usize] |= 1 << robot.p.0;
        }

        // Not proud of this one, but assume any 16-length line of drones mean the pattern is found
        for i in 0..grid.len() {
            for j in 0..112 {
                let m = 0b1111111111111111 << j;
                if grid[i] & m == m {
                    return seconds;
                }
            }
        }
    }

    0
}

#[derive(Copy, Clone, Debug)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    fn run_move<const W: i32, const H: i32>(&mut self) {
        let (px, py) = &mut self.p;
        let (vx, vy) = self.v;

        *px = add_velocity::<W>(*px, vx);
        *py = add_velocity::<H>(*py, vy);
    }

    fn parse_list(input: &[u8]) -> Vec<Self> {
        Self::parser()
            .delimited_by(b'\n')
            .repeat()
            .parse_value(input)
            .unwrap()
    }

    fn parser<'i>() -> impl Parser<'i, Self> {
        b"p="
            .and_instead(
                parser::int::<i32>()
                    .and_discard(b',')
                    .and(parser::int::<i32>()),
            )
            .and_discard(b" v=")
            .and(
                parser::int::<i32>()
                    .and_discard(b',')
                    .and(parser::int::<i32>()),
            )
            .map(|(p, v)| Robot { p, v })
    }
}

#[inline]
fn add_velocity<const W: i32>(p: i32, v: i32) -> i32 {
    let p = p + v;
    if p < 0 {
        p + W
    } else {
        p % W
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn robot_moves_correctly() {
        let mut robot = Robot::parser().parse_value(b"p=2,4 v=2,-3").unwrap();
        assert_eq!(robot.p, (2, 4));
        assert_eq!(robot.v, (2, -3));

        robot.run_move::<11, 7>();
        assert_eq!(robot.p, (4, 1));

        robot.run_move::<11, 7>();
        assert_eq!(robot.p, (6, 5));

        robot.run_move::<11, 7>();
        assert_eq!(robot.p, (8, 2));

        robot.run_move::<11, 7>();
        assert_eq!(robot.p, (10, 6));

        robot.run_move::<11, 7>();
        assert_eq!(robot.p, (1, 3));

        let mut robot = Robot::parser().parse_value(b"p=3,3 v=-3,-3").unwrap();
        assert_eq!(robot.p, (3, 3));
        assert_eq!(robot.v, (-3, -3));

        robot.run_move::<11, 7>();
        assert_eq!(robot.p, (0, 0));

        robot.run_move::<11, 7>();
        assert_eq!(robot.p, (8, 4));
    }

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(part_1::<11, 7>(&Robot::parse_list(EXAMPLE)), 12);
    }
}
