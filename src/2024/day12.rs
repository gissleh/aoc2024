use common::grid::Grid;
use common::point::{CardinalNeighbors, XNeighbors};
use common::runner::{Runner, WithExtra};
use common::search;
use common::search::{Order, KE};
use rustc_hash::FxHashSet;

type FarmGrid = Grid<(u8, u8), Vec<u8>, u8>;
type UniqueFarmGrid = Grid<(u8, u8), Vec<u16>, u16>;

pub fn main(r: &mut Runner, input: &[u8]) {
    let grid = r.prep("Parse", || parse(input));
    let WithExtra(_, grid) = r.part("Part 1", || part_1(&grid));
    r.part("Part 2", || part_2(&grid));
}

fn part_1(farm: &FarmGrid) -> WithExtra<u32, UniqueFarmGrid> {
    let mut new_grid = UniqueFarmGrid::new_vec(*farm.size());
    let mut search = search::dfs().with_seen_space(FxHashSet::default());
    let mut price = 0;
    let mut index = 0;

    for y in 1..farm.size().1 - 1 {
        for x in 1..farm.size().0 - 1 {
            if search.push(KE((x, y), farm[(x, y)])) {
                index += 1;
            }

            let (area, perimeters, next_new_grid) = search.fold(
                (0, 0, new_grid),
                |search, KE(pos, curr)| {
                    let mut perimeters = 4;

                    for neigh in pos.cardinal_neighbors() {
                        let next = farm[neigh];
                        if next == curr {
                            search.push(KE(neigh, next));
                            perimeters -= 1;
                        }
                    }

                    Some((pos, perimeters))
                },
                |(a, p, mut g), (pos, perimeters)| {
                    g[pos] = index;
                    (a + 1, p + perimeters, g)
                },
            );

            new_grid = next_new_grid;

            price += area * perimeters;
        }
    }

    WithExtra(price, new_grid)
}

fn part_2(farm: &UniqueFarmGrid) -> u32 {
    #[cfg(test)]
    println!("--");

    const L_CORNERS: &[(u8, u8)] = &[
        (0b0011, 0b0001),
        (0b0101, 0b0010),
        (0b1010, 0b0100),
        (0b1100, 0b1000),
    ];

    const T_INTERSECTIONS: &[(u8, u8)] = &[
        (0b1011, 0b0101),
        (0b0111, 0b0011),
        (0b1101, 0b1010),
        (0b1110, 0b1100),
    ];

    let mut search = search::dfs().with_seen_space(FxHashSet::default());
    let mut price = 0;

    for y in 1..farm.size().1 - 1 {
        for x in 1..farm.size().0 - 1 {
            search.push(KE((x, y), farm[(x, y)]));
            let (area, perimeters) = search.fold(
                (0, 0),
                |search, KE(pos, curr)| {
                    let mut corners = 0u8;
                    let mut sides = 0u8;

                    for (i, neigh) in pos.cardinal_neighbors().iter().enumerate() {
                        let next = farm[*neigh];
                        if next == curr {
                            search.push(KE(*neigh, next));
                            sides |= 1 << i;
                        }
                    }

                    for (i, neigh) in pos.x_neighbors().iter().enumerate() {
                        let next = farm[*neigh];
                        if next == curr {
                            corners |= 1 << i;
                        }
                    }

                    if corners == 0 && sides == 0 {
                        return Some(4)
                    } else if sides.count_ones() > 3 {
                        return Some(4 - corners.count_ones())
                    } else if sides.count_ones() == 1 {
                        return Some(2)
                    } else if let Some((_, inner_mask)) = L_CORNERS.iter().find(|(sides2, _)| *sides2 == sides) {
                        return if corners & *inner_mask == 0 {
                            Some(2)
                        } else {
                            Some(1)
                        }
                    } else if let Some((_, inner_mask)) = T_INTERSECTIONS.iter().find(|(sides2, _)| *sides2 == sides) {
                        Some((!corners & *inner_mask).count_ones())
                    } else {
                        Some(0)
                    }
                },
                |(a, p), perimeters| (a + 1, p + perimeters),
            );

            if area > 0 {
                #[cfg(test)]
                println!("{}: a={area} s={perimeters}", farm[(x, y)]);

                price += area * perimeters;
            }
        }
    }

    price
}

fn parse(input: &[u8]) -> FarmGrid {
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let height = input.len() / (width + 1);

    let mut grid = Grid::new_with_default(
        ((width + 2) as u8, (height + 2) as u8),
        vec![255; (width + 2) * (height + 2)],
        255,
    );

    let mut pos = (1, 1);
    for v in input.iter() {
        if v == &b'\n' {
            pos = (1, pos.1 + 1);
            continue;
        }

        grid[pos] = *v - b'A';
        pos.0 += 1;
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1A: &[u8] = b"AAAA
BBCD
BBCC
EEEC
";

    const EXAMPLE_2A: &[u8] = b"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

    const EXAMPLE_2B: &[u8] = b"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

    #[test]
    fn part_2_works_on_examples() {
        assert_eq!(part_2(&part_1(&parse(EXAMPLE_1A)).1), 80);
        assert_eq!(part_2(&part_1(&parse(EXAMPLE_2A)).1), 236);
        assert_eq!(part_2(&part_1(&parse(EXAMPLE_2B)).1), 368);
    }
}
