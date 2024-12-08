use arrayvec::ArrayVec;
use rustc_hash::FxHashSet;
use common::runner::Runner;
use common::grid::GridCoordinate;

pub fn main(r: &mut Runner, input: &[u8]) {
    let antenna_amp = r.prep("Parse", || AntennaMap::parse(input));

    r.part("Part 1", || part_1(&antenna_amp));
    r.part("Part 2", || part_2(&antenna_amp));

    r.info("Antennas", &antenna_amp.positions_by_key.iter().map(|a| a.len()).sum::<usize>());
    r.info("0 Antennas", &format!("{:?}", antenna_amp.positions_by_key[0]));
}

fn part_1(antenna_map: &AntennaMap) -> usize {
    let mut set = FxHashSet::with_capacity_and_hasher(256, Default::default());
    for antennas in antenna_map.positions_by_key.iter() {
        for i in 0..antennas.len() {
            let a = antennas[i];
            for j in 0..antennas.len() {
                if i == j {
                    continue
                }
                let b = antennas[j];

                let ab = (b.0 + (b.0 - a.0), b.1 + (b.1 - a.1));
                if ab.in_bounds(&antenna_map.size) && !antennas.contains(&ab) {
                    #[cfg(test)]
                    println!("Anitenode at {ab:?} ({a:?} and {b:?})");

                    set.insert(ab);
                }
            }
        }
    }

    set.iter().count()
}

fn part_2(antenna_map: &AntennaMap) -> usize {
    let mut set = FxHashSet::with_capacity_and_hasher(256, Default::default());
    for antennas in antenna_map.positions_by_key.iter() {
        for i in 0..antennas.len() {
            let a = antennas[i];
            for j in 0..antennas.len() {
                if i == j {
                    continue
                }
                let b = antennas[j];

                let mut ab = b;
                while ab.in_bounds(&antenna_map.size) {
                    #[cfg(test)]
                    println!("Anitenode at {ab:?} ({a:?} and {b:?})");

                    set.insert(ab);

                    ab.0 += b.0 - a.0;
                    ab.1 += b.1 - a.1;
                }
            }
        }
    }

    set.iter().count()
}

struct AntennaMap {
    positions_by_key: [ArrayVec<(i16, i16), 8>; 62],
    size: (i16, i16),
}

impl AntennaMap {
    fn parse(input: &[u8]) -> Self {
        let width = input.iter().position(|&c| c == b'\n').unwrap();
        let height = input.len() / (width + 1);
        let size = (width as i16, height as i16);
        let mut curr = (0, 0);

        let mut res = Self{
            size,
            positions_by_key: std::array::from_fn(|_| ArrayVec::new_const())
        };

        for c in input.iter() {
            match *c {
                b'\n' => { continue }
                b'.' => { }
                b'0'..=b'9' => { res.positions_by_key[(*c - b'0') as usize].push(curr); }
                b'a'..=b'z' => { res.positions_by_key[((*c - b'a') + 10) as usize].push(curr); }
                b'A'..=b'Z' => { res.positions_by_key[((*c - b'A') + 36) as usize].push(curr); }
                _ => { unreachable!(); }
            }

            curr = curr.next(&size)
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1A: &[u8] = b"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........
";

    const EXAMPLE_1B: &[u8] = b"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........
";

    const EXAMPLE_1C: &[u8] = b"..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........
";

    const EXAMPLE_1D: &[u8] = b"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    const EXAMPLE_2A: &[u8] = b"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
";

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(part_1(&AntennaMap::parse(EXAMPLE_1B)), 4);
        assert_eq!(part_1(&AntennaMap::parse(EXAMPLE_1A)), 2);
        assert_eq!(part_1(&AntennaMap::parse(EXAMPLE_1C)), 4);
        assert_eq!(part_1(&AntennaMap::parse(EXAMPLE_1D)), 14);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(part_2(&AntennaMap::parse(EXAMPLE_2A)), 9);
        assert_eq!(part_2(&AntennaMap::parse(EXAMPLE_1D)), 34);
    }
}