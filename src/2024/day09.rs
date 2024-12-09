use common::runner::Runner;

const FREE_SPACE: u16 = u16::MAX;

pub fn main(r: &mut Runner, input: &[u8]) {
    let file = r.prep("Parse", || parse_file(input));
    r.part("Part 1", || part_1(&file));
    r.part("Part 2", || part_2(&file));
    r.info("Disk Size", &file.len());
    r.info(
        "Highest ID",
        &file
            .iter()
            .filter(|v| **v != FREE_SPACE)
            .max()
            .unwrap_or(&0),
    );

    r.start_over();
    let file = r.prep("Parse (Segment Map)", || parse_segments(input));
    r.part("Part 1 (Segment Map)", || part_1_segments(&file));
    r.part("Part 2 (Segment Map)", || part_2_segments(&file));
}

fn part_1(uncompacted_disk: &[u16]) -> u64 {
    let mut compacted_disk = uncompacted_disk.to_vec();
    let mut head = 0usize;
    let mut tail = compacted_disk.len() - 1;

    while head < tail {
        while compacted_disk[head] != FREE_SPACE && head < tail {
            head += 1;
        }
        while compacted_disk[tail] == FREE_SPACE && tail > 0 {
            tail -= 1;
            compacted_disk.pop();
        }

        compacted_disk[head] = compacted_disk[tail];
        compacted_disk.pop();
        tail -= 1;
    }

    compacted_disk
        .iter()
        .enumerate()
        .filter(|(_, id)| **id != FREE_SPACE)
        .map(|(position, id)| position as u64 * *id as u64)
        .sum()
}

fn part_2(uncompacted_disk: &[u16]) -> u64 {
    let mut compacted_disk = uncompacted_disk.to_vec();

    let mut head = 0usize;
    let mut tail = compacted_disk.len() - 1;

    while tail > 0 {
        while compacted_disk[head] != FREE_SPACE {
            head += 1;
        }
        while compacted_disk[tail] == FREE_SPACE {
            tail -= 1;
        }

        let file_id = compacted_disk[tail];
        let head_size = compacted_disk[head..]
            .iter()
            .take_while(|v| **v == FREE_SPACE)
            .count();
        let file_size = compacted_disk[..=tail]
            .iter()
            .rev()
            .take_while(|v| **v == file_id)
            .count();

        if head >= tail {
            head = 0;
            if tail > file_size {
                tail -= file_size;
            } else {
                tail = 0;
            }
            continue;
        }

        if head_size >= file_size {
            #[cfg(test)]
            println!("{file_id} moved {file_size} from {tail} to {head}");

            for i in 0..file_size {
                compacted_disk[head] = file_id;
                compacted_disk[tail - i] = FREE_SPACE;
                head += 1;
            }

            head = 0;
            tail -= file_size;
        } else {
            head += head_size;
        }
    }

    compacted_disk
        .iter()
        .enumerate()
        .filter(|(_, id)| **id != FREE_SPACE)
        .map(|(position, id)| position as u64 * *id as u64)
        .sum()
}

fn parse_file(input: &[u8]) -> Vec<u16> {
    let mut res = Vec::with_capacity(input.len() * 8);
    let mut current_id = 0u16;
    let mut is_free = false;
    for ch in input.iter() {
        if *ch == b'\n' {
            break;
        }

        let size = ch - b'0';
        if is_free {
            res.extend((0..size).map(|_| FREE_SPACE));
        } else {
            res.extend((0..size).map(|_| current_id));
            current_id += 1;
        }

        is_free = !is_free;
    }

    res
}

fn parse_segments(input: &[u8]) -> Vec<DiskSegment> {
    let mut res = Vec::with_capacity(input.len() * 2);
    let mut current_id = 0u16;
    let mut is_free = false;
    for ch in input.iter() {
        if *ch == b'\n' {
            break;
        }

        let size = ch - b'0';
        if is_free {
            res.push(DiskSegment::Free(size));
        } else {
            res.push(DiskSegment::File(current_id, size));
            current_id += 1;
        }

        is_free = !is_free;
    }

    res
}

fn part_1_segments(uncompacted_disk: &[DiskSegment]) -> u64 {
    let mut compacted_disk = uncompacted_disk.to_vec();

    let mut tail = uncompacted_disk.len() - 1;
    let mut head = 0usize;

    #[cfg(test)]
    println!("{compacted_disk:?}");

    while head < tail {
        while let DiskSegment::File(_, _) = compacted_disk[head] {
            head += 1;
        }
        while let DiskSegment::Free(_) = compacted_disk[tail] {
            tail -= 1;
        }

        if head >= tail {
            break;
        }

        if let DiskSegment::File(file_id, file_len) = compacted_disk[tail] {
            if let DiskSegment::Free(free_len) = compacted_disk[head] {
                #[cfg(test)]
                println!(
                    "tail={tail} ({:?}) head={head} ({:?})",
                    compacted_disk[tail], compacted_disk[head]
                );
                if file_len == free_len {
                    compacted_disk[head] = DiskSegment::File(file_id, file_len);
                    compacted_disk[tail] = DiskSegment::Free(free_len);
                } else if free_len > file_len {
                    compacted_disk[head] = DiskSegment::File(file_id, file_len);
                    compacted_disk[tail] = DiskSegment::Free(file_len);
                    compacted_disk.insert(head + 1, DiskSegment::Free(free_len - file_len));
                } else {
                    compacted_disk[head] = DiskSegment::File(file_id, free_len);
                    compacted_disk[tail] = DiskSegment::File(file_id, file_len - free_len);
                }
                #[cfg(test)]
                println!("{compacted_disk:?}");
            }
        }
    }

    DiskSegment::checksum(&compacted_disk)
}

fn part_2_segments(uncompacted_disk: &[crate::day09::DiskSegment]) -> u64 {
    let mut compacted_disk = uncompacted_disk.to_vec();
    let mut tail = compacted_disk.len() - 1;
    let mut last_head = 0;

    while tail > 0 {
        if let DiskSegment::File(file_id, file_len) = compacted_disk[tail] {
            let mut first = false;
            for head in last_head..tail {
                if let DiskSegment::Free(free_len) = compacted_disk[head] {
                    if !first {
                        last_head = head;
                        first = true;
                    }

                    if free_len == file_len {
                        compacted_disk[head] = DiskSegment::File(file_id, file_len);
                        compacted_disk[tail] = DiskSegment::Free(file_len);
                        break;
                    } else if free_len > file_len {
                        compacted_disk[head] = DiskSegment::File(file_id, file_len);
                        compacted_disk[tail] = DiskSegment::Free(file_len);
                        compacted_disk.insert(head + 1, DiskSegment::Free(free_len - file_len));
                        tail += 1;
                        break;
                    }
                }
            }
        }

        tail -= 1;
    }

    DiskSegment::checksum(&compacted_disk)
}

#[derive(Clone, Copy, Debug)]
enum DiskSegment {
    Free(u8),
    File(u16, u8),
}

impl DiskSegment {
    fn checksum(disk: &[DiskSegment]) -> u64 {
        disk.iter()
            .fold(
                (0usize, 0usize),
                |(position, total), segment| match segment {
                    DiskSegment::Free(l) => (position + *l as usize, total),
                    DiskSegment::File(file_id, l) => (
                        position + *l as usize,
                        total
                            + (position..(position + (*l as usize)))
                                .map(|p| p * (*file_id as usize))
                                .sum::<usize>(),
                    ),
                },
            )
            .1 as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LONG_EXAMPLE: &[u8] = b"2333133121414131402\n";

    #[test]
    fn part_2_works_on_examples() {
        assert_eq!(part_2(&parse_file(LONG_EXAMPLE)), 2858);
    }

    #[test]
    fn part_1_segment_works_on_examples() {
        assert_eq!(part_1_segments(&parse_segments(LONG_EXAMPLE)), 1928);
    }
}
