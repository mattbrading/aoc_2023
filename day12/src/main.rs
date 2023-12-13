use std::collections::HashMap;

fn sum_possible_arrangements(input: &str, unfolded: bool) -> u64 {
    input
        .lines()
        .enumerate()
        .inspect(|(i, _)| println!("{}", i))
        .map(|(_, l)| process_line(l, unfolded))
        .sum()
}

fn process_line(input: &str, unfolded: bool) -> u64 {
    let (springs, groups) = input.split_once(" ").unwrap();

    let mut cache = HashMap::new();

    let springs: String = springs
        .chars()
        .chain(['?'])
        .cycle()
        .take({
            if unfolded {
                springs.len() * 5 + 4
            } else {
                springs.len()
            }
        })
        .collect::<String>();

    let groups: Vec<u64> = groups
        .trim()
        .split(",")
        .map(|v| v.parse().expect(format!("Invalid Digit: |{}|", v).as_str()))
        .collect();

    let groups = groups
        .iter()
        .copied()
        .cycle()
        .take({
            if unfolded {
                groups.len() * 5
            } else {
                groups.len()
            }
        })
        .collect();

    return find_possible_arrangements(springs.as_str(), &groups, &mut cache);
}

fn find_possible_arrangements(
    springs: &str,
    groups: &Vec<u64>,
    cache: &mut HashMap<(Vec<u64>, String), u64>,
) -> u64 {
    let cache_key = (groups.clone(), springs.to_string());

    if groups.is_empty() {
        if springs.contains("#") {
            return 0;
        } else {
            return 1;
        }
    }

    if cache.contains_key(&cache_key) {
        return cache[&cache_key];
    }

    let mut groups = groups.clone();

    let mut possible_arrangements = 0;

    let mut counter = 0;
    let mut slice_point = 0;
    for (index, char) in springs.chars().enumerate() {
        match (char, counter) {
            ('#', 0) => {
                slice_point = index;
                counter += 1;
            }
            ('#', _) => counter += 1,
            ('.', 0) => {}
            ('.', _) => {
                if groups.first().is_some_and(|v| *v == counter) {
                    slice_point = index;
                    counter = 0;
                    groups.remove(0);
                } else {
                    return 0;
                }
            }
            ('?', _) => {
                let mut str_a = springs.to_string();
                let mut str_b = springs.to_string();
                str_a.replace_range(index..index + 1, ".");
                str_b.replace_range(index..index + 1, "#");
                possible_arrangements +=
                    find_possible_arrangements(&str_a[slice_point..], &groups, cache);
                possible_arrangements +=
                    find_possible_arrangements(&str_b[slice_point..], &groups, cache);

                cache.insert(cache_key, possible_arrangements);

                return possible_arrangements;
            }
            _ => {}
        }
    }

    if counter > 0 {
        if groups.first().is_some_and(|v| *v == counter) {
            groups.remove(0);
        } else {
            return 0;
        }
    }

    if groups.is_empty() {
        possible_arrangements += 1;
    }

    cache.insert(cache_key, possible_arrangements);

    return possible_arrangements;
}

fn main() {
    aoc::run(12, |input| {
        let part_1 = sum_possible_arrangements(input, false);
        let part_2 = sum_possible_arrangements(input, true);

        return (Some(part_1), Some(part_2));
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
            let result = process_line(input, false);

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_find_possible_arrangements_unfolded() {
        let examples = [
            ("#.#.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 16384),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 16),
            ("????.######..#####. 1,6,5", 2500),
            ("?###???????? 3,2,1", 506250),
        ];

        for (input, expected) in examples {
            let result = process_line(input, true);

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

        let result = sum_possible_arrangements(input, false);

        assert_eq!(result, 21);
    }

    #[test]
    fn test_sum_possible_arrangements_unfolded() {
        let input = "\
            #.#.### 1,1,3\n\
            .??..??...?##. 1,1,3\n\
            ?#?#?#?#?#?#?#? 1,3,1,6\n\
            ????.#...#... 4,1,1\n\
            ????.######..#####. 1,6,5\n\
            ?###???????? 3,2,1\
        ";

        let result = sum_possible_arrangements(input, true);

        assert_eq!(result, 525152);
    }
}
