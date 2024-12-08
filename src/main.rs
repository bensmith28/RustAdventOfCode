use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};

mod year2023day1;
mod year2023day2;
mod year2023day3;
mod year2023day4;
mod year2023day5;
mod year2023day6;
mod year2023day7;
mod year2023day8;
mod year2023day9;
mod year2023day10;
mod year2024day1;
mod year2024day2;
mod year2023day11;
mod year2024day3;
mod year2024day4;
mod year2024day5;
mod year2024day6;
mod year2024day7;
mod year2024day8;

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

fn read_string(file_name: &str) -> String {
    let file = File::open(file_name);
    if file.is_err() {
        panic!("Error opening file {}", file_name);
    }
    let mut s = String::new();
    if file.unwrap().read_to_string(&mut s).is_err() {
        panic!("Error reading file {}", file_name);
    }
    s
}