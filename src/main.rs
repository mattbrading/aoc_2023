use std::fs;

mod day1;

fn read_file (file_path: &str) -> String {
    return fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
}

fn day_1_result() {
    let input = read_file("./src/day1.txt");

    let result = day1::document_calbration_sum(input.as_str());

    println!("Day 1 Result: {}", result);
}

fn main() {
    println!("Merry Xmas!");

    day_1_result();
}