use common::parser;
use common::parser::Parser;
use common::runner::Runner;

pub fn main(r: &mut Runner, input: &[u8]) {
    let machines = r.prep("Parse", || Machine::parse_list(input));
    r.part("Part 1 (BF)", || part_1_bf(&machines));
    r.set_tail("Parse");
    r.part("Part 1", || part_1(&machines));
    r.part("Part 2", || part_2(&machines));

    r.info("Machines", &machines.len());
}

fn part<F>(machines: &[Machine], f: F) -> i64
where
    F: Fn(&Machine) -> Option<i64>,
{
    machines.iter().filter_map(f).sum::<i64>()
}

fn part_1_bf(machines: &[Machine]) -> i64 {
    part(machines, |m| m.solve_bruteforce(100))
}

fn part_1(machines: &[Machine]) -> i64 {
    part(machines, |m| m.solve())
}

fn part_2(machines: &[Machine]) -> i64 {
    part(machines, |m| m.with_correction().solve())
}

struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn with_correction(&self) -> Self {
        Self {
            a: self.a,
            b: self.b,
            prize: (self.prize.0 + 10000000000000, self.prize.1 + 10000000000000),
        }
    }

    fn solve_bruteforce(&self, max: i64) -> Option<i64> {
        let (ax, ay) = self.a;
        let (bx, by) = self.b;
        let (px, py) = self.prize;

        for a_presses in 1..=max {
            let ayp = ay * a_presses;
            if ayp > py {
                break;
            }

            let rem = (py - ayp) % by;
            if rem == 0 {
                for b_presses in 1..=max {
                    let rx = (a_presses * ax) + (b_presses * bx);
                    if rx == px {
                        let ry = (a_presses * ay) + (b_presses * by);
                        if ry == py {
                            return Some((a_presses * 3) + b_presses);
                        }
                    } else if rx > px {
                        break;
                    }
                }
            }
        }

        None
    }

    fn solve(&self) -> Option<i64> {
        let (ax, ay) = self.a;
        let (bx, by) = self.b;
        let (px, py) = self.prize;

        let determinant = (ax * by) - (bx * ay);
        if determinant == 0 {
            return None;
        }

        let a_presses = ((px * by) - (bx * py)) / determinant;
        let b_presses = ((ax * py) - (px * ay)) / determinant;

        let rx = (a_presses * ax) + (b_presses * bx);
        let ry = (a_presses * ay) + (b_presses * by);
        if (rx, ry) == self.prize {
            Some((a_presses * 3) + b_presses)
        } else {
            None
        }
    }

    fn parse_list(input: &[u8]) -> Vec<Self> {
        Self::parser().repeat().parse_value(input).unwrap()
    }

    fn parser<'i>() -> impl Parser<'i, Self> {
        b"Button A: X+"
            .and_instead(
                parser::uint::<i64>()
                    .and_discard(b", Y+")
                    .and(parser::uint::<i64>())
                    .and_discard(b'\n'),
            )
            .and_discard(b"Button B: X+")
            .and(
                parser::uint::<i64>()
                    .and_discard(b", Y+")
                    .and(parser::uint::<i64>())
                    .and_discard(b'\n'),
            )
            .and_discard(b"Prize: X=")
            .and(
                parser::uint::<i64>()
                    .and_discard(b", Y=")
                    .and(parser::uint::<i64>())
                    .and_discard(b'\n'),
            )
            .and_skip(b"\n")
            .map(|((a, b), prize)| Self { a, b, prize })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn part1_works_on_example() {
        assert_eq!(part_1(&Machine::parse_list(EXAMPLE)), 480);
    }
}
