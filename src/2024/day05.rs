use arrayvec::ArrayVec;
use common::constants::U128_BITS;
use common::parser;
use common::parser::{digit, Parser};
use common::runner::{BothParts, Runner};
use std::cmp::Ordering;

pub fn main(r: &mut Runner, input: &[u8]) {
    let (page_ordering, input) = r.prep("Parse Ordering", || {
        PageOrdering::parser().parse(input).unwrap()
    });
    let version_updates = r.prep("Parse Version Updates", || {
        VersionUpdate::list_parser().parse_value(input).unwrap()
    });

    r.part("Part 1", || part_1(&version_updates, &page_ordering));
    r.part("Part 2", || part_2(&version_updates, &page_ordering));

    r.set_tail("Parse Version Updates");
    r.part("Both Parts Combined", || {
        both_parts(&version_updates, &page_ordering)
    });

    r.info("Orderings", &page_ordering.size());
    r.info("Version Updates", &version_updates.len());
}

fn part_1(version_updates: &[VersionUpdate], page_ordering: &PageOrdering) -> u32 {
    version_updates
        .iter()
        .filter(|v| v.is_sorted(&page_ordering))
        .map(|v| v.middle_number() as u32)
        .sum()
}

fn part_2(version_updates: &[VersionUpdate], page_ordering: &PageOrdering) -> u32 {
    version_updates
        .iter()
        .filter(|v| !v.is_sorted(&page_ordering))
        .map(|v| v.sorted(&page_ordering).middle_number() as u32)
        .sum()
}

fn both_parts(
    version_updates: &[VersionUpdate],
    page_ordering: &PageOrdering,
) -> BothParts<u32, u32> {
    version_updates.iter().fold(BothParts(0u32, 0u32), |r, p| {
        if p.is_sorted(&page_ordering) {
            BothParts(r.0 + p.middle_number() as u32, r.1)
        } else {
            BothParts(r.0, r.1 + p.sorted(&page_ordering).middle_number() as u32)
        }
    })
}

struct PageOrdering([u128; 100]);

impl PageOrdering {
    fn is_before(&self, pa: u8, pb: u8) -> bool {
        self.0[pa as usize] & U128_BITS[pb as usize] != 0
    }

    fn size(&self) -> u32 {
        self.0.iter().map(|v| v.count_ones()).sum()
    }

    #[inline]
    fn parser<'i>() -> impl Parser<'i, Self> {
        parser::uint::<usize>()
            .and_discard(b'|')
            .and(parser::uint::<u8>())
            .delimited_by(b'\n')
            .repeat_fold_mut(
                || PageOrdering([0; 100]),
                |c, (a, b)| {
                    c.0[a] |= 1 << b;
                },
            )
            .and_discard(b"\n\n")
    }
}

struct VersionUpdate(ArrayVec<u8, 24>);

impl VersionUpdate {
    fn is_sorted(&self, ordering: &PageOrdering) -> bool {
        self.0.is_sorted_by(|a, b| ordering.is_before(*a, *b))
    }

    fn sorted(&self, ordering: &PageOrdering) -> Self {
        let mut new_list = self.0.clone();

        new_list.sort_by(|a, b| {
            if ordering.is_before(*a, *b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        Self(new_list)
    }

    fn middle_number(&self) -> u8 {
        self.0[self.0.len() / 2]
    }

    fn parser<'i>() -> impl Parser<'i, Self> {
        two_digits()
            .delimited_by(b',')
            .repeat_limited(1, 24)
            .map(|l| VersionUpdate(l))
    }

    fn list_parser<'i>() -> impl Parser<'i, Vec<Self>> {
        Self::parser().delimited_by(b'\n').repeat()
    }
}

#[inline]
fn two_digits<'i>() -> impl Parser<'i, u8> {
    digit().and(digit()).map(|(d1, d2)| d1 * 10 + d2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn parse_works() {
        let (page_ordering, input) = PageOrdering::parser().parse(EXAMPLE).unwrap();
        let (version_updates, _) = VersionUpdate::list_parser().parse(input).unwrap();

        assert_eq!(page_ordering.size(), 21);
        assert_eq!(version_updates.len(), 6);
    }

    #[test]
    fn part_1_works_on_example() {
        let (page_ordering, input) = PageOrdering::parser().parse(EXAMPLE).unwrap();
        let (version_updates, _) = VersionUpdate::list_parser().parse(input).unwrap();
        assert_eq!(part_1(&version_updates, &page_ordering), 143);
    }

    #[test]
    fn part_2_works_on_example() {
        let (page_ordering, input) = PageOrdering::parser().parse(EXAMPLE).unwrap();
        let (version_updates, _) = VersionUpdate::list_parser().parse(input).unwrap();
        assert_eq!(part_2(&version_updates, &page_ordering), 123);
    }

    #[test]
    fn both_parts_works_on_example() {
        let (page_ordering, input) = PageOrdering::parser().parse(EXAMPLE).unwrap();
        let (version_updates, _) = VersionUpdate::list_parser().parse(input).unwrap();
        assert_eq!(
            both_parts(&version_updates, &page_ordering),
            BothParts(143, 123)
        );
    }
}
