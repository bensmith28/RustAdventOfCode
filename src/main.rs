use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

mod year2023day1;
mod year2023day2;
mod year2023day3;
mod year2023day4;
mod year2023day5;

fn main() {
    println!("Hello, world!");
}

fn read_lines(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name);
    if file.is_err() {
        panic!("Error opening file {}", file_name);
    }
    let reader = BufReader::new(file.unwrap());

    reader.lines()
}