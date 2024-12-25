use common::parser;
use common::parser::Parser;
use common::runner::Runner;

pub fn main(r: &mut Runner, input: &[u8]) {
    let (locks, keys) = r.prep("Parse", || parse(input));

    r.part("Part 1", || part_1(&locks, &keys));

    r.info("Locks", &locks.len());
    r.info("Keys", &keys.len());
}

fn part_1(locks: &[[u8; 5]], keys: &[[u8; 5]]) -> usize {
    keys.iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| {
                    key.iter()
                        .zip(lock.iter())
                        .find(|(key_pin, lock_pin)| **key_pin + **lock_pin > 5)
                        .is_none()
                })
                .count()
        })
        .sum()
}

fn parse(input: &[u8]) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    b"#####\n"
        .map(|_| true)
        .or(b".....\n".map(|_| false))
        .and(parser::line().repeat_fold_mut(
            || [0u8; 5],
            |current, line| {
                for i in 0..5 {
                    if line[i] == b'#' {
                        current[i] += 1;
                    }
                }
            },
        ))
        .and_skip(b'\n')
        .repeat_fold_mut(
            || (Vec::with_capacity(64), Vec::with_capacity(64)),
            |(locks, keys), (is_lock, pins)| {
                if is_lock {
                    locks.push(pins);
                } else {
                    keys.push(pins.map(|v| v - 1))
                }
            },
        )
        .parse_value(input)
        .unwrap()
}
