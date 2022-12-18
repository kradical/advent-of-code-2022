use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part_one() {
    // [Q]         [N]             [N]
    // [H]     [B] [D]             [S] [M]
    // [C]     [Q] [J]         [V] [Q] [D]
    // [T]     [S] [Z] [F]     [J] [J] [W]
    // [N] [G] [T] [S] [V]     [B] [C] [C]
    // [S] [B] [R] [W] [D] [J] [Q] [R] [Q]
    // [V] [D] [W] [G] [P] [W] [N] [T] [S]
    // [B] [W] [F] [L] [M] [F] [L] [G] [J]
    // 1   2   3   4   5   6   7   8   9
    let mut columns = vec![
        vec!['B', 'V', 'S', 'N', 'T', 'C', 'H', 'Q'],
        vec!['W', 'D', 'B', 'G'],
        vec!['F', 'W', 'R', 'T', 'S', 'Q', 'B'],
        vec!['L', 'G', 'W', 'S', 'Z', 'J', 'D', 'N'],
        vec!['M', 'P', 'D', 'V', 'F'],
        vec!['F', 'W', 'J'],
        vec!['L', 'N', 'Q', 'B', 'J', 'V'],
        vec!['G', 'T', 'R', 'C', 'J', 'Q', 'S', 'N'],
        vec!['J', 'S', 'Q', 'C', 'W', 'D', 'M'],
    ];

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_str = line.unwrap();
        if !line_str.starts_with("move ") {
            continue;
        }

        let mut spliterator = line_str.split_whitespace();

        spliterator.next();
        let depth_str = spliterator.next().unwrap();
        let depth = depth_str.parse::<usize>().unwrap();
        spliterator.next();
        let from_str = spliterator.next().unwrap();
        let from = from_str.parse::<usize>().unwrap() - 1;
        spliterator.next();
        let to_str = spliterator.next().unwrap();
        let to = to_str.parse::<usize>().unwrap() - 1;

        for _ in 0..depth {
            let value = columns[from].pop().unwrap();
            columns[to].push(value);
        }
    }

    let result = columns
        .iter()
        .map(|c| c.last())
        .flatten()
        .collect::<String>();

    println!("Part 1: {}", result);
}

fn part_two() {
    println!("Part 2");
}

fn main() {
    part_one();
    part_two();
}
