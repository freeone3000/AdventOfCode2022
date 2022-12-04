use std::fs::File;
use std::io::{BufRead, BufReader};

type Range = (i32, i32);

fn split_parts(line: &str, pat: &str) -> (Range, Range) {
    let mut parts = line.split(pat);
    let range1 = split_range(parts.next().unwrap());
    let range2 = split_range(parts.next().unwrap());
    (range1, range2)
}

fn split_range(line: &str) -> Range {
    let mut parts = line.split("-");
    (parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap())
}

fn fully_contains((min1, max1): &Range, (min2, max2): &Range) -> i32 {
    if (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1) {
        1
    } else {
        0
    }
}
fn has_overlap((min1, max1): &Range, (min2, max2): &Range) -> i32 {
    if (min1 <= min2 && max1 >= min2) || (min2 <= min1 && max2 >= min1) {
        1
    } else {
        0
    }
}

fn calculate(lines: &Vec<String>) -> (i32, i32) {
    lines.iter()
        .map(|line| split_parts(line, ","))
        .fold((0, 0), |(day1, day2), (range1, range2)| {
            (
                day1 + fully_contains(&range1, &range2),
                day2 + has_overlap(&range1, &range2)
            )
        })
}

pub fn main(filename: &str) -> (i32, i32) {
    let file = File::open(filename).expect("Can open file");
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|line| line.unwrap()).collect();
    calculate(&lines)
}