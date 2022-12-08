use std::fs::File;
use std::io::{BufRead, BufReader};

type Grid = Vec<Vec<char>>;

fn is_visible(grid: &Grid, tgt_i: usize, tgt_j: usize) -> bool {
    if grid.len() == 0 || grid[tgt_i].len() == 0 {
        return true; // empty grids match
    }
    if tgt_i == 0 || tgt_j == 0 || tgt_i == grid.len() - 1 || tgt_j == grid[tgt_i].len() - 1 {
        return true;
    }
    let tgt = grid[tgt_i][tgt_j];

    let mut found = true;
    for i in 0..tgt_i {
        if grid[i][tgt_j] >= tgt {
            found = false;
            break;
        }
    }
    if found {
        return true;
    }

    found = true;
    for i in tgt_i+1..grid.len() {
        if grid[i][tgt_j] >= tgt {
            found = false;
            break;
        }
    }
    if found {
        return true;
    }

    found = true;
    for j in 0..tgt_j {
        if grid[tgt_i][j] >= tgt {
            found = false;
            break;
        }
    }
    if found {
        return true;
    }

    found = true;
    for j in tgt_j+1..grid[tgt_i].len() {
        if grid[tgt_i][j] >= tgt {
            found = false;
            break;
        }
    }

    found
}

fn get_scenic_score(grid: &Grid, tgt_i: usize, tgt_j: usize) -> i32 {
    let tgt = grid[tgt_i][tgt_j];

    let mut scenic_score_up = 0;
    for i in (0..tgt_i).rev() {
        scenic_score_up += 1;
        if grid[i][tgt_j] >= tgt {
            break;
        }
    }

    let mut scenic_score_down = 0;
    for i in tgt_i+1..grid.len() {
        scenic_score_down += 1;
        if grid[i][tgt_j] >= tgt {
            break;
        }
    }

    let mut scenic_score_left = 0;
    for j in (0..tgt_j).rev() {
        scenic_score_left += 1;
        if grid[tgt_i][j] >= tgt {
            break;
        }
    }

    let mut scenic_score_right = 0;
    for j in tgt_j+1..grid[tgt_i].len() {
        scenic_score_right += 1;
        if grid[tgt_i][j] >= tgt {
            break;
        }
    }

    scenic_score_up * scenic_score_right * scenic_score_down * scenic_score_left
}

fn calculate_day1(grid: &Grid) -> i32 {
    let mut day1 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if is_visible(&grid, i, j) {
                day1 += 1;
            }
        }
    }
    day1
}

fn calculate_day2(grid: &Grid) -> i32 {
    let mut day2 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let ss = get_scenic_score(&grid, i, j);
            if ss > day2 {
                day2 = ss;
            }
        }
    }
    day2
}

pub fn main(filename: &str) -> (i32, i32) {
    let f = File::open(filename).expect("Can open given file");
    let reader = BufReader::new(f);

    let grid: Grid =
        reader.lines()
            .map(|l| l.expect("Can read line").chars().collect::<Vec<char>>())
            .collect();

    let day1 = calculate_day1(&grid);
    let day2 = calculate_day2(&grid);
    (day1, day2)
}
