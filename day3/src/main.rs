use std::{fs, time::Instant};

#[derive(Clone, Debug, PartialEq)]
struct Part {
    value: u32,
    row: u32,
    cols: (u32, u32),
}

#[derive(Clone, Debug, PartialEq)]

struct Gear {
    row: u32,
    col: u32,
}

fn parse_line(input: &str, row: u32) -> (Vec<Part>, Vec<Gear>) {
    let mut parts: Vec<Part> = [].to_vec();
    let mut gears: Vec<Gear> = [].to_vec();

    let mut number_start: Option<usize> = None;

    for (index, char) in input.char_indices() {
        let is_digit = char.is_digit(10);
        let index_int: u32 = index.try_into().unwrap();
        let index_int = index_int;

        if char == '*' {
            gears.push(Gear {
                row,
                col: index_int,
            })
        }

        match (is_digit, number_start) {
            (true, None) => number_start = Some(index),
            (false, Some(start)) => {
                let number = input.get(start..index).unwrap();
                let number = u32::from_str_radix(number, 10).unwrap();
                let start_int: u32 = start.try_into().unwrap();
                parts.push(Part {
                    value: number,
                    row,
                    cols: (start_int, index_int),
                });
                number_start = None
            }
            _ => {}
        };
    }

    if number_start.is_some() {
        let start = number_start.unwrap();
        let number = input.get(start..).unwrap();
        let number = u32::from_str_radix(number, 10).unwrap();
        let start_int: u32 = start.try_into().unwrap();
        let end_int: u32 = input.len().try_into().unwrap();
        parts.push(Part {
            value: number,
            row,
            cols: (start_int, end_int),
        });
    }

    return (parts, gears);
}

fn parse_schematic(input: &str) -> (u32, u32) {
    let mut possible_parts: Vec<Part> = [].to_vec();
    let mut possible_gears: Vec<Gear> = [].to_vec();

    let lines: Vec<&str> = input.lines().collect();
    let lines_length: i32 = lines.len().try_into().unwrap();

    for row in 0..lines.len() {
        let row_int: u32 = row.try_into().unwrap();
        let (mut parts, mut gears) = parse_line(lines.get(row).unwrap(), row_int);

        possible_parts.append(&mut parts);
        possible_gears.append(&mut gears);
    }

    let mut parts_sum = 0;
    let mut gear_ratios_sum = 0;
    let mut true_parts: Vec<Part> = [].to_vec();

    for part in possible_parts {
        let mut matched = false;

        let row_above: i32 = part.row.try_into().unwrap();
        let row_above = (row_above - 1).max(0);

        let row_below: i32 = part.row.try_into().unwrap();
        let row_below = (row_below + 2).min(lines_length);

        let neighbor_rows = row_above..row_below;

        for row in neighbor_rows {
            if matched {
                break;
            }
            let (start_col, end_col) = part.cols;

            let col_before: i32 = start_col.try_into().unwrap();
            let col_before = (col_before - 1).max(0);

            let col_after: i32 = end_col.try_into().unwrap();
            let col_after = (col_after + 1).min(lines_length);

            let neighbor_cols = col_before..col_after;

            for col in neighbor_cols {
                let row_index: usize = row.try_into().unwrap();
                let col_index: usize = col.try_into().unwrap();
                let chars: Vec<char> = lines.get(row_index).unwrap().chars().collect();
                let char = chars.get(col_index).unwrap();
                let char = char.to_owned();
                if char != '.' && !char.is_digit(10) {
                    matched = true;
                    break;
                }
            }
        }

        if matched {
            parts_sum += part.value;
            true_parts.push(part);
        }
    }

    for gear in possible_gears {
        let row_above: i32 = gear.row.try_into().unwrap();
        let row_above = (row_above - 1).max(0);

        let row_below: i32 = gear.row.try_into().unwrap();
        let row_below = (row_below + 2).min(lines_length);

        let neighbor_rows = row_above..row_below;

        let mut neighbors: Vec<Part> = [].to_vec();

        for row in neighbor_rows {
            let col_before: i32 = gear.col.try_into().unwrap();
            let col_before = (col_before - 1).max(0);

            let col_after: i32 = gear.col.try_into().unwrap();
            let col_after = (col_after + 2).min(lines_length);

            let neighbor_cols = col_before..col_after;

            for col in neighbor_cols {
                let part = true_parts.iter()
                    .find(|p| (p.cols.0..p.cols.1).contains(&col.try_into().unwrap()) && p.row == row.try_into().unwrap());

                match part {
                    Some(p) => {
                        if !neighbors.contains(p) {
                            neighbors.push(p.clone())
                        }
                    },
                    None => {},
                }
            }
        }

        
        if neighbors.len() == 2 {
            let gear_ratio = neighbors.iter().fold(1, |acc, x| acc * x.value);
            gear_ratios_sum += gear_ratio;
        }
    }

    return (parts_sum, gear_ratios_sum);
}

fn main() {
    println!("Advent of Code, Day 3!");

    let input =
        fs::read_to_string("./src/day3.txt").expect("Should have been able to read the file");

    let timer = Instant::now();

    let (part_1, part_2) = parse_schematic(input.as_str());

    let time_taken = timer.elapsed();

    println!("Day 3 Result, Part 1: {}", part_1);
    println!("Day 3 Result, Part 2: {}", part_2);
    println!("Time Taken: {:?}", time_taken);
}

#[cfg(test)]
mod tests {

    use crate::{parse_line, parse_schematic, Gear, Part};

    #[test]
    fn parse_line_test() {
        let examples = [
            (
                "123",
                (
                    [Part {
                        value: 123,
                        row: 0,
                        cols: (0, 3),
                    }]
                    .to_vec(),
                    [].to_vec(),
                ),
            ),
            (
                ".123..456...",
                (
                    [
                        Part {
                            value: 123,
                            row: 0,
                            cols: (1, 4),
                        },
                        Part {
                            value: 456,
                            row: 0,
                            cols: (6, 9),
                        },
                    ]
                    .to_vec(),
                    [].to_vec(),
                ),
            ),
            (
                "1.123.$456..**.4",
                (
                    [
                        Part {
                            value: 1,
                            row: 0,
                            cols: (0, 1),
                        },
                        Part {
                            value: 123,
                            row: 0,
                            cols: (2, 5),
                        },
                        Part {
                            value: 456,
                            row: 0,
                            cols: (7, 10),
                        },
                        Part {
                            value: 4,
                            row: 0,
                            cols: (15, 16),
                        },
                    ]
                    .to_vec(),
                    [Gear { row: 0, col: 12 }, Gear { row: 0, col: 13 }].to_vec(),
                ),
            ),
        ];

        for (input, expected) in examples {
            let result = parse_line(input, 0);

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_parse_schematic() {
        let input = "\
            467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598..\
        ";

        let (part_1, part_2) = parse_schematic(input);

        assert_eq!(part_1, 4361);
        assert_eq!(part_2, 467835);
    }
}
