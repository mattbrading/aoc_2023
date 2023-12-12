fn sum_possible_arrangements(input: &str) -> u32 {
    input
        .lines()
        .enumerate()
        .inspect(|(i, _)| println!("{}", i))
        .map(|(_, l)| process_line(l))
        .sum()
}

fn process_line(input: &str) -> u32 {
    let (springs, groups) = input.split_once(" ").unwrap();

    let groups: Vec<u32> = groups
        .trim()
        .split(",")
        .map(|v| v.parse().expect(format!("Invalid Digit: |{}|", v).as_str()))
        .collect();

    return find_possible_arrangements(springs, &groups);
}

fn find_possible_arrangements(springs: &str, groups: &Vec<u32>) -> u32 {
    let mut found_groups = vec![];

    let mut possible_arrangements = 0;

    let mut counter = 0;
    for (index, char) in springs.chars().enumerate() {
        match (char, counter) {
            ('#', _) => counter += 1,
            ('.', 0) => {}
            ('.', _) => {
                found_groups.push(counter);
                counter = 0;
            }
            ('?', _) => {
                let mut str_a = springs.to_string();
                let mut str_b = springs.to_string();
                str_a.replace_range(index..index + 1, ".");
                str_b.replace_range(index..index + 1, "#");
                possible_arrangements += find_possible_arrangements(&str_a, &groups);
                possible_arrangements += find_possible_arrangements(&str_b, &groups);
                return possible_arrangements;
            }
            _ => {}
        }
    }

    if counter > 0 {
        found_groups.push(counter);
    }

    if *groups == found_groups {
        possible_arrangements += 1;
    }

    return possible_arrangements;
}

fn main() {
    aoc::run(12, |input| {
        let part_1 = sum_possible_arrangements(input) as u64;

        return (Some(part_1), None);
    })
}

#[cfg(test)]
mod tests {
    use crate::{process_line, sum_possible_arrangements};

    #[test]
    fn test_find_possible_arrangements() {
        let examples = [
            ("#.#.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 4),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 1),
            ("????.######..#####. 1,6,5", 4),
            ("?###???????? 3,2,1", 10),
        ];

        for (input, expected) in examples {
            let result = process_line(input);

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_sum_possible_arrangements() {
        let input = "\
            #.#.### 1,1,3\n\
            .??..??...?##. 1,1,3\n\
            ?#?#?#?#?#?#?#? 1,3,1,6\n\
            ????.#...#... 4,1,1\n\
            ????.######..#####. 1,6,5\n\
            ?###???????? 3,2,1\
        ";

        let result = sum_possible_arrangements(input);

        assert_eq!(result, 21);
    }
}
