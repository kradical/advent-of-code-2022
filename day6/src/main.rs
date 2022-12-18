use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;
    let mut buffer: VecDeque<char> = VecDeque::new();

    for line in reader.lines() {
        let line_str = line.unwrap();
        for c in line_str.chars() {
            result += 1;
            buffer.push_front(c);

            if buffer.len() > 4 {
                buffer.pop_back();
            }

            if buffer.len() == 4 && has_unique_elements(&buffer) {
                break;
            }
        }
    }

    println!("Part 1: {}", result);
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn part_two() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let _line_str = line.unwrap();
    }

    println!("Part 2:");
}

fn main() {
    part_one();
    part_two();
}
