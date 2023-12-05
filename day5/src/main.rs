use std::{fs, time::Instant};

type Map = (u64, u64, u64);

fn find_map_dest(input: u64, maps: &Vec<Map>) -> u64 {
    let found = maps
        .iter()
        .find(|(_, start, len)| (start.to_owned()..start + len).contains(&input));

    match found {
        Some((dest_start, source_start, _)) => {
            let offset = input - source_start;
            return dest_start + offset;
        }
        None => input,
    }
}

fn find_best_location(input: &str) -> u64 {
    let (seeds_line, maps) = input.split_once("\n\n").unwrap();

    let seeds = seeds_line
        .split_once(": ")
        .and_then(|l| Some(l.1))
        .expect("Failed to parse seed line")
        .split_whitespace()
        .map(|i| u64::from_str_radix(i, 10).expect("Invalid seed number"));

    let maps = maps.split("\n\n").map(|maps| {
        maps.lines()
            .skip(1)
            .map(|l| {
                let mut items = l
                    .splitn(3, " ")
                    .map(|i| u64::from_str_radix(i, 10).unwrap());

                return (
                    items.next().unwrap(),
                    items.next().unwrap(),
                    items.next().unwrap(),
                );
            })
            .collect::<Vec<Map>>()
    });

    return seeds
        .map(|seed| maps.clone().fold(seed, |acc, map| find_map_dest(acc, &map)))
        .min()
        .expect("No seeds in input!");
}

fn main() {
    println!("Advent of Code, Day 5!");

    let input =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");

    let timer = Instant::now();

    let result = find_best_location(input.as_str());

    let time_taken = timer.elapsed();

    println!("Day 5 Result, Part 1: {}", result);
    println!("Time Taken: {:?}", time_taken);
}

#[cfg(test)]
mod tests {
    use crate::{find_best_location, find_map_dest};

    #[test]
    fn test_find_map_dest() {
        let maps = [(50, 98, 2), (52, 50, 48)].to_vec();

        let examples = [(99, 51), (53, 55), (15, 15)];

        for (input, expected) in examples {
            let result = find_map_dest(input, &maps);

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_find_best_location() {
        let input = "\
            seeds: 79 14 55 13\n\n\
            seed-to-soil map:\n\
            50 98 2\n\
            52 50 48\n\n\
            soil-to-fertilizer map:\n\
            0 15 37\n\
            37 52 2\n\
            39 0 15\n\n\
            fertilizer-to-water map:\n\
            49 53 8\n\
            0 11 42\n\
            42 0 7\n\
            57 7 4\n\n\
            water-to-light map:\n\
            88 18 7\n\
            18 25 70\n\n\
            light-to-temperature map:\n\
            45 77 23\n\
            81 45 19\n\
            68 64 13\n\n\
            temperature-to-humidity map:\n\
            0 69 1\n\
            1 0 69\n\n\
            humidity-to-location map:\n\
            60 56 37\n\
            56 93 4\
        ";

        let result = find_best_location(input);

        assert_eq!(result, 35)
    }
}
