use arrayvec::ArrayVec;
use common::parser;
use common::parser::Parser;
use common::runner::{BothParts, Runner};
use rayon::prelude::*;
use std::fmt::{Display, Formatter};

pub fn main(r: &mut Runner, input: &[u8]) {
    let equations = r.prep("Parse", || {
        Equation::list_parser().parse_value(input).unwrap()
    });
    r.part("Both Parts", || both_parts(&equations));
}

fn both_parts(equations: &[Equation]) -> BothParts<u64, u64> {
    equations
        .par_iter()
        .fold(
            || BothParts(0, 0),
            |BothParts(p1, p2), e| {
                if e.check_p1() {
                    BothParts(p1 + e.expected, p2 + e.expected)
                } else if e.check_p2() {
                    BothParts(p1, p2 + e.expected)
                } else {
                    #[cfg(test)]
                    println!("Failed: {} {:?}", e.expected, e.operands);
                    BothParts(p1, p2)
                }
            },
        )
        .sum()
}

struct Equation {
    expected: u64,
    operands: ArrayVec<u64, 14>,
}

impl Equation {
    fn check_p1(&self) -> bool {
        for n in 0..(1 << self.operands.len()) {
            let mut n = n;
            let mut total = self.operands[0];
            for v in self.operands.iter().skip(1) {
                let (op, next_n) = Operator::next_p1(n);

                total = op.apply(total, *v);
                if total > self.expected {
                    break;
                }

                n = next_n;
            }

            if total == self.expected {
                return true;
            }
        }

        false
    }

    fn check_p2(&self) -> bool {
        for n in 0..3u32.pow(self.operands.len() as u32 - 1) {
            let mut n = n;
            let mut total = self.operands[0];

            #[cfg(test)]
            print!("{} ", self.operands[0]);

            for i in 1..self.operands.len() {
                let (op, next_n) = Operator::next_p2(n);
                #[cfg(test)]
                print!("{op} {} ", self.operands[i]);

                total = op.apply(total, self.operands[i]);

                n = next_n;

                if total > self.expected {
                    #[cfg(test)]
                    print!("> {total} (stopped early)");

                    break;
                }
            }

            if total == self.expected {
                #[cfg(test)]
                println!("== {total}");
                return true;
            }

            #[cfg(test)]
            if total < self.expected {
                println!("< {total}");
            }

            #[cfg(test)]
            println!();
        }

        false
    }

    #[allow(dead_code)]
    fn new(expected: u64, operands: ArrayVec<u64, 14>) -> Equation {
        Self { expected, operands }
    }

    #[inline]
    fn parser<'i>() -> impl Parser<'i, Self> {
        parser::uint::<u64>()
            .and_discard(b": ")
            .and(parser::uint::<u64>().delimited_by(b' ').repeat())
            .map(|(expected, operands)| Self { expected, operands })
    }

    #[inline]
    fn list_parser<'i>() -> impl Parser<'i, Vec<Self>> {
        Self::parser().delimited_by(b'\n').repeat()
    }
}

enum Operator {
    Add,
    Mul,
    Cat,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Mul => write!(f, "*"),
            Operator::Cat => write!(f, "||"),
        }
    }
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Mul => a * b,
            Operator::Cat => {
                for f in [10, 100, 1_000, 10_000, 100_000, 1_000_000] {
                    if f > b {
                        return (a * f) + b;
                    }
                }

                unreachable!()
            }
        }
    }

    fn next_p1(v: u32) -> (Operator, u32) {
        (
            if v & 1 == 1 {
                Operator::Mul
            } else {
                Operator::Add
            },
            v >> 1,
        )
    }

    fn next_p2(v: u32) -> (Operator, u32) {
        (
            match v % 3 {
                0 => Operator::Add,
                1 => Operator::Mul,
                2 => Operator::Cat,
                _ => unreachable!(),
            },
            v / 3,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn operator_apply() {
        assert_eq!(Operator::Add.apply(1, 2), 3);
        assert_eq!(Operator::Mul.apply(3, 2), 6);
        assert_eq!(Operator::Cat.apply(3, 2), 32);
        assert_eq!(Operator::Cat.apply(643, 332), 643332);
    }

    #[test]
    fn both_parts_works_on_example() {
        let stuff = Equation::list_parser().parse_value(EXAMPLE).unwrap();
        assert_eq!(both_parts(&stuff), BothParts(3749, 11387));
    }
}
