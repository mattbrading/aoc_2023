use {
  once_cell::sync::Lazy,
  regex::Regex,
};

pub fn document_calbration_sum (input: &str) -> u32 {
  let mut result = 0;

  for line in input.lines() {
    result += get_calibration_value(line)
  }

  return result;
} 

fn string_to_num (input: &str) -> u32 {
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
    _ => u32::from_str_radix(input, 10).unwrap()
  };
}

fn get_calibration_value (input: &str) -> u32 {
  static RE_FIRST: Lazy<Regex> = Lazy::new(|| Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[1-9]).*").unwrap());
  static RE_LAST: Lazy<Regex> = Lazy::new(|| Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|[1-9])").unwrap());

  let first = RE_FIRST.captures(input).unwrap().get(1).unwrap().as_str();
  let last = RE_LAST.captures(input).unwrap().get(1).unwrap().as_str();

  let first = string_to_num(first) * 10;
  let last = string_to_num(last);

  let result = first + last;

  return result;
}


#[cfg(test)]
mod tests {
    use crate::day1::get_calibration_value;
    use crate::day1::document_calbration_sum;

    #[test]
    fn value_by_line() {
        let examples = [
          ("1abc2", 12),
          ("pqr3stu8vwx", 38),
          ("a1b2c3d4e5f", 15),
          ("treb7uchet", 77),
        ];

        for (input, output) in examples {
          let result = get_calibration_value(input);
          assert_eq!(result, output);
        }
    }

    #[test]
    fn document_sum_value () {
      let example = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
      let result = document_calbration_sum(example);
      assert_eq!(result, 142);
    }


    #[test]
    fn value_by_line_part_two() {
        let examples = [
          ("two1nine", 29),
          ("eightwothree", 83),
          ("abcone2threexyz", 13),
          ("xtwone3four", 24),
          ("4nineeightseven2", 42),
          ("zoneight234", 14),
          ("7pqrstsixteen", 76),
        ];

        for (input, output) in examples {
          let result = get_calibration_value(input);
          assert_eq!(result, output);
        }
    }

    #[test]
    fn document_sum_value_part_two () {
      let example = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
      let result = document_calbration_sum(example);
      assert_eq!(result, 281);
    }

    #[test]
    fn value_by_line_overlapping() {
        let examples = [
          ("abdoneightabd", 18),
          ("2abdoneightabd", 28),
        ];

        for (input, output) in examples {
          let result = get_calibration_value(input);
          assert_eq!(result, output);
        }
    }
}