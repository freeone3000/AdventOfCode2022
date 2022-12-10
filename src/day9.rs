use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Location {
    x: i32,
    y: i32,
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
#[derive(Debug, Clone, Copy)] // it's *one value*
enum Direction {
    Up = 0,
    Down = 1,
    Right = 2,
    Left = 3,
}

struct Insn {
    direction: Direction,
    count: usize,
}

fn are_touching(head: &Location, tail: &Location) -> bool {
    head.x.abs_diff(tail.x) <= 1 && head.y.abs_diff(tail.y) <= 1
}

fn catchup(head: &Location, tail: &mut Location) {
    if are_touching(head, tail) {
        return;
    }
    if head.x == tail.x { // same column
        if head.y >= tail.y + 2 {
            tail.y += 1;
        } else if head.y <= tail.y - 2 {
            tail.y -= 1;
        }
    } else if head.y == tail.y { // same row
        if head.x >= tail.x + 2 {
            tail.x += 1;
        } else if head.x <= tail.x - 2 {
            tail.x -= 1;
        }
    } else { // neither same row nor same column!
        // this is likely going to be a source of issues
        if head.y > tail.y {
            tail.y += 1;
        } else { // head.y < tail.y
            tail.y -= 1;
        }
        if head.x > tail.x {
            tail.x += 1;
        } else { // head.x < tail.x
            tail.x -= 1;
        }
    }
    assert!(are_touching(head, tail)); // verify we've done this correctly
}

fn do_motion(head: &mut Location, tail: &mut Location, direction: Direction) -> Location {
    let (dx, dy) = DIRECTIONS[direction as usize];
    head.x += dx;
    head.y += dy;
    catchup(head, tail);
    tail.clone()
}

fn parse_line(line: String) -> Insn {
    let mut tokens = line.split_whitespace();
    let dir_candidate = tokens.next().expect("empty line");
    let direction = match dir_candidate {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "R" => Direction::Right,
        "L" => Direction::Left,
        _ => panic!("Unknown direction: {}", dir_candidate),
    };
    let count = tokens.next().expect("no count").parse::<usize>().expect("not a number");
    Insn { direction, count }
}

pub fn main(filename: &str) -> (i32, i32) {
    let file = File::open(filename).expect("can open file");
    let reader = BufReader::new(file);

    let mut head = Location { x: 0, y: 0};
    let mut tail = Location { x: 0, y: 0 };
    let mut seen = HashSet::new();
    seen.insert(tail.clone());
    for insn in reader.lines().map(|line| parse_line(line.expect("has line"))) {
        let (count, dir) = (insn.count, insn.direction);
        for _i in 0..count {
            do_motion(&mut head, &mut tail, dir);
            seen.insert(tail.clone());
        }
    }
    let day1 = seen.len();

    (day1 as i32, 0)
}
