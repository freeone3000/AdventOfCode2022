use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::rotate_buffer::RotateBuffer;

type CResult = (usize, usize);

fn detect_packet_start(line: &String, count: usize) -> Option<usize> {
    let mut buf = RotateBuffer::new(count);
    for (idx, c) in line.char_indices() {
        buf.push(c);
        if buf.distinct() >= count {
            return Some(idx+1);
        }
    }
    None
}

fn run_line<E: Error>(line: Result<String, E>) -> CResult {
    let xu = line.unwrap();
    (detect_packet_start(&xu, 4).unwrap(), detect_packet_start(&xu, 14).unwrap())
}

pub fn main(filename: &str) -> Vec<CResult> {
    let f = File::open(filename).expect("Can open given file");
    let reader = BufReader::new(f);

    reader.lines()
        .map(run_line)
        .collect()
}
