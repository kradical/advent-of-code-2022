use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn get_directory_sizes() -> HashMap<String, i32> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut directories = HashSet::<String>::new();
    let mut file_sizes = HashMap::<String, i32>::new();

    let mut current_dir = String::new();
    for line in reader.lines() {
        let line_str = line.unwrap();
        let mut spliterator = line_str.split_whitespace();

        if line_str.starts_with("$") {
            let cmd = spliterator.nth(1).unwrap();

            if cmd == "cd" {
                let dir = spliterator.nth(0).unwrap();

                current_dir = match dir {
                    "/" => "/".to_owned(),
                    ".." => {
                        let rsplit_str = current_dir.rsplit_once("/").unwrap();

                        if rsplit_str.0.is_empty() {
                            "/".to_owned()
                        } else {
                            rsplit_str.0.to_owned()
                        }
                    }
                    x => {
                        if current_dir == "/" {
                            format!("/{}", x)
                        } else {
                            format!("{}/{}", current_dir, x)
                        }
                    }
                };

                directories.insert(current_dir.clone());
            }
        } else {
            let size_str = spliterator.nth(0).unwrap();
            let name = spliterator.nth(0).unwrap();

            if size_str == "dir" {
                continue;
            }

            let size = size_str.parse::<i32>().unwrap();
            let full_name = format!("{}/{}", current_dir, name);

            file_sizes.insert(full_name, size);
        }
    }

    let mut directory_sizes = HashMap::<String, i32>::new();
    for directory in directories {
        let mut total_size = 0;

        for (name, size) in &file_sizes {
            if name.starts_with(&directory) {
                total_size += size;
            }
        }

        directory_sizes.insert(directory, total_size);
    }

    return directory_sizes;
}

fn part_one() {
    let directory_sizes = get_directory_sizes();

    let cumulative_size = directory_sizes
        .values()
        .filter(|size| **size <= 100_000)
        .sum::<i32>();

    println!("Part 1: {}", cumulative_size);
}

fn main() {
    part_one();
}
