use std::{collections::HashMap, time::Instant, fs, env::args};
enum Instruction {
    L,
    R,
}

fn find_step_count(input: &str) -> u32 {
    let (instructions, map) = input.split_once("\n\n").unwrap();

    let mut instructions = instructions
        .trim()
        .chars()
        .map(|c| match c {
            'L' => Some(Instruction::L),
            'R' => Some(Instruction::R),
            _=> None
        }.expect("Invalid Instruction"))
        .cycle();

    let map = map.lines().fold(HashMap::new(), |mut acc, l| {
        let string = l
            .to_owned()
            .replace(&['_', '(', ')', ',', '='], "");
        
        let mut pieces = string
            .split_whitespace()
            .map(|l| l.to_string());

        let key = pieces.next().unwrap();
        let val = (pieces.next().unwrap(), pieces.next().unwrap());

        acc.insert(key, val);
        return acc;
    });

    let mut location = String::from("AAA");
    let mut counter = 0;

    while location != "ZZZ" {
        counter += 1;
        let next = &map[&location];
        location = match instructions.next().unwrap() {
            Instruction::L => next.0.clone(),
            Instruction::R => next.1.clone(),
        };
    }

    return counter;
}

fn main() {
    println!("Advent of Code, Day 8!");

    let file_path = args().nth(1).expect("Missing File Path!");

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let timer = Instant::now();

    let part_1 = find_step_count(input.as_str());

    let time_taken = timer.elapsed();

    println!("Day 8 Result, Part 1: {}", part_1);
    // println!("Day 8 Result, Part 2: {}", part_2);
    println!("Time Taken: {:?}", time_taken);
}

#[cfg(test)]
mod tests {
    use crate::find_step_count;

    #[test]
    fn test_find_step_count() {
        let input = "\
            LLR\n\n\
            AAA = (BBB, BBB)\n\
            BBB = (AAA, ZZZ)\n\
            ZZZ = (ZZZ, ZZZ)\
        ";

        let result = find_step_count(input);

        assert_eq!(result, 6);
    }
}
