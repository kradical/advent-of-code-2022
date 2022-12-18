use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Forest {
    trees: Vec<u8>,
    columns: usize,
    rows: usize,
}

impl Forest {
    fn get(&self, row: usize, col: usize) -> u8 {
        let index = self.columns * row + col;
        self.trees[index]
    }
}

fn read_forest() -> Forest {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut trees = Vec::<u8>::new();
    let mut rows = 0;

    for _line in reader.lines() {
        rows += 1;
        let line = _line.unwrap();

        trees.extend(line.chars().map(|c| c.to_digit(10).unwrap() as u8));
    }

    let columns = trees.len() / rows;

    Forest {
        trees,
        columns,
        rows,
    }
}

fn part_one() {
    let forest = read_forest();

    let Forest { columns, rows, .. } = forest;

    let mut visible_trees = 2 * rows + 2 * columns - 4;

    for i in 1..(rows - 1) {
        for j in 1..(columns - 1) {
            let current = forest.get(i, j);

            let mut left_trees = (0..j).map(|left| forest.get(i, left));
            let mut right_trees = ((j + 1)..columns).map(|right| forest.get(i, right));
            let mut up_trees = (0..i).map(|up| forest.get(up, j));
            let mut down_trees = ((i + 1)..rows).map(|down| forest.get(down, j));

            let is_visible = left_trees.all(|x| x < current)
                || right_trees.all(|x| x < current)
                || up_trees.all(|x| x < current)
                || down_trees.all(|x| x < current);

            if is_visible {
                visible_trees += 1;
            }
        }
    }

    println!("Part 1: {}", visible_trees);
}

fn part_two() {
    println!("Part 2: ");
}

fn main() {
    part_one();
    part_two();
}
