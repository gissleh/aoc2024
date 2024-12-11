use common::parser;
use common::parser::Parser;
use common::runner::Runner;
use rustc_hash::{FxBuildHasher, FxHashMap};

pub fn main(r: &mut Runner, input: &[u8]) {
    let pebbles = r.prep("Parse", || parser().parse_value(input).unwrap());

    r.part("Part 1 (Brute)", || part_brute(&pebbles, 25));
    r.set_tail("Parse");
    r.part("Part 1 (DP)", || part_dp(&pebbles, 25));
    r.part("Part 2 (DP)", || part_dp(&pebbles, 75));
    r.connect("Part 1 (Brute)", "Part 2 (DP)");
}

fn parser<'i>() -> impl Parser<'i, Vec<u64>> {
    parser::uint::<u64>().delimited_by(b' ').repeat()
}

fn part_brute(pebbles: &[u64], times: u32) -> usize {
    let mut pebbles = Pebbles::from(pebbles);
    for _ in 0..times {
        pebbles.run();
    }

    pebbles.data.len()
}

fn part_dp(pebbles: &[u64], times: u32) -> u64 {
    let mut total = 0;
    let mut cache =
        FxHashMap::with_capacity_and_hasher(pebbles.len() * 128, FxBuildHasher::default());

    for pebble in pebbles.iter() {
        total += count_pebbles(*pebble, times, &mut cache);
    }

    total
}

fn count_pebbles(pebble: u64, remaining: u32, cache: &mut FxHashMap<(u64, u32), u64>) -> u64 {
    if remaining == 0 {
        return 1;
    }

    if remaining > 3 {
        if let Some(count) = cache.get(&(pebble, remaining)) {
            return *count;
        }
    }

    let next_r = remaining - 1;
    let res = match pebble {
        0 => count_pebbles(1, next_r, cache),
        10..=99 => {
            count_pebbles(pebble / 10, next_r, cache) + count_pebbles(pebble % 10, next_r, cache)
        }
        1000..=9999 => {
            count_pebbles(pebble / 100, next_r, cache) + count_pebbles(pebble % 100, next_r, cache)
        }
        100000..=999999 => {
            count_pebbles(pebble / 1_000, next_r, cache)
                + count_pebbles(pebble % 1_000, next_r, cache)
        }
        10000000..=99999999 => {
            count_pebbles(pebble / 10_000, next_r, cache)
                + count_pebbles(pebble % 10_000, next_r, cache)
        }
        1000000000..=9999999999 => {
            count_pebbles(pebble / 100_000, next_r, cache)
                + count_pebbles(pebble % 100_000, next_r, cache)
        }
        100000000000..=999999999999 => {
            count_pebbles(pebble / 1_000_000, next_r, cache)
                + count_pebbles(pebble % 1_000_000, next_r, cache)
        }
        10000000000000..=99999999999999 => {
            count_pebbles(pebble / 10_000_000, next_r, cache)
                + count_pebbles(pebble % 10_000_000, next_r, cache)
        }
        1000000000000000..=9999999999999999 => {
            count_pebbles(pebble / 100_000_000, next_r, cache)
                + count_pebbles(pebble % 100_000_000, next_r, cache)
        }
        100000000000000000.. => unreachable!(),
        _ => count_pebbles(pebble * 2024, next_r, cache),
    };

    if remaining > 3 {
        cache.insert((pebble, remaining), res);
    }

    res
}

struct Pebbles {
    data: Vec<Pebble>,
}

impl Pebbles {
    fn run(&mut self) {
        let mut current = 0;
        loop {
            let Pebble { number, next } = self.data[current];
            match number {
                0 => self.data[current].number = 1,
                10..=99 => self.split_at(current, number / 10, number % 10),
                1000..=9999 => self.split_at(current, number / 100, number % 100),
                100000..=999999 => self.split_at(current, number / 1_000, number % 1_000),
                10000000..=99999999 => self.split_at(current, number / 10_000, number % 10_000),
                1000000000..=9999999999 => {
                    self.split_at(current, number / 100_000, number % 100_000)
                }
                100000000000..=999999999999 => {
                    self.split_at(current, number / 1_000_000, number % 1_000_000)
                }
                10000000000000..=99999999999999 => {
                    self.split_at(current, number / 10_000_000, number % 10_000_000)
                }
                1000000000000000..=9999999999999999 => {
                    self.split_at(current, number / 100_000_000, number % 100_000_000)
                }
                100000000000000000.. => unreachable!(),
                _ => self.data[current].number *= 2024,
            }

            current = next;
            if current == 0 {
                break;
            }
        }

        #[cfg(test)]
        if self.data.len() < 25 {
            let mut current = 0;

            loop {
                let Pebble { number, next } = self.data[current];

                print!("{} ", number);

                current = next;
                if current == 0 {
                    break;
                }
            }

            println!();
        }
    }

    fn split_at(&mut self, index: usize, a: u64, b: u64) {
        self.data[index].number = a;
        self.insert_after(index, b);
    }

    fn insert_after(&mut self, index: usize, number: u64) {
        self.data.push(Pebble {
            number,
            next: self.data[index].next,
        });
        self.data[index].next = self.data.len() - 1;
    }

    fn from(initial_numbers: &[u64]) -> Self {
        let mut data = Vec::with_capacity(initial_numbers.len());
        for (i, number) in initial_numbers.iter().enumerate() {
            data.push(Pebble {
                number: *number,
                next: i + 1,
            });
        }
        data.last_mut().unwrap().next = 0;

        Self { data }
    }
}

#[derive(Debug, Copy, Clone)]
struct Pebble {
    number: u64,
    next: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"125 17\n";

    #[test]
    fn part_1_dp_brute_works_on_example() {
        assert_eq!(part_brute(&parser().parse_value(EXAMPLE).unwrap(), 6), 22);
        assert_eq!(
            part_brute(&parser().parse_value(EXAMPLE).unwrap(), 25),
            55312
        );
    }

    #[test]
    fn dp_tests() {
        assert_eq!(part_dp(&[0], 1), 1);
        assert_eq!(part_dp(&[0], 2), 1);
        assert_eq!(part_dp(&[0], 3), 2);
    }

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(part_dp(&parser().parse_value(EXAMPLE).unwrap(), 1), 3);
        assert_eq!(part_dp(&parser().parse_value(EXAMPLE).unwrap(), 2), 4);
        assert_eq!(part_dp(&parser().parse_value(EXAMPLE).unwrap(), 3), 5);
        assert_eq!(part_dp(&parser().parse_value(EXAMPLE).unwrap(), 4), 9);
        assert_eq!(part_dp(&parser().parse_value(EXAMPLE).unwrap(), 5), 13);
        assert_eq!(part_dp(&parser().parse_value(EXAMPLE).unwrap(), 6), 22);
        assert_eq!(part_dp(&parser().parse_value(EXAMPLE).unwrap(), 25), 55312);
    }
}
