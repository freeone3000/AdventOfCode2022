use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_biggest(filename: &str) -> [i32; 3] {
    let f = File::open(filename).expect("Can open given file");
    let reader = BufReader::new(f);

    let mut values = [0; 3];
    let mut cur_value = 0;
    for line_w in reader.lines() {
        let line = line_w.unwrap();
        if line == "" {
            assign_and_rotate(&mut values,  cur_value);
            cur_value = 0;
        } else {
            cur_value += line.parse::<i32>().expect("Can parse the line");
        }
    }
    assign_and_rotate(&mut values, cur_value);
    values
}

fn assign_and_rotate(values: &mut [i32; 3], cur_value: i32) {
    for i in 0..values.len() {
        if values[i] < cur_value {
            for j in (i + 1..values.len()).rev() {
                values[j] = values[j - 1];
            }
            values[i] = cur_value;
            break;
        }
    }
}

pub fn main(filename: &str) -> (i32, i32) {
    let res = parse_biggest(filename);
    (res[0], res[0]+ res[1] + res[2])
}