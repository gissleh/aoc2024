use arrayvec::ArrayVec;
use common::parser;
use common::parser::Parser;
use common::runner::Runner;
use rustc_hash::FxHashMap;

pub fn main(r: &mut Runner, input: &[u8]) {
    let (x, y, wires) = r.prep("Parse", || parse(input));

    r.part("Part 1", || part_1(&wires, x, y));
    r.part("Part 2", || part_2(&wires));
    r.info("Gates", &wires.map.len());
}

fn part_1(wires: &Wires, x: u64, y: u64) -> u64 {
    wires.run_z(x, y)
}

fn part_2(wires: &Wires) -> String {
    const Z00: u16 = z_id(0);
    const Z01: u16 = z_id(1);

    let mut baddies = ArrayVec::<u16, 80>::new();

    let last_z = (0..64)
        .map(|i| z_id(i))
        .take_while(|id| wires.map.contains_key(&id))
        .last()
        .unwrap();

    for (out, op) in wires.map.iter().map(|(a, b)| (*a, *b)) {
        match op {
            Op::XOR(a, b) => {
                if a.named().is_some() != (Z00 != out && is_z(out)) {
                    baddies.push(out);
                    continue;
                }

                if out != Z00 && out != Z01 {
                    if let Some((an, a)) = a.named().map(|n| (n, wires.map[&n])) {
                        if let Some((bn, b)) = b.named().map(|n| (n, wires.map[&n])) {
                            if a.is_and() {
                                baddies.push(an);
                            }
                            if b.is_and() {
                                baddies.push(bn);
                            }
                        }
                    }
                }
            }
            Op::OR(a, b) => {
                if is_z(out) && out != last_z {
                    baddies.push(out);
                    continue;
                }

                if let Some((an, a)) = a.named().map(|n| (n, wires.map[&n])) {
                    if let Some((bn, b)) = b.named().map(|n| (n, wires.map[&n])) {
                        if !a.is_and() {
                            baddies.push(an);
                        }
                        if !b.is_and() {
                            baddies.push(bn);
                        }
                    }
                }
            }
            Op::AND(_, _) => {
                if is_z(out) {
                    baddies.push(out);
                    continue;
                }
            }
        }
    }

    baddies.sort_unstable();
    let mut res = String::with_capacity(4 * 8);
    for i in 0..baddies.len() {
        if i > 0 && baddies[i] == baddies[i - 1] {
            continue;
        }

        if !res.is_empty() {
            res.push(',')
        }

        for ch in id_to_name(baddies[i]) {
            res.push(ch as char);
        }
    }

    res
}

fn parse(input: &[u8]) -> (u64, u64, Wires) {
    xy_parser()
        .and_discard(b'\n')
        .and(Wires::parser())
        .map(|((x, y), wires)| (x, y, wires))
        .parse_value(input)
        .unwrap()
}

fn xy_parser<'i>() -> impl Parser<'i, (u64, u64)> {
    b'x'.or(b'y')
        .and(parser::uint::<u32>())
        .and_discard(b": ")
        .and(b'0'.or(b'1'))
        .and_discard(b'\n')
        .repeat_fold(
            || (0u64, 0u64),
            |(x, y), ((xy, i), v)| {
                if v == b'0' {
                    (x, y)
                } else if xy == b'x' {
                    (x | 1 << i, y)
                } else {
                    (x, y | 1 << i)
                }
            },
        )
}

struct Wires {
    map: FxHashMap<u16, Op>,
}

impl Wires {
    fn run_z(&self, x: u64, y: u64) -> u64 {
        let mut res = 0;
        for z in 0..64 {
            if let Some(op) = self.map.get(&z_id(z)) {
                if self.run_op(*op, x, y) {
                    res |= 1 << z;
                }
            } else {
                break;
            }
        }

        res
    }

    fn run_ref(&self, r: Ref, x: u64, y: u64) -> bool {
        match r {
            Ref::X(i) => x & 1 << i != 0,
            Ref::Y(i) => y & 1 << i != 0,
            Ref::Named(n) => self.run_op(self.map[&n], x, y),
        }
    }

    fn run_op(&self, op: Op, x: u64, y: u64) -> bool {
        match op {
            Op::AND(a, b) => self.run_ref(a, x, y) && self.run_ref(b, x, y),
            Op::OR(a, b) => self.run_ref(a, x, y) || self.run_ref(b, x, y),
            Op::XOR(a, b) => self.run_ref(a, x, y) ^ self.run_ref(b, x, y),
        }
    }

    fn parser<'i>() -> impl Parser<'i, Wires> {
        Op::parser()
            .and_discard(b" -> ")
            .and(Ref::parser())
            .delimited_by(b'\n')
            .repeat_fold_mut(
                || FxHashMap::with_capacity_and_hasher(128, Default::default()),
                |named, (op, out)| match out {
                    Ref::Named(id) => {
                        named.insert(id, op);
                    }
                    _ => panic!("{out:?} is not a valid output"),
                },
            )
            .map(|named| Self { map: named })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Ref {
    X(u16),
    Y(u16),
    Named(u16),
}

impl Ref {
    fn parser<'i>() -> impl Parser<'i, Ref> {
        fn two_digit(a: [u8; 3]) -> u16 {
            (((a[1] - b'0') * 10) + (a[2] - b'0')) as u16
        }

        fn op_number(a: [u8; 3]) -> u16 {
            ((digit(a[0]) as u16) * 36 * 36) + ((digit(a[1]) as u16) * 36) + (digit(a[2]) as u16)
        }

        parser::n_bytes().map(|name: [u8; 3]| {
            if name[0] == b'x' {
                Self::X(two_digit(name))
            } else if name[0] == b'y' {
                Self::Y(two_digit(name))
            } else {
                Self::Named(op_number(name))
            }
        })
    }

    fn named(&self) -> Option<u16> {
        match self {
            Ref::X(_) => None,
            Ref::Y(_) => None,
            Ref::Named(n) => Some(*n),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Op {
    XOR(Ref, Ref),
    AND(Ref, Ref),
    OR(Ref, Ref),
}

impl Op {
    fn parser<'i>() -> impl Parser<'i, Op> {
        Ref::parser()
            .and_discard(b' ')
            .and(parser::word())
            .and_discard(b' ')
            .and(Ref::parser())
            .map(|((a, operand), b): ((Ref, &[u8]), Ref)| match operand {
                b"AND" => Op::AND(a, b),
                b"OR" => Op::OR(a, b),
                b"XOR" => Op::XOR(a, b),
                _ => unreachable!(),
            })
    }

    fn is_and(&self) -> bool {
        match self {
            Self::AND(_, _) => true,
            _ => false,
        }
    }
}

fn digit(v: u8) -> u8 {
    match v {
        b'0'..=b'9' => v - b'0',
        b'a'..=b'z' => v - b'a' + 10,
        _ => unreachable!(),
    }
}

const fn z_id(z: u16) -> u16 {
    (36 * 36 * 35) + ((z / 10) * 36) + (z % 10)
}

fn is_z(id: u16) -> bool {
    id >= (36 * 36 * 35)
}

#[inline]
fn id_to_name(id: u16) -> [u8; 3] {
    [
        id_char(id / (36 * 36)),
        id_char((id / 36) % 36),
        id_char(id % 36),
    ]
}

#[inline]
fn id_char(id: u16) -> u8 {
    if id < 10 {
        (id as u8) + b'0'
    } else {
        (id as u8 - 10) + b'a'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &[u8] = b"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

    const EXAMPLE_2: &[u8] = b"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

    #[test]
    fn part_1_works_on_simple_example() {
        let (x, y, wires) = parse(EXAMPLE_1);
        assert_eq!(part_1(&wires, x, y), 4);
    }

    #[test]
    fn part_1_works_on_complex_example() {
        let (x, y, wires) = parse(EXAMPLE_2);
        assert_eq!(part_1(&wires, x, y), 2024);
    }
}
