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
    fn parser<'i>() -> impl Parser<'i, Self> {
        b"mul("
            .and_instead(uint::<u32>().only_if(|v| *v > 0 && *v < 1000))
            .and_discard(b',')
            .and(uint::<u32>().only_if(|v| *v > 0 && *v < 1000))
            .and_discard(b')')
            .map(|(x, y)| Instruction::Mul(x, y))
            .or(b'd'.and_instead(
                b"on't()".map(|_| Instruction::Dont)
                .or(b"o()".map(|_| Instruction::Do))
            ))
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
    fn instruction_parser() {
        assert_eq!(
            Instruction::parser().parse_value(b"mul(44,46)"),
            Some(Instruction::Mul(44, 46))
        );
        assert_eq!(
            Instruction::parser().parse_value(b"mul(123,4)"),
            Some(Instruction::Mul(123, 4))
        );
        assert_eq!(
            Instruction::parser().parse_value(b"do()"),
            Some(Instruction::Do)
        );
        assert_eq!(
            Instruction::parser().parse_value(b"don't()"),
            Some(Instruction::Dont)
        );
        assert_eq!(Instruction::parser().parse_value(b"mul(4*"), None);
        assert_eq!(Instruction::parser().parse_value(b"mul(6,9!"), None);
        assert_eq!(Instruction::parser().parse_value(b"?(12,34)"), None);
        assert_eq!(Instruction::parser().parse_value(b"mul ( 2 , 4 )"), None);
        assert_eq!(Instruction::parser().parse_value(b"do("), None);
        assert_eq!(Instruction::parser().parse_value(b"don't("), None);
        assert_eq!(Instruction::parser().parse_value(b"do_not()"), None);
        assert_eq!(Instruction::parser().parse_value(b"mul(0,999)"), None);
        assert_eq!(Instruction::parser().parse_value(b"mul(999,0)"), None);
        assert_eq!(Instruction::parser().parse_value(b"mul(1000,543)"), None);
        assert_eq!(Instruction::parser().parse_value(b"mul(64,1000)"), None);
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
