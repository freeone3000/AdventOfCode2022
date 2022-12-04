use std::fs::File;
use std::io::{BufRead, BufReader};

enum RoundScore {
    Win = 6,
    Tie = 3,
    Loss = 0,
}
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn decode(c: Option<char>) -> Shape {
    match c {
        Some('A') => Shape::Rock,
        Some('X') => Shape::Rock,
        Some('B') => Shape::Paper,
        Some('Y') => Shape::Paper,
        Some('C') => Shape::Scissors,
        Some('Z') => Shape::Scissors,
        _ => panic!("Invalid shape"),
    }
}

fn decode2(c: Option<char>) -> RoundScore {
    match c {
        Some('X') => RoundScore::Loss,
        Some('Y') => RoundScore::Tie,
        Some('Z') => RoundScore::Win,
        _ => panic!("Invalid shape"),
    }
}

//noinspection DuplicatedCode // I don't know a nicer way to do this
fn score_line(line: &String) -> i32 {
    let opp = decode(line.chars().nth(0));
    let yours = decode(line.chars().nth(2));
    match yours {
        Shape::Rock => match opp {
            Shape::Rock => (yours as i32) + (RoundScore::Tie as i32),
            Shape::Paper => (yours as i32) + (RoundScore::Loss as i32),
            Shape::Scissors => (yours as i32) + (RoundScore::Win as i32),
        }
        Shape::Paper => match opp {
            Shape::Rock => (yours as i32) + (RoundScore::Win as i32),
            Shape::Paper => (yours as i32) + (RoundScore::Tie as i32),
            Shape::Scissors => (yours as i32) + (RoundScore::Loss as i32),
        }
        Shape::Scissors => match opp {
            Shape::Rock => (yours as i32) + (RoundScore::Loss as i32),
            Shape::Paper => (yours as i32) + (RoundScore::Win as i32),
            Shape::Scissors => (yours as i32) + (RoundScore::Tie as i32),
        }
    }
}

fn win_from(shape: &Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    }
}
fn lose_from(shape: &Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

fn score_line2(line: &String) -> i32 {
    let opp = decode(line.chars().nth(0));
    let result = decode2(line.chars().nth(2));
    let your_shape = match result {
        RoundScore::Win => win_from(&opp),
        RoundScore::Loss => lose_from(&opp),
        RoundScore::Tie => opp,
    };
    (your_shape as i32) + (result as i32)
}

fn parse_file(filename: &str) -> (i32, i32) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let day1 = lines.iter().map(|line| score_line(line)).sum();
    let day2 = lines.iter().map(|line| score_line2(line)).sum();
    (day1, day2)
}

pub fn main(filename: &str) -> (i32, i32) {
    parse_file(filename)
}