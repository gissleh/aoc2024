use arrayvec::ArrayVec;
use common::parser::Parser;
use common::runner::Runner;
use rustc_hash::FxHashMap;

const WHITE: u8 = 0;
const BLUE: u8 = 1;
const BLACK: u8 = 2;
const RED: u8 = 3;
const GREEN: u8 = 4;

pub fn main(r: &mut Runner, input: &[u8]) {
    let (towels, patterns) = r.prep("Parse", || parse(input));

    r.part("Part 1", || part_1(&towels, &patterns));
    r.part("Part 2", || part_2(&towels, &patterns));

    r.info("Towel nodes", &towels.nodes.len());
    r.info("Patterns", &patterns.len());
}

fn part_1(towels: &Towels, patterns: &[Pattern]) -> u32 {
    patterns
        .iter()
        .filter(|p| towels.possible_patterns(*p) > 0)
        .count() as u32
}

fn part_2(towels: &Towels, patterns: &[Pattern]) -> u64 {
    patterns.iter().map(|p| towels.possible_patterns(p)).sum()
}

fn parse(input: &[u8]) -> (Towels, Vec<Pattern>) {
    Towels::parser()
        .and_discard(b"\n\n")
        .and(Pattern::parser().and_discard(b'\n').repeat())
        .parse_value(input)
        .unwrap()
}

struct Towels {
    nodes: Vec<TowelNode>,
}

impl Towels {
    fn new() -> Self {
        let mut nodes = Vec::with_capacity(128);
        nodes.push(TowelNode {
            value: 0,
            next: [0, 0, 0, 0, 0],
        });

        Self { nodes }
    }

    fn add(&mut self, pattern: &Pattern, value: u16) {
        let mut current_index = 0usize;
        for i in 0..pattern.0.len() {
            let pi = pattern.0[i] as usize;
            if self.nodes[current_index].next[pi] != 0 {
                current_index = self.nodes[current_index].next[pi] as usize;
            } else {
                self.nodes.push(TowelNode::default());
                let next_index = self.nodes.len() - 1;
                self.nodes[current_index].next[pi] = next_index as u16;
                current_index = next_index;
            }
        }

        #[cfg(debug_assertions)]
        assert_eq!(self.nodes[current_index].value, 0);

        self.nodes[current_index].value = value;
    }

    fn possible_patterns(&self, pattern: &Pattern) -> u64 {
        self.possible_patterns_recursion_step(&pattern.0, &mut FxHashMap::default())
    }

    fn possible_patterns_recursion_step<'p>(
        &self,
        pattern: &'p [u8],
        cache: &mut FxHashMap<&'p [u8], u64>,
    ) -> u64 {
        if pattern.len() == 0 {
            return 1;
        }

        if let Some(combos) = cache.get(pattern) {
            return *combos;
        }

        let mut total = 0;
        for (_, len) in self.iter(pattern) {
            total += self.possible_patterns_recursion_step(&pattern[len..], cache);
        }

        cache.insert(pattern, total);
        total
    }

    fn iter<'t, 'p>(&'t self, pattern: &'p [u8]) -> TowelsPatternIterator<'t, 'p> {
        TowelsPatternIterator::new(self, pattern)
    }

    #[inline]
    fn parser<'i>() -> impl Parser<'i, Self> {
        Pattern::parser()
            .delimited_by(b", ")
            .repeat_fold_mut(
                || (Self::new(), 1),
                |(towels, next_value), pattern| {
                    towels.add(&pattern, *next_value);
                    *next_value += 1;
                },
            )
            .map(|(towels, _)| towels)
    }
}

struct TowelsPatternIterator<'t, 'p> {
    towels: &'t Towels,
    pattern: &'p [u8],
    current_index: usize,
    current_pos: usize,
}

impl<'t, 'p> TowelsPatternIterator<'t, 'p> {
    fn new(towels: &'t Towels, pattern: &'p [u8]) -> Self {
        Self {
            current_index: if pattern.len() > 0 {
                towels.nodes[0].next[pattern[0] as usize] as usize
            } else {
                0
            },
            current_pos: 0,
            pattern,
            towels,
        }
    }
}

impl<'t, 'p> Iterator for TowelsPatternIterator<'t, 'p> {
    type Item = (u16, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_index != 0 {
            let node = &self.towels.nodes[self.current_index];

            self.current_pos += 1;
            if self.current_pos < self.pattern.len() {
                self.current_index = node.next[self.pattern[self.current_pos] as usize] as usize;
            } else {
                self.current_index = 0
            }

            if node.value != 0 {
                return Some((node.value, self.current_pos));
            }
        }

        None
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
struct TowelNode {
    value: u16,
    next: [u16; 5],
}

impl TowelNode {
    #[allow(dead_code)]
    fn new(value: u16, w: u16, u: u16, b: u16, r: u16, g: u16) -> Self {
        Self {
            value,
            next: [w, u, b, r, g],
        }
    }
}

struct Pattern(ArrayVec<u8, 64>);

impl Pattern {
    #[inline]
    fn parser<'i>() -> impl Parser<'i, Self> {
        b'w'.map(|_| WHITE)
            .or(b'u'.map(|_| BLUE))
            .or(b'b'.map(|_| BLACK))
            .or(b'r'.map(|_| RED))
            .or(b'g'.map(|_| GREEN))
            .repeat()
            .map(|array| Pattern(array))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &[u8] = b"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    fn parse_pattern(input: &[u8]) -> Pattern {
        Pattern::parser().parse_value(input).unwrap()
    }

    #[test]
    fn adding_towels_work() {
        let mut towels = Towels::new();
        towels.add(&parse_pattern(b"r"), 1);

        assert_eq!(
            towels.nodes,
            vec![
                TowelNode::new(0, 0, 0, 0, 1, 0),
                TowelNode::new(1, 0, 0, 0, 0, 0),
            ]
        );

        towels.add(&parse_pattern(b"urug"), 2);
        assert_eq!(
            towels.nodes,
            vec![
                TowelNode::new(0, 0, 2, 0, 1, 0),
                TowelNode::new(1, 0, 0, 0, 0, 0),
                TowelNode::new(0, 0, 0, 0, 3, 0),
                TowelNode::new(0, 0, 4, 0, 0, 0),
                TowelNode::new(0, 0, 0, 0, 0, 5),
                TowelNode::new(2, 0, 0, 0, 0, 0),
            ]
        );

        towels.add(&parse_pattern(b"uru"), 3);
        assert_eq!(
            towels.nodes,
            vec![
                TowelNode::new(0, 0, 2, 0, 1, 0),
                TowelNode::new(1, 0, 0, 0, 0, 0),
                TowelNode::new(0, 0, 0, 0, 3, 0),
                TowelNode::new(0, 0, 4, 0, 0, 0),
                TowelNode::new(3, 0, 0, 0, 0, 5),
                TowelNode::new(2, 0, 0, 0, 0, 0),
            ]
        );

        towels.add(&parse_pattern(b"ug"), 4);
        assert_eq!(
            towels.nodes,
            vec![
                TowelNode::new(0, 0, 2, 0, 1, 0),
                TowelNode::new(1, 0, 0, 0, 0, 0),
                TowelNode::new(0, 0, 0, 0, 3, 6),
                TowelNode::new(0, 0, 4, 0, 0, 0),
                TowelNode::new(3, 0, 0, 0, 0, 5),
                TowelNode::new(2, 0, 0, 0, 0, 0),
                TowelNode::new(4, 0, 0, 0, 0, 0),
            ]
        );
    }

    #[test]
    fn parsing_example_towels_work() {
        let towels = Towels::parser().parse_value(EXAMPLE_1).unwrap();

        assert_eq!(
            towels.nodes,
            vec![
                TowelNode::new(0, 2, 0, 4, 1, 5),
                TowelNode::new(1, 0, 0, 8, 0, 0),
                TowelNode::new(0, 0, 0, 0, 3, 0),
                TowelNode::new(2, 0, 0, 0, 0, 0),
                TowelNode::new(3, 6, 0, 0, 10, 0),
                TowelNode::new(4, 0, 0, 9, 0, 0),
                TowelNode::new(0, 0, 7, 0, 0, 0),
                TowelNode::new(5, 0, 0, 0, 0, 0),
                TowelNode::new(6, 0, 0, 0, 0, 0),
                TowelNode::new(7, 0, 0, 0, 0, 0),
                TowelNode::new(8, 0, 0, 0, 0, 0),
            ]
        )
    }

    #[test]
    fn pattern_check_works() {
        let towels = Towels::parser().parse_value(EXAMPLE_1).unwrap();

        assert_eq!(towels.possible_patterns(&parse_pattern(b"brwrr")), 2);
        assert_eq!(towels.possible_patterns(&parse_pattern(b"bggr")), 1);
        assert_eq!(towels.possible_patterns(&parse_pattern(b"gbbr")), 4);
        assert_eq!(towels.possible_patterns(&parse_pattern(b"rrbgbr")), 6);
        assert_eq!(towels.possible_patterns(&parse_pattern(b"bwurrg")), 1);
        assert_eq!(towels.possible_patterns(&parse_pattern(b"brgr")), 2);
        assert_eq!(towels.possible_patterns(&parse_pattern(b"ubwu")), 0);
        assert_eq!(towels.possible_patterns(&parse_pattern(b"bbrgwb")), 0);
    }

    #[test]
    fn part_1_works_on_example() {
        let (towels, patterns) = parse(EXAMPLE_1);
        assert_eq!(part_1(&towels, &patterns), 6);
    }

    #[test]
    fn part_2_works_on_example() {
        let (towels, patterns) = parse(EXAMPLE_1);
        assert_eq!(part_2(&towels, &patterns), 16);
    }
}
