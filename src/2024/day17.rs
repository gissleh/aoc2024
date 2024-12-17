use common::parser;
use common::parser::Parser;
use common::runner::Runner;
use num::range_step;
use std::fmt::Write;

const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

pub fn main(r: &mut Runner, input: &[u8]) {
    let (program, registers) = r.prep("Parse", || parser().parse_value(input).unwrap());

    r.part("Part 1", || part_1(&program, registers));
    r.part("Part 2", || part_2(&program, registers));

    r.info("Program", &print_program(&program));
}

fn part_1(program: &[u8], registers: [u64; 3]) -> String {
    let (res, _) = run_program(program, registers);

    let mut res_str = String::with_capacity(res.len() * 2);
    for b in res.into_iter() {
        if !res_str.is_empty() {
            res_str.push(',');
        }

        res_str.push((b + b'0') as char);
    }

    res_str
}

fn part_2(program: &[u8], registers: [u64; 3]) -> u64 {
    let mut output_buffer = Vec::with_capacity(program.len());
    part_2_step(
        8u64.pow(program.len() as u32) - 1,
        8u64.pow(program.len() as u32 - 1),
        program.len() - 1,
        program,
        registers,
        &mut output_buffer,
    )
    .unwrap()
}

fn part_2_step(
    a: u64,
    step_size: u64,
    probe_start: usize,
    program: &[u8],
    registers: [u64; 3],
    output_buffer: &mut Vec<u8>,
) -> Option<u64> {
    let mut lowest = None;
    let mut registers = registers;

    for n in 0..8 {
        let a = a - (step_size * n);
        registers[0] = a;
        run_program_with(program, registers, output_buffer);

        if program[probe_start..] == output_buffer[probe_start..] {
            if probe_start == 0 {
                lowest = lowest_of(lowest, a);
            } else if let Some(result) = part_2_step(
                a,
                step_size / 8,
                probe_start - 1,
                program,
                registers,
                output_buffer,
            ) {
                lowest = lowest_of(lowest, result);
            }
        }
    }

    lowest
}

fn parser<'i>() -> impl Parser<'i, (Vec<u8>, [u64; 3])> {
    b"Register A: "
        .and_instead(parser::uint::<u64>())
        .and_discard(b"\nRegister B: ")
        .and(parser::uint::<u64>())
        .and_discard(b"\nRegister C: ")
        .and(parser::uint::<u64>())
        .and_discard(b"\n\nProgram: ")
        .and(parser::digit().delimited_by(b',').repeat::<Vec<_>>())
        .map(|(((a, b), c), program)| (program, [a, b, c]))
}

fn run_program(program: &[u8], registers: [u64; 3]) -> (Vec<u8>, [u64; 3]) {
    let mut output = Vec::with_capacity(8);
    let registers = run_program_with(program, registers, &mut output);

    (output, registers)
}

fn run_program_with(program: &[u8], registers: [u64; 3], output: &mut Vec<u8>) -> [u64; 3] {
    let mut registers = registers;
    let mut pc = 0;

    output.clear();

    while pc < program.len() {
        match program[pc] {
            ADV => {
                registers[0] =
                    registers[0] / 2_u64.pow(combo_operand(program[pc + 1], registers) as u32);
                pc += 2;
            }
            BXL => {
                registers[1] ^= program[pc + 1] as u64;
                pc += 2;
            }
            BST => {
                registers[1] = combo_operand(program[pc + 1], registers) % 8;
                pc += 2;
            }
            JNZ => {
                if registers[0] != 0 {
                    pc = program[pc + 1] as usize;
                } else {
                    pc += 2;
                }
            }
            BXC => {
                registers[1] ^= registers[2];
                pc += 2;
            }
            OUT => {
                output.push((combo_operand(program[pc + 1], registers) % 8) as u8);
                pc += 2;
            }
            BDV => {
                registers[1] =
                    registers[0] / 2_u64.pow(combo_operand(program[pc + 1], registers) as u32);
                pc += 2;
            }
            CDV => {
                registers[2] =
                    registers[0] / 2_u64.pow(combo_operand(program[pc + 1], registers) as u32);
                pc += 2;
            }
            _ => unreachable!(),
        }
    }

    registers
}

fn print_program(program: &[u8]) -> String {
    let mut res = String::with_capacity(1024);
    for pc in range_step(0, program.len(), 2) {
        match program[pc] {
            ADV => {
                writeln!(res, "{pc} ADV {}", combo_operand_name(program[pc + 1])).unwrap();
            }
            BXL => {
                writeln!(res, "{pc} BXL {}", real_operand_name(program[pc + 1])).unwrap();
            }
            BST => {
                writeln!(res, "{pc} BST {}", combo_operand_name(program[pc + 1])).unwrap();
            }
            JNZ => {
                writeln!(res, "{pc} JNZ {}", real_operand_name(program[pc + 1])).unwrap();
            }
            BXC => {
                writeln!(res, "{pc} BXC").unwrap();
            }
            OUT => {
                writeln!(res, "{pc} OUT {}", combo_operand_name(program[pc + 1])).unwrap();
            }
            BDV => {
                writeln!(res, "{pc} BDV {}", combo_operand_name(program[pc + 1])).unwrap();
            }
            CDV => {
                writeln!(res, "{pc} CDV {}", combo_operand_name(program[pc + 1])).unwrap();
            }
            _ => unreachable!(),
        }
    }

    res
}

fn combo_operand(code: u8, registers: [u64; 3]) -> u64 {
    if code < 4 {
        code as u64
    } else {
        registers[(code - 4) as usize]
    }
}

fn combo_operand_name(code: u8) -> char {
    if code < 4 {
        (code + b'0') as char
    } else {
        [b'a', b'b', b'c', b'X'][code as usize - 4] as char
    }
}

fn real_operand_name(code: u8) -> char {
    (code + b'0') as char
}

fn lowest_of<T: Ord>(old: Option<T>, new: T) -> Option<T> {
    match old {
        Some(old) => {
            if new < old {
                Some(new)
            } else {
                Some(old)
            }
        }
        None => Some(new),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &[u8] = b"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const EXAMPLE_2: &[u8] = b"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn toy_programs_work_as_expected() {
        assert_eq!(run_program(&[2, 6], [0, 0, 9]).1[1], 1);
        assert_eq!(
            run_program(&[5, 0, 5, 1, 5, 4], [10, 0, 0]).0,
            vec![0, 1, 2]
        );
        assert_eq!(run_program(&[1, 7], [0, 29, 0]).1[1], 26);
        assert_eq!(run_program(&[4, 0], [0, 2024, 43690]).1[1], 44354);

        assert_eq!(
            run_program(&[0, 1, 5, 4, 3, 0], [2024, 0, 0]).0,
            vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
        );
        assert_eq!(run_program(&[0, 1, 5, 4, 3, 0], [2024, 0, 0]).1[0], 0);
    }

    #[test]
    fn part_1_works_on_example() {
        let (program, registers) = parser().parse_value(EXAMPLE_1).unwrap();
        println!("{}", print_program(&program));
        assert_eq!(part_1(&program, registers), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_2_works_on_example() {
        let (program, registers) = parser().parse_value(EXAMPLE_2).unwrap();
        assert_eq!(run_program(&program, [0, 0, 0]).0, vec![0]);
        assert_eq!(run_program(&program, [8, 0, 0]).0, vec![1, 0]);
        assert_eq!(
            run_program(&program, [(4 * 8 * 8) + 3 * 8, 0, 0]).0,
            vec![3, 4, 0]
        );

        assert_eq!(part_2(&program, registers), 117440);
    }
}
