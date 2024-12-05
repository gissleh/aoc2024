#![feature(binary_heap_into_iter_sorted)]
#![feature(array_windows)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

use common::runner::run;

fn main() {
    run(2024, 1, day01::main);
    run(2024, 2, day02::main);
    run(2024, 3, day03::main);
    run(2024, 4, day04::main);
    run(2024, 5, day05::main);
}
