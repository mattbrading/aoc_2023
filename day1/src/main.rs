use aoc;

use {once_cell::sync::Lazy, regex::Regex};

fn document_calbration_sum(input: &str) -> aoc::Result {
    let part_1 = input
        .lines()
        .map(get_calibration_value_a)
        .fold(None, |acc: Option<u64>, val| match val {
            Some(v) => acc.and_then(|x| Some(x + v)).or(Some(v)),
            None => acc,
        });

    let part_2 = input.lines().map(get_calibration_value_b).sum();

    return (part_1, Some(part_2));
}

fn string_to_num(input: &str) -> u64 {
    return match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => u64::from_str_radix(input, 10).unwrap(),
    };
}
fn get_calibration_value_a(input: &str) -> Option<u64> {
    let nums = input.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();

    match (nums.first(), nums.last()) {
        (Some(a), Some(b)) => [*a, *b].iter().collect::<String>().parse::<u64>().ok(),
        _ => None,
    }
}

fn get_calibration_value_b(input: &str) -> u64 {
    static RE_FIRST: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[1-9]).*").unwrap()
    });
    static RE_LAST: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|[1-9])").unwrap()
    });

    let first = RE_FIRST.captures(input).unwrap().get(1).unwrap().as_str();
    let last = RE_LAST.captures(input).unwrap().get(1).unwrap().as_str();

    let first = string_to_num(first) * 10;
    let last = string_to_num(last);

    let result = first + last;

    return result;
}

fn main() {
    aoc::run(1, document_calbration_sum);
}

#[cfg(test)]
mod tests {
    use crate::{document_calbration_sum, get_calibration_value_a, get_calibration_value_b};

    #[test]
    fn value_by_line() {
        let examples = [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];

        for (input, output) in examples {
            let result = get_calibration_value_a(input);
            assert_eq!(result, Some(output));
        }
    }

    #[test]
    fn document_sum_value() {
        let example = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let (result, _) = document_calbration_sum(example);
        assert_eq!(result, Some(142));
    }

    #[test]
    fn value_by_line_part_two() {
        let examples = [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];

        for (input, output) in examples {
            let result = get_calibration_value_b(input);
            assert_eq!(result, output);
        }
    }

    #[test]
    fn document_sum_value_part_two() {
        let example = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let (_, result) = document_calbration_sum(example);
        assert_eq!(result, Some(281));
    }

    #[test]
    fn value_by_line_overlapping() {
        let examples = [("abdoneightabd", 18), ("2abdoneightabd", 28)];

        for (input, output) in examples {
            let result = get_calibration_value_b(input);
            assert_eq!(result, output);
        }
    }
}
