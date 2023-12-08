use std::{env::args, fs, time::Instant};

pub type Result = (Option<u64>, Option<u64>);

pub fn run(day: u8, runner: fn(&str) -> Result) {
    println!("Advent of Code, Day {}!", day);

    let file_path = args().nth(1)
        .unwrap_or(format!("./day{}/src/input.txt", day));

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let timer = Instant::now();

    let (part_1, part_2) = runner(input.as_str());

    let time_taken = timer.elapsed();

    let part_1 = match part_1 {
        Some(v) => format!("{}", v),
        None => "No value!".to_string(),
    };

    println!("Day {day} Result, Part 1: {}", part_1);

    let part_2 = match part_2 {
        Some(v) => format!("{}", v),
        None => "No value!".to_string(),
    };

    println!("Day {day} Result, Part 2: {}", part_2);

    println!("Time Taken: {:?}", time_taken);
}