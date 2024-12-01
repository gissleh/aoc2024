use common::parser;
use common::parser::Parser;
use common::runner::Runner;

pub fn main(r: &mut Runner, input: &[u8]) {
    let (left_list, right_list) = r.prep("Parse + Sort", || parse_and_sort(&input));
    r.part("Part 1", || part_one(&left_list, &right_list));
    r.part("Part 2", || part_two(&left_list, &right_list));
}

fn parse_and_sort(input: &[u8]) -> (Vec<u32>, Vec<u32>) {
    let (mut left_list, mut right_list) = input_parser().parse_value(input).unwrap();
    left_list.sort_unstable();
    right_list.sort_unstable();

    (left_list, right_list)
}

fn part_one(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| r.abs_diff(*l))
        .sum()
}

fn part_two(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
        .iter()
        .map(|l| {
            if let Ok(start_index) = right_list.binary_search(&l) {
                right_list[start_index..]
                    .iter()
                    .take_while(|r| **r == *l)
                    .chain(
                        right_list[..start_index]
                            .iter()
                            .rev()
                            .take_while(|r| **r == *l),
                    )
                    .count() as u32
                    * *l
            } else {
                0
            }
        })
        .sum()
}

fn input_parser<'i>() -> impl Parser<'i, (Vec<u32>, Vec<u32>)> {
    parser::uint::<u32>()
        .and_discard(b"   ".as_slice())
        .and(parser::uint::<u32>())
        .and_discard(b'\n')
        .repeat_fold(
            || (Vec::with_capacity(64), Vec::with_capacity(64)),
            |(mut vl, mut vr), (cl, cr)| {
                vl.push(cl);
                vr.push(cr);
                (vl, vr)
            },
        )
}
