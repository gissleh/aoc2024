#![feature(binary_heap_into_iter_sorted)]
#![feature(array_windows)]

mod day01;
mod day02;
mod day03;

use common::runner::run;

fn main() {
    run(2024, 1, day01::main);
    run(2024, 2, day02::main);
    run(2024, 3, day03::main);
}
