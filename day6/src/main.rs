use std::{fs, time::Instant};

fn count_winning_options(duration: &u64, min_distance: &u64) -> u64 {
    let rhs = duration.clone() as f64 / 2.0;
    let lhs: f64 = ((duration.pow(2) - 4 * min_distance) as f64).sqrt() / 2.0;

    let min_root = (rhs - lhs).floor() as u64 + 1;
    let max_root = (rhs + lhs).ceil() as u64;

    return max_root - min_root;
}

struct Result {
    part_1: u64,
    part_2: u64,
}

fn parse_input(input: &str) -> Result {
    let (time_str, dist_str) = input.split_once("\n").unwrap();

    let times = time_str
        .split_whitespace()
        .skip(1)
        .map(|str| str.parse::<u64>().unwrap());
    let dists = dist_str
        .split_whitespace()
        .skip(1)
        .map(|str| str.parse::<u64>().unwrap());

    let part_1 = times
        .zip(dists)
        .map(|(duration, min_distance)| count_winning_options(&duration, &min_distance))
        .fold(1, |acc, val| acc * val);

    let combined_time = time_str
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let combined_dist = dist_str
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let part_2 = count_winning_options(&combined_time, &combined_dist);

    return Result { part_1, part_2 };
}

fn main() {
    println!("Advent of Code, Day 6!");

    let input =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");

    let timer = Instant::now();

    let result = parse_input(input.as_str());

    let time_taken = timer.elapsed();

    println!("Day 6 Result, Part 1: {}", result.part_1);
    println!("Day 6 Result, Part 2: {}", result.part_2);
    println!("Time Taken: {:?}", time_taken);
}

#[cfg(test)]
mod tests {
    use crate::{count_winning_options, parse_input};

    #[test]

    fn test_count_winning_options() {
        let races = [(7, 9, 4), (15, 40, 8), (30, 200, 9), (71530, 940200, 71503)];

        for (duration, min_distance, expected) in races {
            let result = count_winning_options(&duration, &min_distance);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_parse_input() {
        let input = "\
            Time:      7  15   30\n\
            Distance:  9  40  200\
        ";

        let result = parse_input(input);
        assert_eq!(result.part_1, 288);
        assert_eq!(result.part_2, 71503);
    }
}
