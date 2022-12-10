mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod rotate_buffer;
mod day7;
mod day8;
mod day9;

fn print_separator() {
    println!("========================================");
}

fn main() {
    // TODO do actual tests!
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
    let day3_test = day3::main("input/day3_test.txt");
    assert_eq!(day3_test.0, 157);
    assert_eq!(day3_test.1, 70);
    let day3 = day3::main("input/day3.txt");
    println!("Day 3:\n Part 1: {}\n Part 2: {}", day3.0, day3.1);

    print_separator();
    let day4_test = day4::main("input/day4_test.txt");
    assert_eq!(day4_test.0, 2);
    assert_eq!(day4_test.1, 4);
    let day4 = day4::main("input/day4.txt");
    println!("Day 4:\n Part 1: {}\n Part 2: {}", day4.0, day4.1);

    print_separator();
    let day5_test = day5::main("input/day5_test.txt");
    assert_eq!(day5_test.0, "CMZ");
    assert_eq!(day5_test.1, "MCD");
    let day5 = day5::main("input/day5.txt");
    println!("Day 5:\n Part 1: {}\n Part 2: {}", day5.0, day5.1);

    print_separator();
    let day6_test = day6::main("input/day6_test.txt");
    assert_eq!(day6_test, vec!((7, 19), (5, 23), (6, 23), (10, 29), (11, 26)));
    let day6 = day6::main("input/day6.txt");
    println!("Day 6:\n Part 1: {}\n Part 2: {}", day6[0].0, day6[0].1);

    print_separator();
    let day7_test = day7::main("input/day7_test.txt");
    assert_eq!(day7_test.0, 95437);
    assert_eq!(day7_test.1, 24933642);
    let day7 = day7::main("input/day7.txt");
    println!("Day 7:\n Part 1: {}\n Part 2: {}", day7.0, day7.1);

    print_separator();
    let day8_test = day8::main("input/day8_test.txt");
    assert_eq!(day8_test.0, 21);
    assert_eq!(day8_test.1, 8);
    let day8 = day8::main("input/day8.txt");
    println!("Day 8:\n Part 1: {}\n Part 2: {}", day8.0, day8.1);

    print_separator();
    let day9_test = day9::main("input/day9_test.txt");
    assert_eq!(day9_test.0, 13);
    let day9 = day9::main("input/day9.txt");
    println!("Day 9:\n Part 1: {}\n Part 2: {}", day9.0, day9.1);
}
