use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut highest_totals = [0, 0, 0];
    let mut running_total = 0;

    for line in reader.lines() {
      let line_str = line.unwrap();
      if line_str.is_empty() {
        running_total = 0;
      } else {
        running_total += line_str.parse::<i32>().unwrap();
      }

      if running_total > highest_totals[2] {
        highest_totals[2] = running_total;
      }

      if running_total > highest_totals[1] {
        highest_totals.swap(1, 2);
      }

      if running_total > highest_totals[0] {
        highest_totals.swap(0, 1);
      }
    }

    let total: i32 = highest_totals.iter().sum();

    println!("part 1: {}", highest_totals[0]);
    println!("part 2: {}", total);
}
