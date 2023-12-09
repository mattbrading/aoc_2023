use std::collections::HashSet;

/*
10  13  16  21  30  45  68
   3   3   5   9  15  23
     0   2   4   6   8
       2   2   2   2
         0   0   0
*/
use aoc;
struct Dataset {
    data: Vec<i64>,
}

impl From<&str> for Dataset {
    fn from(value: &str) -> Self {
        let data = value
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();

        return Self { data };
    }
}

impl Dataset {
    fn next_point(&self) -> Option<i64> {
        let (set, differences) = self.data.windows(2)
            .fold((HashSet::new(), Vec::new()), |acc: (HashSet<i64>, Vec<i64>), v| {
                let (mut set, mut vec) = acc;
                let difference = v[1] - v[0];
                set.insert(difference);
                vec.push(difference);
                return (set, vec)
            });

        let all_same = set.len() == 1;

        if all_same {
            return self
                .data
                .last()
                .zip(differences.last())
                .and_then(|(a, b)| Some(a + b));
        } else {
            return self
                .data
                .last()
                .zip(Dataset { data: differences }.next_point())
                .and_then(|(a, b)| Some(a + b));
        }
    }
}

fn evaluate_datapoints(input: &str) -> aoc::Result {
    let part_1 = input
        .lines()
        .map(|l| Dataset::from(l).next_point())
        .sum::<Option<i64>>()
        .map(|v| v.try_into().ok())
        .unwrap();

    return (part_1, None);
}

fn main() {
    aoc::run(9, evaluate_datapoints);
}

#[cfg(test)]
mod tests {
    use crate::{evaluate_datapoints, Dataset};

    #[test]
    fn test_next_datapoint() {
        let examples = [
            ("0 3 6 9 12 15", 18),
            ("1 3 6 10 15 21", 28),
            ("10 13 16 21 30 45", 68),
        ];

        for (input, expected) in examples {
            let result = Dataset::from(input).next_point();

            assert_eq!(result, Some(expected));
        }
    }

    #[test]
    fn test_sum_next_datapoints() {
        let input = "\
            0 3 6 9 12 15\n\
            1 3 6 10 15 21\n\
            10 13 16 21 30 45\
        ";

        let (result, _) = evaluate_datapoints(input);

        assert_eq!(result, Some(114));
    }
}
