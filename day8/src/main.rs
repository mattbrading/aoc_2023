use std::{collections::HashMap, env::args, fs, time::Instant};
#[derive(Debug)]
enum Instruction {
    L,
    R,
}

impl From<&char> for Instruction {
    fn from(value: &char) -> Self {
        match value {
            'L' => Some(Instruction::L),
            'R' => Some(Instruction::R),
            _ => None,
        }
        .expect("Invalid Instruction")
    }
}

fn traverse(
    start: String,
    map: &HashMap<String, (String, String)>,
    instructions: &Vec<Instruction>,
    end_condition: fn(&String) -> bool,
) -> Option<u64> {
    let mut instructions = instructions.iter().cycle();
    let mut location = start;
    let mut count = Some(0);
    while end_condition(&location) {
        count = count.and_then(|v| Some(v + 1));
        let next = &map.get(&location);

        location = match (next, instructions.next()) {
            (Some(n), Some(Instruction::L)) => n.0.clone(),
            (Some(n), Some(Instruction::R)) => n.1.clone(),
            _ => {
                count = None;
                break;
            }
        };
    }

    return count;
}

fn gcd(a: u64, b: u64) -> u64 {
    if b > 0 {
        return gcd(b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

struct Result {
    part_1: Option<u64>,
    part_2: Option<u64>,
}
fn find_step_count(input: &str) -> Result {
    let (instructions, map) = input.split_once("\n\n").unwrap();

    let instructions = instructions
        .trim()
        .chars()
        .map(|c| Instruction::from(&c))
        .collect();

    let map = map.lines().fold(HashMap::new(), |mut acc, l| {
        let string = l.to_owned().replace(&['_', '(', ')', ',', '='], "");

        let mut pieces = string.split_whitespace().map(|l| l.to_string());

        let key = pieces.next().unwrap();
        let val = (pieces.next().unwrap(), pieces.next().unwrap());

        acc.insert(key, val);
        return acc;
    });

    let part_1 = traverse(String::from("AAA"), &map, &instructions, |l| l != "ZZZ");

    let visting = map.keys().filter(|k| k.ends_with("A")).map(|k| k.clone());

    let part_2 = visting
        .map(|start| traverse(start, &map, &instructions, |l| !l.ends_with("Z")))
        .fold(Some(1), |acc, val| Some(lcm(val.unwrap(), acc.unwrap())));

    return Result { part_1, part_2 };
}

fn main() {
    println!("Advent of Code, Day 8!");

    let file_path = args().nth(1).expect("Missing File Path!");

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let timer = Instant::now();

    let result = find_step_count(input.as_str());

    let time_taken = timer.elapsed();

    println!(
        "Day 8 Result, Part 1: {}",
        result.part_1.unwrap_or_default()
    );
    println!(
        "Day 8 Result, Part 2: {}",
        result.part_2.unwrap_or_default()
    );
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

        assert_eq!(result.part_1, Some(6));
    }

    #[test]
    fn test_find_ghost_step_count() {
        let input = "\
            LR\n\n\
            11A = (11B, XXX)\n\
            11B = (XXX, 11Z)\n\
            11Z = (11B, XXX)\n\
            22A = (22B, XXX)\n\
            22B = (22C, 22C)\n\
            22C = (22Z, 22Z)\n\
            22Z = (22B, 22B)\n\
            XXX = (XXX, XXX)\
        ";

        let result = find_step_count(input);

        assert_eq!(result.part_2, Some(6));
    }
}
