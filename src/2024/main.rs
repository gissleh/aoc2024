#![feature(binary_heap_into_iter_sorted)]
#![feature(array_windows)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

use common::runner::run;

fn main() {
    run(2024, 1, day01::main);
    run(2024, 2, day02::main);
    run(2024, 3, day03::main);
    run(2024, 4, day04::main);
    run(2024, 5, day05::main);
    run(2024, 6, day06::main);
    run(2024, 7, day07::main);
    run(2024, 8, day08::main);
    run(2024, 9, day09::main);
    run(2024, 10, day10::main);
}
