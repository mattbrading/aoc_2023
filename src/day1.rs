pub fn document_calbration_sum (input: &str) -> u32 {
  let mut result = 0;

  for line in input.lines() {
    result += get_calibration_value(line)
  }

  return result;
} 

fn get_calibration_value (input: &str) -> u32 {
  let mut first: u32 = 0;
  let mut last: u32 = 0;

  for character in input.chars() {
   match character.to_digit(10) {
       Some(val) => {
        if first == 0 {
          first = val * 10
        }
        last = val
       },
       None => {}
   }
  }

  return first + last;
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
}