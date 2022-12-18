use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn detect_start(count: usize, input: impl IntoIterator<Item=char>) -> i32 {
    let mut result = 0;
    let mut buffer: VecDeque<char> = VecDeque::new();

    for c in input {
        result += 1;
        buffer.push_front(c);

        if buffer.len() > count {
            buffer.pop_back();
        }

        if buffer.len() == count && has_unique_elements(&buffer) {
            break;
        }
    }

    return result;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let line = reader.lines().take(1).next().unwrap().unwrap();

    let p1_result = detect_start(4, line.chars());
    println!("Part 1: {}", p1_result);

    let p2_result = detect_start(14, line.chars());
    println!("Part 2: {}", p2_result);
}
