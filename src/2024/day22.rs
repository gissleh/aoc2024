use common::parser;
use common::parser::Parser;
use common::runner::Runner;
use rayon::prelude::*;

pub fn main(r: &mut Runner, input: &[u8]) {
    let numbers = r.prep("Parse", || parse(input));

    r.part("Part 1", || part_1(&numbers));
    r.part("Part 2", || part_2(&numbers));
    r.set_tail("Part 1");
    r.part("Part 2 (Parallel)", || part_2_rayon(&numbers));

    r.info("Numbers", &numbers.len());
}

fn part_1(numbers: &[i32]) -> u64 {
    numbers
        .par_iter()
        .map(|n| nth_secret(*n, 2000) as u64)
        .sum()
}

fn parse(input: &[u8]) -> Vec<i32> {
    parser::uint::<i32>()
        .and_discard(b'\n')
        .repeat()
        .parse_value(input)
        .unwrap()
}

fn next_secret(secret: i32) -> i32 {
    let secret = ((secret << 6) ^ secret) & 16777215;
    let secret = (secret >> 5) ^ secret;
    let secret = ((secret << 11) ^ secret) & 16777215;
    secret
}

fn nth_secret(secret: i32, n: i32) -> i32 {
    let mut secret = secret;
    for _ in 0..n {
        secret = next_secret(secret);
    }

    secret
}

fn part_2(numbers: &[i32]) -> u16 {
    let mut seen_seq = [0; 20 * 20 * 20 * 20];
    let mut seq_totals = [0; 20 * 20 * 20 * 20];

    for (i, number) in numbers.iter().enumerate() {
        let i = i as u8;

        for (price, seq) in SecretIterator(*number).sequences(2000) {
            let key = cache_key(seq);
            if seen_seq[key] != i {
                seen_seq[key] = i;
                seq_totals[key] += price as u16;
            }
        }
    }

    *seq_totals.iter().max().unwrap()
}

fn part_2_rayon(numbers: &[i32]) -> u16 {
    numbers
        .par_iter()
        .fold(
            || {
                (
                    1u8,
                    vec![0u8; 20 * 20 * 20 * 20],
                    vec![0u16; 20 * 20 * 20 * 20],
                )
            },
            |(i, mut seen_map, mut totals), number| {
                let i = if i == 0 { 255 } else { i };

                for (price, seq) in SecretIterator(*number).sequences(2000) {
                    let key = cache_key(seq);
                    if seen_map[key] != i {
                        seen_map[key] = i;
                        totals[key] += price as u16;
                    }
                }

                (i.wrapping_add(1), seen_map, totals)
            },
        )
        .map(|(.., prev)| prev)
        .reduce_with(|mut prev, totals| {
            for i in 0..prev.len() {
                prev[i] += totals[i];
            }

            prev
        })
        .unwrap()
        .iter()
        .max()
        .copied()
        .unwrap()
}

struct SecretIterator(i32);

impl SecretIterator {
    fn sequences(self, n: usize) -> impl Iterator<Item = (i32, [i8; 4])> {
        SecretIterator(self.0)
            .take(n + 1)
            .map_windows(|[a, b, c, d, e]| {
                let a = *a % 10;
                let b = *b % 10;
                let c = *c % 10;
                let d = *d % 10;
                let e = *e % 10;

                (
                    e,
                    [(b - a) as i8, (c - b) as i8, (d - c) as i8, (e - d) as i8],
                )
            })
    }
}

impl Iterator for SecretIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.0;
        self.0 = next_secret(self.0);
        Some(prev)
    }
}

fn cache_key([a, b, c, d]: [i8; 4]) -> usize {
    (((a as i32 + 10) * 20 * 20 * 20)
        + ((b as i32 + 10) * 20 * 20)
        + ((c as i32 + 10) * 20)
        + (d as i32 + 10)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_secret_works_on_examples() {
        assert_eq!(next_secret(123), 15887950);
        assert_eq!(next_secret(15887950), 16495136);
        assert_eq!(next_secret(16495136), 527345);
        assert_eq!(next_secret(527345), 704524);
    }

    #[test]
    fn nth_secret_doesnt_off_by_one() {
        assert_eq!(nth_secret(1, 2000), 8685429);
        assert_eq!(nth_secret(10, 2000), 4700978);
        assert_eq!(nth_secret(100, 2000), 15273692);
        assert_eq!(nth_secret(2024, 2000), 8667524);
    }

    #[test]
    fn secret_iterator_nth_works() {
        assert_eq!(SecretIterator(1).nth(2000), Some(8685429));
        assert_eq!(SecretIterator(2024).nth(2000), Some(8667524));
    }

    #[test]
    fn secret_iterator_sequences_works() {
        assert_eq!(
            SecretIterator(123).sequences(2000).nth(0),
            Some((4, [-3, 6, -1, -1]))
        );
        assert_eq!(
            SecretIterator(123).sequences(2000).nth(1),
            Some((4, [6, -1, -1, 0]))
        );
        assert_eq!(
            SecretIterator(123).sequences(2000).nth(2),
            Some((6, [-1, -1, 0, 2]))
        );
    }
}
