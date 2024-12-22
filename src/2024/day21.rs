use common::grid::Grid;
use common::runner::Runner;
use common::search::{bfs, Cost, Key, Order};
use rustc_hash::{FxHashMap, FxHashSet};

const UP: u8 = b'^';
const LEFT: u8 = b'<';
const RIGHT: u8 = b'>';
const DOWN: u8 = b'v';
const A: u8 = b'A';

const BUTTONS: [u8; 5] = [UP, LEFT, RIGHT, DOWN, A];

pub fn main(r: &mut Runner, input: &[u8]) {
    let codes = r.prep("Parse", || parse(input));

    r.part("Part 1 (Pathfinding)", || part_1_pathfinding(&codes));
    r.set_tail("Parse");
    r.part("Part 1 (Recursive)", || part_1_recursive(&codes));
    r.part("Part 2 (Recursive)", || part_2_recursive(&codes));
}

type Keypad = Grid<(u8, u8), [u8; 30], u8>;

fn part_1_pathfinding(codes: &[[u8; 4]]) -> u64 {
    codes
        .iter()
        .map(|code| run_pathfinding::<3>(code) * code_number(*code))
        .sum()
}

fn part_1_recursive(codes: &[[u8; 4]]) -> u64 {
    codes
        .iter()
        .map(|code| run_recursive::<3>(code) * code_number(*code))
        .sum()
}

fn part_2_recursive(codes: &[[u8; 4]]) -> u64 {
    codes
        .iter()
        .map(|code| run_recursive::<26>(code) * code_number(*code))
        .sum()
}

fn run_recursive<const ROBOTS: usize>(code: &[u8]) -> u64 {
    run_recursive_step::<ROBOTS>(0, code, &mut FxHashMap::default())
}

fn run_recursive_step<const ROBOTS: usize>(
    level: usize,
    code: &[u8],
    cache: &mut FxHashMap<(u8, u8, u8), u64>,
) -> u64 {
    if level == ROBOTS + 1 || code.len() == 0 {
        // The last one is you, where you just need one button press
        return 1;
    }

    let l = level as u8;
    let mut total = 0;
    let mut current = b'A';
    for next in code.iter() {
        let sub_codes = if level > 0 {
            arrow_paths(current, *next)
        } else {
            digit_paths(current, *next)
        };

        total += if let Some(cached_res) = cache.get(&(current, *next, l)).copied() {
            cached_res
        } else {
            let res = sub_codes
                .iter()
                .map(|sub_code| run_recursive_step::<ROBOTS>(level + 1, *sub_code, cache))
                .min()
                .unwrap();

            cache.insert((current, *next, l), res);
            res
        };

        current = *next;
    }

    total
}

fn digit_paths(a: u8, b: u8) -> &'static [&'static [u8]] {
    match &[a, b] {
        b"00" => &[b"A"],
        b"01" => &[b"^<A"],
        b"02" => &[b"^A"],
        b"03" => &[b"^>A", b">^A"],
        b"04" => &[b"^^<A"],
        b"05" => &[b"^^A"],
        b"06" => &[b"^^>A", b">^^A"],
        b"07" => &[b"^^^<A"],
        b"08" => &[b"^^^A"],
        b"09" => &[b"^^^>A", b">^^^A"],
        b"0A" => &[b">A"],
        b"10" => &[b">vA"],
        b"11" => &[b"A"],
        b"12" => &[b">A"],
        b"13" => &[b">>A"],
        b"14" => &[b"^A"],
        b"15" => &[b"^>A", b">^A"],
        b"16" => &[b"^>>A", b">>^A"],
        b"17" => &[b"^^A"],
        b"18" => &[b"^^>A", b">^^A"],
        b"19" => &[b"^^>>A", b">>^^A"],
        b"1A" => &[b">>vA"],
        b"20" => &[b"vA"],
        b"21" => &[b"<A"],
        b"22" => &[b"A"],
        b"23" => &[b">A"],
        b"24" => &[b"<^A"],
        b"25" => &[b"^A"],
        b"26" => &[b"^>A", b">^A"],
        b"27" => &[b"<^^A"],
        b"28" => &[b"^^A"],
        b"29" => &[b"^^>A", b">^^A"],
        b"2A" => &[b"v>A"],
        b"30" => &[b"<vA"],
        b"31" => &[b"<<A"],
        b"32" => &[b"<A"],
        b"33" => &[b"A"],
        b"34" => &[b"<<^A"],
        b"35" => &[b"<^A"],
        b"36" => &[b"^A"],
        b"37" => &[b"<<^^A"],
        b"38" => &[b"<^^A"],
        b"39" => &[b"^^A"],
        b"3A" => &[b"vA"],
        b"40" => &[b">vvA"],
        b"41" => &[b"vA"],
        b"42" => &[b"v>A"],
        b"43" => &[b"v>>A"],
        b"44" => &[b"A"],
        b"45" => &[b">A"],
        b"46" => &[b">>A"],
        b"47" => &[b"^A"],
        b"48" => &[b"^>A", b">^A"],
        b"49" => &[b"^>>A", b">>^A"],
        b"4A" => &[b">>vvA"],
        b"50" => &[b"vvA"],
        b"51" => &[b"<vA"],
        b"52" => &[b"vA"],
        b"53" => &[b"v>A"],
        b"54" => &[b"<A"],
        b"55" => &[b"A"],
        b"56" => &[b">A"],
        b"57" => &[b"<^A"],
        b"58" => &[b"^A"],
        b"59" => &[b"^>A", b">^A"],
        b"5A" => &[b"vv>A"],
        b"60" => &[b"<vvA"],
        b"61" => &[b"<<vA"],
        b"62" => &[b"<vA"],
        b"63" => &[b"vA"],
        b"64" => &[b"<<A"],
        b"65" => &[b"<A"],
        b"66" => &[b"A"],
        b"67" => &[b"<<^A"],
        b"68" => &[b"<^A"],
        b"69" => &[b"^A"],
        b"6A" => &[b"vvA"],
        b"70" => &[b">vvvA"],
        b"71" => &[b"vvA"],
        b"72" => &[b"vv>A"],
        b"73" => &[b"vv>>A"],
        b"74" => &[b"vA"],
        b"75" => &[b"v>A"],
        b"76" => &[b"v>>A"],
        b"77" => &[b"A"],
        b"78" => &[b">A"],
        b"79" => &[b">>A"],
        b"7A" => &[b">>vvvA"],
        b"80" => &[b"vvvA"],
        b"81" => &[b"<vvA"],
        b"82" => &[b"vvA"],
        b"83" => &[b"vv>A"],
        b"84" => &[b"<vA"],
        b"85" => &[b"vA"],
        b"86" => &[b"v>A"],
        b"87" => &[b"<A"],
        b"88" => &[b"A"],
        b"89" => &[b">A"],
        b"8A" => &[b"vvv>A"],
        b"90" => &[b"<vvvA"],
        b"91" => &[b"<<vvA"],
        b"92" => &[b"<vvA"],
        b"93" => &[b"vvA"],
        b"94" => &[b"<<vA"],
        b"95" => &[b"<vA"],
        b"96" => &[b"vA"],
        b"97" => &[b"<<A"],
        b"98" => &[b"<A"],
        b"99" => &[b"A"],
        b"9A" => &[b"vvvA"],
        b"A0" => &[b"<A"],
        b"A1" => &[b"^<<A"],
        b"A2" => &[b"<^A"],
        b"A3" => &[b"^A"],
        b"A4" => &[b"^^<<A"],
        b"A5" => &[b"<^^A"],
        b"A6" => &[b"^^A"],
        b"A7" => &[b"^^^<<A"],
        b"A8" => &[b"<^^^A"],
        b"A9" => &[b"^^^A"],
        b"AA" => &[b"A"],
        _ => panic!("{}{} not found", a as char, b as char),
    }
}

fn arrow_paths(a: u8, b: u8) -> &'static [&'static [u8]] {
    match &[a, b] {
        b"AA" => &[b"A"],
        b"A^" => &[b"<A"],
        b"A<" => &[b"v<<A"],
        b"Av" => &[b"<vA", b"v<A"],
        b"A>" => &[b"vA"],
        b"^^" => &[b"A"],
        b"^A" => &[b">A"],
        b"^<" => &[b"v<A"],
        b"^v" => &[b"vA"],
        b"^>" => &[b"v>A", b">vA"],
        b"<<" => &[b""],
        b"<^" => &[b">^A"],
        b"<A" => &[b">>^A"],
        b"<v" => &[b">A"],
        b"<>" => &[b">>A"],
        b"vv" => &[b"A"],
        b"v^" => &[b"^A"],
        b"vA" => &[b"^>A", b">^A"],
        b"v<" => &[b"<A"],
        b"v>" => &[b">A"],
        b">>" => &[b"A"],
        b">^" => &[b"^<A", b"<^A"],
        b">A" => &[b"^A"],
        b"><" => &[b"<<A"],
        b">v" => &[b"<A"],
        _ => panic!("{}{} not found", a as char, b as char),
    }
}

fn run_pathfinding<const ROBOTS: usize>(code: &[u8]) -> u64 {
    let mut total = 0;
    let mut current_pos = KEYPAD_DIGITS_START;
    let mut search = bfs().with_seen_space(FxHashSet::default());

    for i in 0..code.len() {
        search.reset();
        search.push(SearchState::<ROBOTS>::new(current_pos));

        let (pos, len) = search
            .find(|search, current| {
                for button in BUTTONS {
                    if let Some(next) = current.press(button) {
                        if next.code_len != current.code_len {
                            if code[i] != KEYPAD_DIGITS[next.positions[0]] {
                                continue;
                            }

                            return Some((current.positions[0], next.presses));
                        }

                        search.push(next);
                    }
                }

                None
            })
            .unwrap();

        total += len;
        current_pos = pos;
    }

    total
}

fn parse(input: &[u8]) -> Vec<[u8; 4]> {
    input
        .array_chunks::<5>()
        .map(|w| [w[0], w[1], w[2], w[3]])
        .collect()
}

fn code_number(code: [u8; 4]) -> u64 {
    ((code[0] - b'0') as u64 * 100) + ((code[1] - b'0') as u64 * 10) + (code[2] - b'0') as u64
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct SearchState<const ROBOTS: usize> {
    code_len: u8,
    presses: u64,
    positions: [(u8, u8); ROBOTS],
}

impl<const ROBOTS: usize> Cost<u64> for SearchState<ROBOTS> {
    fn cost(&self) -> u64 {
        self.presses
    }
}

impl<const ROBOTS: usize> Key<([(u8, u8); ROBOTS], u8)> for SearchState<ROBOTS> {
    fn key(&self) -> ([(u8, u8); ROBOTS], u8) {
        (self.positions, self.code_len)
    }
}

impl<const ROBOTS: usize> SearchState<ROBOTS> {
    fn new(init_pos: (u8, u8)) -> Self {
        let mut positions = [KEYPAD_ARROWS_START; ROBOTS];
        positions[0] = init_pos;

        Self {
            code_len: 0,
            presses: 0,
            positions,
        }
    }

    fn press(&self, button: u8) -> Option<Self> {
        if let Some(mut res) = self.press_at(ROBOTS - 1, button) {
            res.presses += 1;
            Some(res)
        } else {
            None
        }
    }

    fn press_at(&self, index: usize, button: u8) -> Option<Self> {
        let keypad = if index > 0 {
            KEYPAD_ARROWS
        } else {
            KEYPAD_DIGITS
        };

        if button == A {
            if index > 0 {
                self.press_at(index - 1, keypad[self.positions[index]])
            } else {
                let mut copy = *self;
                copy.code_len += 1;
                Some(copy)
            }
        } else {
            let mut copy = *self;
            copy.positions[index] = move_button(copy.positions[index], button);
            if keypad[copy.positions[index]] != b'.' {
                Some(copy)
            } else {
                None
            }
        }
    }
}

const KEYPAD_DIGITS: &Keypad = &Keypad::new_const((5, 6), *b"......789..456..123...0A......", b'.');
const KEYPAD_DIGITS_START: (u8, u8) = (3, 4);

const KEYPAD_ARROWS: &Keypad = &Keypad::new_const((5, 4), *b".......^A..<v>................", b'.');
const KEYPAD_ARROWS_START: (u8, u8) = (3, 1);

fn move_button((x, y): (u8, u8), button: u8) -> (u8, u8) {
    match button {
        UP => (x, y - 1),
        LEFT => (x - 1, y),
        RIGHT => (x + 1, y),
        DOWN => (x, y + 1),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"029A
980A
179A
456A
379A
";

    #[test]
    fn pressing_buttons_work() {
        let s = SearchState::<3>::new(KEYPAD_DIGITS_START);

        assert_eq!(s.press(RIGHT), None);
        assert_eq!(s.press(UP), None);
        assert_eq!(
            s.press(LEFT),
            Some(SearchState::<3> {
                presses: 1,
                code_len: 0,
                positions: [KEYPAD_DIGITS_START, KEYPAD_ARROWS_START, (2, 1)]
            })
        );
        assert_eq!(
            s.press(DOWN),
            Some(SearchState::<3> {
                presses: 1,
                code_len: 0,
                positions: [KEYPAD_DIGITS_START, KEYPAD_ARROWS_START, (3, 2)]
            })
        );

        let s2 = SearchState::<3> {
            presses: 1,
            code_len: 0,
            positions: [(2, 2), KEYPAD_ARROWS_START, KEYPAD_ARROWS_START],
        };
        assert_eq!(
            s2.press(A),
            Some(SearchState::<3> {
                presses: 2,
                code_len: 1,
                positions: [(2, 2), KEYPAD_ARROWS_START, KEYPAD_ARROWS_START]
            })
        );
    }

    #[test]
    fn code_number_works() {
        assert_eq!(code_number(*b"123A"), 123);
        assert_eq!(code_number(*b"007A"), 7);
        assert_eq!(code_number(*b"100A"), 100);
        assert_eq!(code_number(*b"060A"), 60);
    }

    #[test]
    fn parse_works_on_example() {
        assert_eq!(
            parse(EXAMPLE),
            vec![*b"029A", *b"980A", *b"179A", *b"456A", *b"379A",]
        )
    }

    #[test]
    fn part_1_pathfinder_works_on_example() {
        assert_eq!(part_1_pathfinding(&parse(EXAMPLE)), 126384)
    }

    #[test]
    fn part_1_recursive_works_on_example() {
        assert_eq!(part_1_recursive(&parse(EXAMPLE)), 126384)
    }

    #[test]
    fn pathfinder_works_on_part1_examples() {
        assert_eq!(run_pathfinding::<1>(b"029A"), 12);
        assert_eq!(run_pathfinding::<2>(b"029A"), 28);
        assert_eq!(run_pathfinding::<3>(b"029A"), 68);
        assert_eq!(run_pathfinding::<3>(b"980A"), 60);
        assert_eq!(run_pathfinding::<3>(b"179A"), 68);
        assert_eq!(run_pathfinding::<3>(b"456A"), 64);
        assert_eq!(run_pathfinding::<3>(b"379A"), 64);
    }

    #[test]
    fn recursive_works_on_part1_examples() {
        assert_eq!(run_recursive::<1>(b"029A"), 12);
        assert_eq!(run_recursive::<2>(b"029A"), 28);
        assert_eq!(run_recursive::<3>(b"029A"), 68);
        assert_eq!(run_recursive::<3>(b"980A"), 60);
        assert_eq!(run_recursive::<3>(b"179A"), 68);
        assert_eq!(run_recursive::<3>(b"456A"), 64);
        assert_eq!(run_recursive::<3>(b"379A"), 64);
    }
}

#[allow(dead_code)]
#[cfg(debug_assertions)]
mod utils {
    use crate::day21::KEYPAD_DIGITS;

    fn generate_paths() {
        const DIGITS: &[u8; 11] = b"0123456789A";

        let mut ud = String::with_capacity(8);
        let mut lr = String::with_capacity(8);

        for ci in DIGITS.iter().copied() {
            let (xi, yi) = KEYPAD_DIGITS
                .iter()
                .find(|(_, v)| **v == ci)
                .map(|(p, _)| p)
                .unwrap();
            let (xi2, yi2) = (xi as i32, yi as i32);
            for cj in DIGITS.iter().copied() {
                let (xj, yj) = KEYPAD_DIGITS
                    .iter()
                    .find(|(_, v)| **v == cj)
                    .map(|(p, _)| p)
                    .unwrap();
                let (xj2, yj2) = (xj as i32, yj as i32);

                let mut xdiff = xj2 - xi2;
                let mut ydiff = yj2 - yi2;

                lr.clear();
                ud.clear();
                while xdiff > 0 {
                    xdiff -= 1;
                    lr.push('>');
                }
                while xdiff < 0 {
                    xdiff += 1;
                    lr.push('<');
                }
                while ydiff > 0 {
                    ydiff -= 1;
                    ud.push('v');
                }
                while ydiff < 0 {
                    ydiff += 1;
                    ud.push('^');
                }

                let ci = ci as char;
                let cj = cj as char;

                if (ud.len() == 0 || lr.len() == 0) || does_panic((xi, yi), &lr, &ud) {
                    println!("b\"{ci}{cj}\" => &[b\"{ud}{lr}A\"],")
                } else if does_panic((xi, yi), &ud, &lr) {
                    println!("b\"{ci}{cj}\" => &[b\"{lr}{ud}A\"],")
                } else if lr.starts_with("<") {
                    println!("b\"{ci}{cj}\" => &[b\"{lr}{ud}A\"],")
                } else if ud.starts_with("v") {
                    println!("b\"{ci}{cj}\" => &[b\"{ud}{lr}A\"],")
                } else {
                    println!("b\"{ci}{cj}\" => &[b\"{ud}{lr}A\", b\"{lr}{ud}A\"],")
                }
            }
        }

        fn does_panic(from: (u8, u8), p1: &str, p2: &str) -> bool {
            let mut p = from;
            for path in [p1, p2] {
                for ch in path.chars() {
                    match ch {
                        '>' => p = (p.0 + 1, p.1),
                        '<' => p = (p.0 - 1, p.1),
                        '^' => p = (p.0, p.1 - 1),
                        'v' => p = (p.0, p.1 + 1),
                        _ => unreachable!(),
                    };

                    if KEYPAD_DIGITS[p] == b'.' {
                        return true;
                    }
                }
            }

            false
        }
    }
}
