use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for _line in reader.lines() {}

    println!("Part 1: ");
}

fn part_two() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for _line in reader.lines() {}

    println!("Part 2: ");
}

fn main() {
    part_one();
    part_two();
}
