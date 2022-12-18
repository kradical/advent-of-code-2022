use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(input: &str) -> Result<Choice, Self::Err> {
        match input {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

enum GameResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for GameResult {
  type Err = ();

  fn from_str(input: &str) -> Result<GameResult, Self::Err> {
      match input {
          "X" => Ok(GameResult::Loss),
          "Y" => Ok(GameResult::Draw),
          "Z" => Ok(GameResult::Win),
          _ => Err(()),
      }
  }
}

fn get_game_result(opponent: &Choice, me: &Choice) -> GameResult {
    match opponent {
        Choice::Rock => match me {
            Choice::Rock => GameResult::Draw,
            Choice::Paper => GameResult::Win,
            Choice::Scissors => GameResult::Loss,
        },
        Choice::Paper => match me {
            Choice::Rock => GameResult::Loss,
            Choice::Paper => GameResult::Draw,
            Choice::Scissors => GameResult::Win,
        },
        Choice::Scissors => match me {
            Choice::Rock => GameResult::Win,
            Choice::Paper => GameResult::Loss,
            Choice::Scissors => GameResult::Draw,
        },
    }
}

fn get_move(opponent: &Choice, result: &GameResult) -> Choice {
  match opponent {
    Choice::Rock => match result {
        GameResult::Draw => Choice::Rock,
        GameResult::Win => Choice::Paper,
        GameResult::Loss => Choice::Scissors,
    },
    Choice::Paper => match result {
        GameResult::Loss => Choice::Rock,
        GameResult::Draw => Choice::Paper,
        GameResult::Win => Choice::Scissors,
    },
    Choice::Scissors => match result {
        GameResult::Win => Choice::Rock,
        GameResult::Loss => Choice::Paper,
        GameResult::Draw => Choice::Scissors,
    },
}
}

fn get_round_points(opponent: Choice, me: Choice) -> i32 {
    let game_result = get_game_result(&opponent, &me);

    me as i32 + game_result as i32
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();
        let mut split_line = line_str.split_whitespace().take(2);

        let opponent_str = split_line.next().unwrap();
        let me_str = split_line.next().unwrap();

        let opponent = Choice::from_str(opponent_str).unwrap();
        let game_result = GameResult::from_str(me_str).unwrap();

        let me = get_move(&opponent, &game_result);
        total += get_round_points(opponent, me);
    }

    println!("{}", total);
}
