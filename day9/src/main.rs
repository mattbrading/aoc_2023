use aoc;
use std::collections::HashSet;
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
    fn next_point(&self) -> (Option<i64>, Option<i64>) {
        let (set, differences) = self.data.windows(2).fold(
            (HashSet::new(), Vec::new()),
            |acc: (HashSet<i64>, Vec<i64>), v| {
                let (mut set, mut vec) = acc;
                let difference = v[1] - v[0];
                set.insert(difference);
                vec.push(difference);
                return (set, vec);
            },
        );

        let all_same = set.len() == 1;

        let (n_start, n_end) = match all_same {
            true => {
                let diff = differences.last().map(|v| v.to_owned());
                (diff, diff)
            }
            false => Dataset { data: differences }.next_point(),
        };

        let start = self
            .data
            .first()
            .zip(n_start)
            .and_then(|(a, b)| Some(a - b));

        let end = self.data.last().zip(n_end).and_then(|(a, b)| Some(a + b));

        return (start, end);
    }
}

fn evaluate_datapoints(input: &str) -> aoc::Result {
    let (start, end) = input.lines().map(|l| Dataset::from(l).next_point()).fold(
        (None, None),
        |acc: (Option<i64>, Option<i64>), v: (Option<i64>, Option<i64>)| {
            (
                acc.0.map_or(v.0, |x| Some(x + v.0.unwrap())),
                acc.1.map_or(v.1, |x| Some(x + v.1.unwrap())),
            )
        },
    );

    return (
        end.map(|v| v.try_into().ok()).unwrap(),
        start.map(|v| v.try_into().ok()).unwrap(),
    );
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
            ("0 3 6 9 12 15", -3, 18),
            ("1 3 6 10 15 21", 0, 28),
            ("10 13 16 21 30 45", 5, 68),
        ];

        for (input, start, end) in examples {
            let result = Dataset::from(input).next_point();

            assert_eq!(result, (Some(start), Some(end)));
        }
    }

    #[test]
    fn test_sum_next_datapoints() {
        let input = "\
            0 3 6 9 12 15\n\
            1 3 6 10 15 21\n\
            10 13 16 21 30 45\
        ";

        let result = evaluate_datapoints(input);

        assert_eq!(result, (Some(114), Some(2)));
    }
}
