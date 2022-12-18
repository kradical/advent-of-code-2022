use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

fn main() {
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

        let priority = match *common {
            'A'..='Z' => *common as u8 - 38,
            'a'..='z' => *common as u8 - 96,
            _ => panic!("NOT ASCII"),
        };

        total += priority as i32;
    }

    println!("{}", total);
}
