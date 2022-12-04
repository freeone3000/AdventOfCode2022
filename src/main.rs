mod day1;
mod day2;

fn print_separator() {
    println!("========================================");
}

fn main() {
    let day1_test = day1::main("input/day1_test.txt");
    assert_eq!(day1_test.0, 24000);
    assert_eq!(day1_test.1, 45000);
    let day1 = day1::main("input/day1.txt");
    println!("Day 1:\nBest: {}\nTop 3: {}", day1.0, day1.1);

    print_separator();
    let day2_test = day2::main("input/day2_test.txt");
    assert_eq!(day2_test.0, 15);
    assert_eq!(day2_test.1, 12);
    let day2 = day2::main("input/day2.txt");
    println!("Day 2:\n Part 1: {}\n Part 2: {}", day2.0, day2.1);
    print_separator();
}
