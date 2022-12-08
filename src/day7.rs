use std::cell::RefCell;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

#[derive(Debug)]
struct VFile {
    is_dir: bool,
    name: String,
    _size: i32,
    children: Vec<Rc<RefCell<VFile>>>,
    parent: Option<Rc<RefCell<VFile>>>,
}

impl Display for VFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.to_fmt_str(0)?)
    }
}

impl VFile {
    fn make_root() -> Self {
        VFile {
            is_dir: true,
            name: "/".to_string(),
            _size: 0,
            children: vec![],
            parent: None.into(),
        }
    }

    fn size(&self) -> i32 {
        if self.is_dir {
            if self._size == 0 {
                self.children.iter().map(|c| c.borrow().size()).sum()
            } else {
                self._size // it's been calculated by collect
            }
        } else {
            self._size
        }
    }

    fn to_fmt_str(&self, indent_count: usize) -> Result<String, std::fmt::Error> {
        let filler = if self.is_dir { "dir".to_string() } else { format!("file, size={}", self._size) };
        let mut builder = String::new();
        let indent = "  ".repeat(indent_count);
        builder.push_str(&*format!("{}- {} ({})\n", indent, self.name, filler));
        if self.is_dir {
            for child in &self.children {
                builder.push_str(&*format!("{}{}", indent, child.borrow().to_fmt_str(indent_count+1)?));
            }
        }
        Ok(builder)
    }
}

fn collect(root: &Rc<RefCell<VFile>>) -> Vec<Rc<RefCell<VFile>>> {
    let mut result: Vec<Rc<RefCell<VFile>>> = vec![Rc::clone(root)];
    let mut r = root.borrow_mut();
    if r.is_dir {
        for child in &r.children {
            result.extend(collect(child));
        }
        if r._size == 0 {
            r._size = r.size(); //memoize
        }
    }
    result
}

fn parse_line(line: &String, parent: &Rc<RefCell<VFile>>) -> Rc<RefCell<VFile>> {
    let mut parts = line.split_whitespace();
    let size_str = parts.next().unwrap();
    let name = parts.next().unwrap();
    let is_dir = size_str == "dir";

    Rc::new(RefCell::new(VFile {
        is_dir,
        name: name.to_string(),
        _size: if is_dir { 0 } else { size_str.parse().unwrap() },
        children: vec![],
        parent: Some(Rc::clone(parent)),
    }))
}

fn parse_tree_from_root<T>(lines_iter: &mut T) -> Rc<RefCell<VFile>>
    where T : Iterator<Item=String>
{
    // input is a list of bash commands traversing through a filesystem
    // we want to parse this into a tree of VFiles
    let root = Rc::new(RefCell::new(VFile::make_root()));

    let mut current_dir = Rc::clone(&root);

    let mut lines = lines_iter.peekable();
    while let Some(line) = lines.next() {
        if line.trim() == "" {
            continue;
        }

        let mut parts = line.split_whitespace().skip(1); // skip $
        let command = parts.next().unwrap();
        match command {
            "cd" => {
                let c_dir = Rc::clone(&current_dir);
                let arg = parts.next().unwrap();
                if arg == "/" {
                    current_dir = Rc::clone(&root);
                } else if arg == ".." {
                    current_dir = Rc::clone(c_dir.borrow().parent.as_ref().expect("no parent"));
                } else {
                    for child in c_dir.borrow().children.iter() {
                        if child.borrow().name == arg {
                            current_dir = Rc::clone(child);
                            break;
                        }
                    }
                }
            }
            "ls" => {
                let mut children = vec![];
                loop {
                    match lines.peek() {
                        Some(s) => {
                            if s.starts_with("$") {
                                break;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                    let line = lines.next().unwrap();
                    let new_file = parse_line(&line, &current_dir);
                    children.push(new_file);
                }
                current_dir.borrow_mut().children = children;
            }
            _ => {
                panic!("unknown command: {}", command);
            }
        }
    }
    root
}

fn collect_smaller_than(limit: i32, root: &Rc<RefCell<VFile>>) -> i32 {
    collect(&root).iter()
        .filter(|f| f.borrow().is_dir && f.borrow().size() < limit)
        .map(|f| f.borrow().size())
        .sum()
}

fn calculate_part2(total_size: i32, min_space: i32, root: &Rc<RefCell<VFile>>) -> i32 {
    let free_space = total_size - root.borrow().size();
    let to_delete = min_space - free_space;
    collect(&root).iter()
        .filter(|f| f.borrow().is_dir && f.borrow().size() > to_delete)
        .map(|f| f.borrow().size())
        .min()
        .expect("there is an answer")
}

pub fn main(filename: &str) -> (i32, i32) {
    let f = File::open(filename).expect("Can open given file");
    let reader = BufReader::new(f);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let tree = parse_tree_from_root(&mut lines);
    let part1 = collect_smaller_than(100_000, &tree);
    let part2 = calculate_part2(70000000, 30000000, &tree);

    (part1, part2)
}
