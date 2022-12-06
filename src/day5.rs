use std::collections::VecDeque;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Container = VecDeque<char>;

#[derive(Clone, Debug)]
struct Containers {
    crates: Vec<Container>,
}

impl fmt::Display for Containers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.crates.len() == 0 {
            return write!(f, "");
        }

        let max_size = self.crates.iter().map(|c| c.len()).max().expect("We have at least one stack");
        // vertically write containers from top to bottom...
        for j in (0..max_size).rev() {
            // in order
            for i in 0..self.crates.len() {
                if j < self.crates[i].len() {
                    write!(f, "[{}]", self.crates[i][j])?;
                } else {
                    write!(f, "   ")?;
                }
            }
            writeln!(f, "")?;
        }
        for j in 0..max_size {
            write!(f, " {} ", j+1)?;
        }
        writeln!(f, "")?;
        Ok(())
    }
}

fn parse_crates<R: std::io::Read>(reader: &mut BufReader<R>) -> Containers {
    let mut containers = Containers { crates: Vec::new() };
    let mut line = String::new();
    loop {
        reader.read_line(&mut line).expect("Can read line");
        if line.trim_end() == "" {
            break;
        }

        let chars = line.chars().collect::<Vec<_>>();
        line.clear();
        // fill our empty vecs first
        for _i in 0..(chars.len() / 4)+1 {
            containers.crates.push(Container::new());
        }
        // then fill with data
        chars.chunks(4).enumerate().for_each(|(i, c)| {
            if c[0] == '[' && c[2] == ']' {
                containers.crates[i].push_front(c[1]);
            }
        });
    }
    containers
}

#[derive(Debug)]
struct Insn {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_instructions<R: std::io::Read>(reader: &mut BufReader<R>) -> Vec<Insn> {
    reader.lines().map(|line| {
        let line = line.expect("Can read line");
        let mut parts = line.split_whitespace();
        parts.next(); // discard "move"
        let count = parts.next().expect("Can read count").parse::<usize>().expect("Can parse count");
        parts.next(); //discard "from"
        let from = parts.next().expect("Can read from").parse::<usize>().expect("Can parse from");
        parts.next(); //discard "to"
        let to = parts.next().expect("Can read to").parse::<usize>().expect("Can parse to");
        Insn { count, from, to }
    }).collect()
}

fn run_program(containers: &mut Containers, instructions: &Vec<Insn>) {
    for insn in instructions {
        let from = insn.from - 1;
        let to = insn.to - 1;
        for _i in 0..insn.count {
            let c = containers.crates[from].pop_back().expect("Can pop from source");
            containers.crates[to].push_back(c);
        }
    }
}

fn run_program2(containers: &mut Containers, instructions: &Vec<Insn>) {
    for insn in instructions {
        let from = insn.from - 1;
        let to = insn.to - 1;
        let mut tmp = Container::new();
        for _i in 0..insn.count {
            let c = containers.crates[from].pop_back().expect("Can pop from source");
            tmp.push_back(c); //pushing to the temp queue automatically flips it
        }
        while let Some(c) = tmp.pop_back() {
            containers.crates[to].push_back(c);
        }
    }
}

fn read_containers(containers: &Containers) -> String {
    containers.crates.iter()
        .map(|container| container.back())
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .collect::<String>()
}

pub fn main(filename: &str) -> (String, String) {
    let f = File::open(filename).expect("Can open given file");
    let mut reader = BufReader::new(f);

    let mut containers = parse_crates(&mut reader);
    let instructions = parse_instructions(&mut reader);
    let mut containers2 = containers.clone();
    //day1
    run_program(&mut containers, &instructions);
    let day1 = read_containers(&containers);
    //day2
    run_program2(&mut containers2, &instructions);
    let day2 = read_containers(&containers2);
    (day1, day2)
}