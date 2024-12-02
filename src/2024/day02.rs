use arrayvec::ArrayVec;
use common::parser;
use common::parser::Parser;
use common::runner::Runner;

pub fn main(r: &mut Runner, input: &[u8]) {
    let reports = r.prep("Parse", || input_parser().parse_value(input).unwrap());
    r.part("Part 1", || part_1(&reports));
    r.part("Part 2", || part_2(&reports));
}

fn part_1(reports: &[Report]) -> u32 {
    reports.iter()
        .filter(|r| r.safe())
        .count() as u32
}

fn part_2(reports: &[Report]) -> u32 {
    reports.iter()
        .filter(|r| r.safe_p2())
        .count() as u32
}

fn input_parser<'i>() -> impl Parser<'i, Vec<Report>> {
    Report::parser()
        .delimited_by(b'\n')
        .repeat()
}

struct Report {
    levels: ArrayVec<u8, 8>
}

impl Report {
    #[inline]
    fn safe_p2(&self) -> bool {
        match self.unsafety() {
            Unsafety::Safe => true,
            Unsafety::BadDirection(i) => (i == 1 && self.without(0).safe()) || (i != 1 && self.without(i).safe()) || self.without(i + 1).safe(),
            Unsafety::BadDiff(i) => self.without(i).safe() || self.without(i+1).safe(),
        }
    }

    #[inline]
    fn without(&self, i: usize) -> Report {
        let mut levels = self.levels.clone();
        levels.remove(i);

        Report{levels}
    }

    fn safe(&self) -> bool {
        match self.unsafety() {
            Unsafety::Safe => true,
            _ => false,
        }
    }

    #[inline]
    fn unsafety(&self) -> Unsafety {
        use Unsafety::*;

        if self.levels[0] > self.levels[1] {
            for (i, [a,b]) in self.levels.array_windows::<2>().enumerate() {
                let abs_diff = a.abs_diff(*b);
                if abs_diff == 0 || abs_diff > 3 {
                    return BadDiff(i);
                }

                if *a < *b {
                    return BadDirection(i);
                }
            }
        } else {
            for (i, [a,b]) in self.levels.array_windows::<2>().enumerate() {
                let abs_diff = a.abs_diff(*b);
                if abs_diff == 0 || abs_diff > 3 {
                    return BadDiff(i);
                }

                if *a > *b {
                    return BadDirection(i);
                }
            }
        }

        Safe
    }

    #[inline]
    fn parser<'i>() -> impl Parser<'i, Self> {
        parser::uint::<u8>()
            .delimited_by(b' ')
            .repeat_limited(1, 8)
            .map(|levels| Self { levels })
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Unsafety {
    Safe,
    BadDiff(usize),
    BadDirection(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn part1_works_on_example() {
        let input = input_parser().parse_value(EXAMPLE).unwrap();
        assert_eq!(input.len(), 6);
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn part2_works_on_example() {
        let input = input_parser().parse_value(EXAMPLE).unwrap();
        assert_eq!(input.len(), 6);
        assert_eq!(part_2(&input), 4);
    }

    #[test]
    fn part1_safety() {
        let gen = |i| Report::parser().parse_value(i).unwrap();

        assert_eq!(gen(b"7 6 4 2 1").safe(), true);
        assert_eq!(gen(b"1 2 7 8 9").safe(), false);
        assert_eq!(gen(b"9 7 6 2 1").safe(), false);
        assert_eq!(gen(b"1 3 2 4 5").safe(), false);
        assert_eq!(gen(b"8 6 4 4 1").safe(), false);
        assert_eq!(gen(b"1 3 6 7 9").safe(), true);
    }

    #[test]
    fn part2_safety() {
        let gen = |i| Report::parser().parse_value(i).unwrap();

        assert_eq!(gen(b"7 6 4 2 1").safe_p2(), true);
        assert_eq!(gen(b"1 2 7 8 9").safe_p2(), false);
        assert_eq!(gen(b"9 7 6 2 1").safe_p2(), false);
        assert_eq!(gen(b"1 3 2 4 5").safe_p2(), true);
        assert_eq!(gen(b"8 6 4 4 1").safe_p2(), true);
        assert_eq!(gen(b"1 3 6 7 9").safe_p2(), true);

        assert_eq!(gen(b"0 0 1 2 3").safe_p2(), true);
        assert_eq!(gen(b"0 1 2 3 3").safe_p2(), true);
    }
}