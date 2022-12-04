use std::fs::File;
use std::io::{BufRead, BufReader};

fn convert_to_priority(c: &char) -> i32 {
    if c >= &'A' && c <= &'Z' {
        return (*c as i32) - ('A' as i32) + 27;
    } else if c >= &'a' && c <= &'z' {
        return (*c as i32) - ('a' as i32) + 1;
    } else {
        panic!("Invalid character: {}", *c);
    }
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines.iter() {
        let chars: Vec<char> = line.chars().collect();
        let second_half = &chars[chars.len() / 2..];
        for i in 0..chars.len() / 2 {
            if second_half.contains(&chars[i]) {
                sum += convert_to_priority(&chars[i]);
                break;
            }
        }
    }
    sum
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    lines.chunks(3).for_each(|chunk| {
        let elves: Vec<Vec<char>> = chunk.iter().map(|line| line.chars().collect()).collect();
        for i in 0..elves[0].len() {
            if elves[1].contains(&elves[0][i]) && elves[2].contains(&elves[0][i]) {
                sum += convert_to_priority(&elves[0][i]);
                break;
            }
        }
    });
    return sum;
}

pub fn main(filename: &str) -> (i32, i32) {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|line| line.unwrap()).collect();
    let day1 = part1(&lines);
    let day2 = part2(&lines);
    (day1, day2)
}