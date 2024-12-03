use common::parser::{uint, Parser};
use common::runner::Runner;

pub fn main(r: &mut Runner, input: &[u8]) {
    let res = r.prep("Parse", || parse(input));
    r.part("Part 1", || part_1(&res));
    r.part("Part 2", || part_2(&res));
}

fn parse(input: &[u8]) -> Vec<Instruction> {
    Instruction::parser()
        .extract()
        .repeat::<Vec<_>>()
        .parse_value(input)
        .unwrap()
}

fn part_1(input: &[Instruction]) -> u32 {
    let mut sum = 0;

    for inst in input.iter() {
        match inst {
            Instruction::Mul(a, b) => {
                sum += *a * *b;
            }
            _ => {}
        }
    }

    sum
}

fn part_2(input: &[Instruction]) -> u32 {
    let mut sum = 0;
    let mut enabled = true;

    for inst in input.iter() {
        match inst {
            Instruction::Do => {
                enabled = true;
            }
            Instruction::Dont => {
                enabled = false;
            }
            Instruction::Mul(a, b) => {
                if enabled {
                    sum += *a * *b;
                }
            }
        }
    }

    sum
}

#[derive(Eq, PartialEq, Debug)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

impl Instruction {
    #[inline]
    fn mul_parser<'i>() -> impl Parser<'i, (u32, u32)> {
        b"mul("
            .and_instead(uint::<u32>())
            .and_discard(b',')
            .and(uint::<u32>())
            .and_discard(b')')
    }

    #[inline]
    fn parser<'i>() -> impl Parser<'i, Self> {
        Self::mul_parser()
            .map(|(x, y)| Instruction::Mul(x, y))
            .or(b"don't()".map(|_| Instruction::Dont))
            .or(b"do()".map(|_| Instruction::Do))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &[u8] =
        b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_2: &[u8] =
        b"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn mul_parser() {
        assert_eq!(
            Instruction::mul_parser().parse_value(b"mul(17,16)"),
            Some((17, 16))
        );
        assert_eq!(
            Instruction::mul_parser().parse_value(b"mul(644,123)"),
            Some((644, 123))
        );
        assert_eq!(Instruction::mul_parser().parse_value(b"mul(644,123"), None);
        assert_eq!(
            Instruction::mul_parser().parse_value(b"mul(644,123,76)"),
            None
        );
        assert_eq!(Instruction::mul_parser().parse_value(b"mul(644)"), None);
        assert_eq!(
            Instruction::mul_parser().parse_value(b"mool(123,345)"),
            None
        );
    }

    #[test]
    fn instruction_parser() {
        assert_eq!(
            Instruction::parser().parse_value(b"mul(17,16)"),
            Some(Instruction::Mul(17, 16))
        );
        assert_eq!(
            Instruction::parser().parse_value(b"do()"),
            Some(Instruction::Do)
        );
        assert_eq!(
            Instruction::parser().parse_value(b"don't()"),
            Some(Instruction::Dont)
        );
        assert_eq!(Instruction::parser().parse_value(b"do("), None);
        assert_eq!(Instruction::parser().parse_value(b"don't("), None);
        assert_eq!(Instruction::parser().parse_value(b"do_not()"), None);
    }

    #[test]
    fn parse_works_on_examples() {
        assert_eq!(
            parse(EXAMPLE_1),
            vec![
                Instruction::Mul(2, 4),
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Mul(8, 5),
            ]
        );

        assert_eq!(
            parse(EXAMPLE_2),
            vec![
                Instruction::Mul(2, 4),
                Instruction::Dont,
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Do,
                Instruction::Mul(8, 5),
            ]
        );
    }

    #[test]
    fn part1_works_on_example() {
        assert_eq!(part_1(&parse(EXAMPLE_1)), 161);
    }

    #[test]
    fn part2_works_on_example() {
        assert_eq!(part_2(&parse(EXAMPLE_2)), 48);
    }
}
