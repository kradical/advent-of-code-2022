use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total: i32 = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();

        let mut spliterator = line_str.split(",");
        let first_str = spliterator.next().unwrap();
        let second_str = spliterator.next().unwrap();

        let first_range = parse_elf(first_str);
        let second_range = parse_elf(second_str);

        if do_ranges_overlap(first_range, second_range) {
            total += 1;
        }
    }

    println!("part 2: {}", total);
}

fn do_ranges_overlap(r1: [i32; 2], r2: [i32; 2]) -> bool {
    r1[0] <= r2[1] && r2[0] <= r1[1]
}

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total: i32 = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();

        let mut spliterator = line_str.split(",");
        let first_str = spliterator.next().unwrap();
        let second_str = spliterator.next().unwrap();

        let first_range = parse_elf(first_str);
        let second_range = parse_elf(second_str);

        if is_either_range_contained(first_range, second_range) {
            total += 1;
        }
    }

    println!("part 1: {}", total);
}

fn parse_elf(elf: &str) -> [i32; 2] {
    let mut spliterator = elf.split("-");
    let first_str = spliterator.next().unwrap();
    let second_str = spliterator.next().unwrap();

    let first_int: i32 = first_str.parse().unwrap();
    let second_int: i32 = second_str.parse().unwrap();

    [first_int, second_int]
}

fn is_either_range_contained(r1: [i32; 2], r2: [i32; 2]) -> bool {
    r1[0] <= r2[0] && r1[1] >= r2[1] || r2[0] <= r1[0] && r2[1] >= r1[1]
}
