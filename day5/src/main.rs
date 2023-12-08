use aoc;
use std::{collections::BTreeMap, ops::Range};

type Map = (u64, u64, u64);

type MapTree = BTreeMap<u64, (u64, u64)>;

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

struct ReduceRanges {
    found_ranges: Vec<Range<u64>>,
    remaining_range: Option<Range<u64>>,
}
fn find_mapped_ranges(input: Range<u64>, map: &MapTree) -> Vec<Range<u64>> {
    let result = map.range(..input.end).fold(
        ReduceRanges {
            found_ranges: [].to_vec(),
            remaining_range: Some(input),
        },
        |mut acc: ReduceRanges, val| {
            return match acc.remaining_range {
                Some(remaining_range) => {
                    let (source, (dest, length)) = val;

                    let compare_range = source.to_owned()..source + length;

                    let contains_start = compare_range.contains(&remaining_range.start);
                    let contains_end = compare_range.contains(&remaining_range.end);

                    let (found, remaining) = match (contains_start, contains_end) {
                        (true, true) => (Some(remaining_range), None),
                        (true, false) => (
                            Some(remaining_range.start..compare_range.end),
                            Some(compare_range.end..remaining_range.end),
                        ),
                        (false, true) => (
                            Some(compare_range.start..remaining_range.end),
                            Some(remaining_range.start..compare_range.start),
                        ),
                        (false, false) => (None, Some(remaining_range)),
                    };

                    match found {
                        Some(range) => {
                            let start_offset = range.start - compare_range.start;
                            let end_offset = range.end - compare_range.start;
                            let dest_start = dest + start_offset;
                            let dest_end = dest + end_offset;
                            acc.found_ranges.push(dest_start..dest_end);
                        }
                        None => {}
                    };

                    acc.remaining_range = remaining;

                    return acc;
                }
                None => acc,
            };
        },
    );

    let mut ranges = result.found_ranges;

    match result.remaining_range {
        Some(range) => ranges.push(range),
        None => {}
    }
    return ranges;
}

fn find_best_location(input: &str) -> aoc::Result {
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

    let part_1 = seeds
        .clone()
        .map(|seed| maps.clone().fold(seed, |acc, map| find_map_dest(acc, &map)))
        .min()
        .expect("No seeds in input!");

    let map_trees: Vec<MapTree> = maps
        .map(|m| {
            m.iter().fold(BTreeMap::new(), |mut acc, val| {
                acc.insert(val.1, (val.0, val.2));
                return acc;
            })
        })
        .collect();

    let part_2 = seeds
        .clone()
        .step_by(2)
        .zip(seeds.clone().skip(1).step_by(2))
        .map(|(start, len)| start..start + len)
        .map(|seed| {
            map_trees
                .iter()
                .fold([seed].to_vec(), |acc: Vec<Range<u64>>, map_tree| {
                    acc.iter()
                        .flat_map(|range| find_mapped_ranges(range.to_owned(), map_tree))
                        .collect()
                })
                .iter()
                .map(|r| r.start)
                .min()
                .expect("No result found!")
        })
        .min()
        .expect("No seeds in input!");

    return (Some(part_1), Some(part_2));
}

fn main() {
    aoc::run(5, find_best_location);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{find_best_location, find_map_dest, find_mapped_ranges, MapTree};

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
    fn test_find_mapped_ranges() {
        let map: MapTree = BTreeMap::from([(50, (52, 20)), (98, (50, 2))]);

        let examples = [(50..99, [(52..72), (50..51), (70..98)].to_vec())];

        for (input, expected) in examples {
            let result = find_mapped_ranges(input, &map);

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

        let (part_1, part_2) = find_best_location(input);

        assert_eq!(part_1, Some(35));
        assert_eq!(part_2, Some(46));
    }
}
