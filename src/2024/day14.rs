use common::parser;
use common::parser::Parser;
use common::runner::Runner;
use common::utils::crt;
use num::integer::lcm;

pub fn main(r: &mut Runner, input: &[u8]) {
    let robots = r.prep("Parse", || Robot::<101, 103>::parse_list(input));

    r.part("Part 1", || part_1::<101, 103>(&robots));
    r.part("Part 2", || part_2::<101, 103>(&robots));

    r.info("Robots", &robots.len());
}

fn part_1<const W: u32, const H: u32>(robots: &[Robot<W, H>]) -> u32 {
    let mut robots = robots.iter().copied().collect::<Vec<_>>();

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.run_move()
        }
    }

    quadrants::<W, H>(&robots).iter().product::<u32>()
}

fn part_2<const W: u32, const H: u32>(robots: &[Robot<W, H>]) -> u32 {
    let mut robots = robots.iter().copied().collect::<Vec<_>>();

    let mut x_hist = vec![0u16; W as usize];
    let mut y_hist = vec![0u16; H as usize];

    let mut best_x = (0u32, 0u16);
    let mut best_y = (0u32, 0u16);

    for seconds in 1..lcm(W, H) {
        for robot in robots.iter_mut() {
            robot.run_move();
            x_hist[robot.p.0 as usize] += 1;
            y_hist[robot.p.1 as usize] += 1;
        }

        let x_max = *x_hist.iter().max().unwrap();
        if x_max > best_x.1 {
            best_x = (seconds, x_max);
        }

        let y_max = *y_hist.iter().max().unwrap();
        if y_max > best_y.1 {
            best_y = (seconds, y_max);
        }

        if best_x.1 > 24 && best_y.1 > 24 {
            // CRT code I borrowed from Rosetta Code 3 years ago go brr.
            return crt(&[(best_x.0 as i64, W as i64), (best_y.0 as i64, H as i64)]) as u32;
        }

        #[cfg(debug_assertions)]
        if x_max > 24 || y_max > 24 {
            println!("Seconds: {}", seconds);
            for y in 0..H {
                for x in 0..W {
                    let count = robots.iter().filter(|r| r.p == (x, y)).count();
                    if count > 0 {
                        print!("{count}");
                    } else {
                        print!(" ");
                    }
                }

                println!();
            }
        }

        x_hist.fill(0);
        y_hist.fill(0);
    }

    0
}

fn quadrants<const W: u32, const H: u32>(robots: &[Robot<W, H>]) -> [u32; 4] {
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

    quadrants
}

#[derive(Copy, Clone, Debug)]
struct Robot<const W: u32, const H: u32> {
    p: (u32, u32),
    v: (u32, u32),
}

impl<const W: u32, const H: u32> Robot<W, H> {
    fn run_move(&mut self) {
        let (px, py) = &mut self.p;
        let (vx, vy) = self.v;

        *px = (*px + vx) % W;
        *py = (*py + vy) % H;
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
                parser::uint::<u32>()
                    .and_discard(b',')
                    .and(parser::uint::<u32>()),
            )
            .and_discard(b" v=")
            .and(
                parser::int::<i32>()
                    .and_discard(b',')
                    .and(parser::int::<i32>()),
            )
            .map(|(p, v): ((u32, u32), (i32, i32))| Robot {
                p,
                v: ((v.0 + W as i32) as u32 % W, (v.1 + H as i32) as u32 % H),
            })
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
        let mut robot = Robot::<11, 7>::parser()
            .parse_value(b"p=2,4 v=2,-3")
            .unwrap();
        assert_eq!(robot.p, (2, 4));
        assert_eq!(robot.v, (2, 7 - 3));

        robot.run_move();
        assert_eq!(robot.p, (4, 1));

        robot.run_move();
        assert_eq!(robot.p, (6, 5));

        robot.run_move();
        assert_eq!(robot.p, (8, 2));

        robot.run_move();
        assert_eq!(robot.p, (10, 6));

        robot.run_move();
        assert_eq!(robot.p, (1, 3));

        let mut robot = Robot::<11, 7>::parser()
            .parse_value(b"p=3,3 v=-3,-3")
            .unwrap();
        assert_eq!(robot.p, (3, 3));
        assert_eq!(robot.v, (11 - 3, 7 - 3));

        robot.run_move();
        assert_eq!(robot.p, (0, 0));

        robot.run_move();
        assert_eq!(robot.p, (8, 4));
    }

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(part_1(&Robot::<11, 7>::parse_list(EXAMPLE)), 12);
    }
}
