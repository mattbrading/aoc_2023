use std::{time::Instant, fs};

#[derive(Clone, Debug, PartialEq)]
struct Part {
    value: u32,
    row: u32,
    cols: (u32, u32),
}

fn parse_line(input: &str, row: u32) -> Vec<Part> {
    let mut parts: Vec<Part> = [].to_vec();

    let mut number_start: Option<usize> = None;

    for (index, char) in input.char_indices() {
        let is_digit = char.is_digit(10);
        let index_int: u32 = index.try_into().unwrap();
        let index_int = index_int;

        match (is_digit, number_start) {
            (true, None) => number_start = Some(index),
            (false, Some(start)) => {
                let number = input.get(start..index).unwrap();
                let number = u32::from_str_radix(number, 10).unwrap();
                let start_int: u32 = start.try_into().unwrap();
                parts.push(Part {
                    value: number,
                    row,
                    cols: (start_int, index_int - 1),
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
            cols: (start_int, end_int - 1),
        });
    }

    return parts;
}

fn parse_schematic(input: &str) -> u32 {
    let mut possible_parts: Vec<Part> = [].to_vec();

    let lines: Vec<&str> = input.lines().collect();
    let lines_length: i32 = lines.len().try_into().unwrap();

    for row in 0..lines.len() {
        let row_int: u32 = row.try_into().unwrap();
        let mut parts = parse_line(lines.get(row).unwrap(), row_int);

        possible_parts.append(&mut parts)
    }

    let mut sum = 0;

    for part in possible_parts {
        let mut matched = false;

        println!("Comparing: {}", part.value);


        let row_above:i32 = part.row.try_into().unwrap();
        let row_above = (row_above - 1).max(0);
        
        let row_below:i32 = part.row.try_into().unwrap();
        let row_below = (row_below + 2).min(lines_length);
        
        let neighbor_rows = row_above..row_below;

        println!("Rows: {:?}", neighbor_rows);

        for row in neighbor_rows {
            if matched {break;}
            let (start_col, end_col) = part.cols;

            let col_before:i32 = start_col.try_into().unwrap();
            let col_before = (col_before - 1).max(0);
            
            let col_after:i32 = end_col.try_into().unwrap();
            let col_after = (col_after + 2).min(lines_length);
            
            let neighbor_cols = col_before..col_after;
            println!("Cols: {:?}", neighbor_cols);
        
            for col in neighbor_cols {
                let row_index: usize = row.try_into().unwrap();
                let col_index: usize = col.try_into().unwrap();
                let chars:Vec<char> = lines.get(row_index).unwrap().chars().collect();
                let char = chars.get(col_index).unwrap();
                let char = char.to_owned();
                println!("{}", char);

                if char != '.' && !char.is_digit(10) {
                    println!("Matched!: {}", char);
                    matched = true;
                    break;
                }

            }
        }

        if matched {
            println!("Matched: {}", part.value);
            sum += part.value;
        }
    }

    return sum;
}

fn main() {
    println!("Advent of Code, Day 3!");

    let input =
    fs::read_to_string("./src/day3.txt").expect("Should have been able to read the file");

    let timer = Instant::now();

    let result = parse_schematic(input.as_str());

    let time_taken = timer.elapsed();

    println!("Day 3 Result, Part 1: {}", result);
    println!("Time Taken: {:?}", time_taken);
}

#[cfg(test)]
mod tests {

    use crate::{parse_line, parse_schematic, Part};

    #[test]
    fn parse_line_test() {
        let examples = [
            (
                "123",
                [Part {
                    value: 123,
                    row: 0,
                    cols: (0, 2),
                }]
                .to_vec(),
            ),
            (
                ".123..456...",
                [
                    Part {
                        value: 123,
                        row: 0,
                        cols: (1, 3),
                    },
                    Part {
                        value: 456,
                        row: 0,
                        cols: (6, 8),
                    },
                ]
                .to_vec(),
            ),
            (
                "1.123.$456..**.4",
                [
                    Part {
                        value: 1,
                        row: 0,
                        cols: (0, 0),
                    },
                    Part {
                        value: 123,
                        row: 0,
                        cols: (2, 4),
                    },
                    Part {
                        value: 456,
                        row: 0,
                        cols: (7, 9),
                    },
                    Part {
                        value: 4,
                        row: 0,
                        cols: (15, 15),
                    },
                ]
                .to_vec(),
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

        let result = parse_schematic(input);


        assert_eq!(result, 4361);
    }
}
