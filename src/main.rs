mod day1;

fn print_separator() {
    println!("========================================");
}

fn main() {
    let day1_test = day1::main("input/day1_test.txt");
    assert_eq!(day1_test.0, 24000);
    assert_eq!(day1_test.1, 45000);
    let day1 = day1::main("input/day1.txt");
    println!("Day 1:\nBest: {}\nTop 3: {}\n", day1.0, day1.1);
    print_separator();
}
