use common::parser;
use common::parser::Parser;
use common::runner::Runner;

pub fn main(r: &mut Runner, input: &[u8]) {
    let (left_list, right_list) = r.prep("Parse + Sort", || parse_and_sort(&input));
    r.part("Part 1", || part_one(&left_list, &right_list));
    r.part("Part 2 (Iterators)", || part_two(&left_list, &right_list));
    r.set_tail("Part 1");
    r.part("Part 2 (Memoized Prev)", || {
        part_two_alt(&left_list, &right_list)
    });
    r.set_tail("Part 1");
    r.part("Part 2 (Huge Array)", || {
        part_two_alt2(&left_list, &right_list)
    });
    r.set_tail("Part 1");
    r.part("Part 2 (Combined Loop)", || part_two_alt3(&left_list, &right_list));
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
            *l * right_list[right_list.partition_point(|v| *v < *l)..]
                .iter()
                .take_while(|r| **r == *l)
                .count() as u32
        })
        .sum()
}

fn part_two_alt(left_list: &[u32], right_list: &[u32]) -> u32 {
    let mut total = 0;
    let mut prev = 0;
    let mut prev_result = 0;

    for r in right_list {
        if *r == prev {
            total += prev_result;
        } else {
            let result = left_list[left_list.partition_point(|v| *v < *r)..]
                .iter()
                .take_while(|l| **l == *r)
                .count() as u32
                * *r;

            total += result;
            prev = *r;
            prev_result = result;
        }
    }

    total
}

fn part_two_alt2(left_list: &[u32], right_list: &[u32]) -> u32 {
    let mut right_map = [0u8; 100000];
    for v in right_list.iter() {
        right_map[*v as usize] += 1;
    }

    left_list
        .iter()
        .map(|l| *l * right_map[*l as usize] as u32)
        .sum()
}

fn part_two_alt3(left_list: &[u32], right_list: &[u32]) -> u32 {
    let mut similarity_score = 0;
    let mut j = 0;
    for l in left_list.iter().copied() {
        while j < right_list.len() && right_list[j] < l {
            j += 1;
        }
        while j < right_list.len() && right_list[j] == l {
            similarity_score += l;
            j += 1;
        }
    }

    similarity_score
}

fn input_parser<'i>() -> impl Parser<'i, (Vec<u32>, Vec<u32>)> {
    parser::uint::<u32>()
        .and_discard(b"   ")
        .and(parser::uint::<u32>())
        .delimited_by(b'\n')
        .repeat_fold(
            || (Vec::with_capacity(64), Vec::with_capacity(64)),
            |(mut vl, mut vr), (cl, cr)| {
                vl.push(cl);
                vr.push(cr);
                (vl, vr)
            },
        )
}
