use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_priority(c: char) -> u8 {
    match c {
        'A'..='Z' => c as u8 - 38,
        'a'..='z' => c as u8 - 96,
        _ => panic!("NOT ASCII"),
    }
}

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total: i32 = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();

        let (first, second) = line_str.split_at(line_str.len() / 2);

        let first_set = HashSet::<_>::from_iter(first.chars());
        let second_set = HashSet::<_>::from_iter(second.chars());

        let mut intersection = first_set.intersection(&second_set);

        // We know there will be 1 overlap (as per problem definition)
        let common = intersection.next().unwrap();
        total += get_priority(*common) as i32;
    }

    println!("part 1: {}", total);
}

fn part_two() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total: i32 = 0;

    let mut counter = 0;
    let mut lines = ["".to_string(), "".to_string(), "".to_string()];
    for line in reader.lines() {
        let line_str = line.unwrap();
        lines[counter] = line_str;

        if counter == 2 {
            let first = HashSet::<_>::from_iter(lines[0].chars());
            let second = HashSet::<_>::from_iter(lines[1].chars());
            let third = HashSet::<_>::from_iter(lines[2].chars());

            // We know there will be 1 overlap (as per problem definition)
            let common = first
                .iter()
                .filter(|c| second.contains(c) && third.contains(c))
                .next()
                .unwrap();

            total += get_priority(*common) as i32;
        }

        counter += 1;
        counter %= 3;
    }

    println!("part 2: {}", total);
}

fn main() {
    part_one();
    part_two();
}
