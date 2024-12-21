use common::grid::Grid;
use common::runner::Runner;
use common::search::{bfs, Cost, Key, Order};
use rustc_hash::FxHashSet;

const UP: u8 = b'^';
const LEFT: u8 = b'<';
const RIGHT: u8 = b'>';
const DOWN: u8 = b'v';
const A: u8 = b'A';

const BUTTONS: [u8; 5] = [UP, LEFT, RIGHT, DOWN, A];

pub fn main(r: &mut Runner, input: &[u8]) {
    let codes = r.prep("Parse", || parse(input));

    r.part("Part 1", || part_1(&codes));
    r.part("Part 2", || part_2(&codes));
}

type Keypad = Grid<(u8, u8), [u8; 30], u8>;

fn part_1(codes: &[[u8; 4]]) -> u64 {
    codes
        .iter()
        .map(|code| run_pathfinding::<3>(*code) * code_number(*code))
        .sum()
}

fn part_2(codes: &[[u8; 4]]) -> u64 {
    codes
        .iter()
        .map(|code| run_pathfinding::<26>(*code) * code_number(*code))
        .sum()
}

fn run_pathfinding<const ROBOTS: usize>(code: [u8; 4]) -> u64 {
    let mut search = bfs().with_seen_space(FxHashSet::default());
    search.push(SearchState::<ROBOTS>::new());

    search
        .find(|search, current| {
            for button in BUTTONS {
                if let Some(next) = current.press(button) {
                    if next.code_len != current.code_len {
                        let check_index = next.code_len as usize - 1;
                        if code[check_index] != KEYPAD_DIGITS[next.positions[0]] {
                            continue;
                        }

                        if next.code_len == 4 {
                            return Some(next.presses);
                        }
                    }

                    search.push(next);
                }
            }

            None
        })
        .unwrap()
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
    fn new() -> Self {
        let mut positions = [KEYPAD_ARROWS_START; ROBOTS];
        positions[0] = KEYPAD_DIGITS_START;

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
        let mut s = SearchState::<3>::new();

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
        assert_eq!(parse(EXAMPLE), vec![
            *b"029A",
            *b"980A",
            *b"179A",
            *b"456A",
            *b"379A",
        ])
    }

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(part_1(&parse(EXAMPLE)), 126384)
    }

    #[test]
    fn pathfinder_works_on_part1_examples() {
        assert_eq!(run_pathfinding::<1>(*b"029A"), 12);
        assert_eq!(run_pathfinding::<2>(*b"029A"), 28);
        assert_eq!(run_pathfinding::<3>(*b"029A"), 68);
        assert_eq!(run_pathfinding::<3>(*b"980A"), 60);
        assert_eq!(run_pathfinding::<3>(*b"179A"), 68);
        assert_eq!(run_pathfinding::<3>(*b"456A"), 64);
        assert_eq!(run_pathfinding::<3>(*b"379A"), 64);
    }
}
