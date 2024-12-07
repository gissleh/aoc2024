use arrayvec::ArrayVec;
use common::parser;
use common::parser::Parser;
use common::runner::{BothParts, Runner};
use rayon::prelude::*;

pub fn main(r: &mut Runner, input: &[u8]) {
    let equations = r.prep("Parse", || {
        Equation::list_parser().parse_value(input).unwrap()
    });
    r.part("Part 1", || part_1(&equations));
}

fn part_1(equations: &[Equation]) -> BothParts<u64, u64> {
    equations
        .iter()
        .fold(BothParts(0, 0), |BothParts(p1, p2), e| {
            if e.check_p1() {
                BothParts(p1+e.expected, p2)
            } else if e.check_p2() {
                BothParts(p1, p2+e.expected)
            } else {
                BothParts(p1, p2)
            }
        })
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
        for n in 0..(self.operands.len() as u16).pow(3) {
            let mut n = n;
            let mut total = self.operands[0];
            for v in self.operands.iter().skip(1) {
                let (op, next_n) = Operator::next_p2(n);

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

    fn parser<'i>() -> impl Parser<'i, Self> {
        parser::uint::<u64>()
            .and_discard(b": ")
            .and(parser::uint::<u64>().delimited_by(b' ').repeat())
            .map(|(expected, operands)| Self { expected, operands })
    }

    fn list_parser<'i>() -> impl Parser<'i, Vec<Self>> {
        Self::parser().delimited_by(b'\n').repeat()
    }
}

enum Operator {
    Add,
    Mul,
    Cat,
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
            },
        }
    }

    fn next_p1(v: u16) -> (Operator, u16) {
        (
            if v & 1 == 1 {
                Operator::Mul
            } else {
                Operator::Add
            },
            v >> 1,
        )
    }

    fn next_p2(v: u16) -> (Operator, u16) {
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
