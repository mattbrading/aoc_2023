use std::{collections::HashMap, ops::Mul};

use array2d::Array2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Rounded,
    Obsticle,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Rounded,
            '#' => Self::Obsticle,
            '.' => Self::Empty,
            _ => panic!("Invalid Char"),
        }
    }
}

struct Dish {
    grid: Array2D<Tile>,
}

impl From<&str> for Dish {
    fn from(value: &str) -> Self {
        let rows: Vec<Vec<Tile>> = value
            .lines()
            .map(|r| r.chars().map(|c| Tile::from(c)).collect())
            .collect();

        let grid = Array2D::from_rows(&rows).unwrap();

        return Dish { grid };
    }
}

impl ToString for Dish {
    fn to_string(&self) -> String {
        self.grid
            .rows_iter()
            .map(|col| {
                col.map(|tile| match tile {
                    Tile::Empty => ".",
                    Tile::Obsticle => "#",
                    Tile::Rounded => "O",
                })
                .collect::<Vec<&str>>()
                .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

enum Direction {
    N,
    E,
    S,
    W,
}

impl Dish {
    fn tilt(&mut self, direction: Direction) -> &mut Self {
        let source = match direction {
            Direction::N | Direction::S => self.grid.as_columns(),
            Direction::E | Direction::W => self.grid.as_rows(),
        };

        let new_cols: Vec<Vec<Tile>> = source
            .iter()
            .map(|row| {
                let mut free_space = None;

                let row: Vec<(usize, &Tile)> = match direction {
                    Direction::N | Direction::W => row.iter().enumerate().collect(),
                    Direction::S | Direction::E => row.iter().rev().enumerate().collect(),
                };

                let row = row
                    .iter()
                    .copied()
                    .fold(vec![], |mut acc, (row_idx, tile)| {
                        acc.push(*tile);
                        match (tile, free_space) {
                            (Tile::Empty, None) => {
                                free_space = Some(row_idx);
                            }
                            (Tile::Obsticle, Some(_)) => {
                                free_space = None;
                            }
                            (Tile::Rounded, Some(free_row)) => {
                                acc[free_row] = Tile::Rounded;
                                acc[row_idx] = Tile::Empty;
                                free_space = Some(free_row + 1);
                            }
                            _ => {}
                        };

                        return acc;
                    });

                match direction {
                    Direction::N | Direction::W => row,
                    Direction::S | Direction::E => row.iter().rev().copied().collect(),
                }
            })
            .collect();

        self.grid = match direction {
            Direction::N | Direction::S => Array2D::from_columns(&new_cols).unwrap(),
            Direction::E | Direction::W => Array2D::from_rows(&new_cols).unwrap(),
        };

        return self;
    }

    fn total_load(&self) -> u64 {
        self.grid
            .rows_iter()
            .rev()
            .enumerate()
            .map(|(idx, row)| row.filter(|c| **c == Tile::Rounded).count().mul(idx + 1) as u64)
            .sum()
    }

    fn cycle(&mut self) -> &Self {
        self.tilt(Direction::N)
            .tilt(Direction::W)
            .tilt(Direction::S)
            .tilt(Direction::E);

        return self;
    }

    fn cycle_repeat(mut self, count: u64) -> Self {
        let mut seen = HashMap::from([(self.to_string(), 0)]);

        let mut cycle_count = 0;
        let cycle_len;

        loop {
            cycle_count += 1;
            println!("{}", cycle_count);
            self.cycle();
            let grid_str = self.to_string();
            if seen.contains_key(&grid_str) {
                cycle_len = cycle_count - seen.get(&grid_str).unwrap();
                break;
            } else {
                seen.insert(grid_str, cycle_count);
            }
        }

        let remaining_cycles = (count - cycle_count) % cycle_len;

        for _ in 0..remaining_cycles {
            self.cycle();
        }

        return self;
    }
}

fn main() {
    aoc::run(14, |input| {
        let part_1 = Dish::from(input).tilt(Direction::N).total_load();
        let part_2 = Dish::from(input).cycle_repeat(1_000_000_000).total_load();

        return (Some(part_1), Some(part_2));
    });
}
#[cfg(test)]
mod tests {
    use crate::Dish;

    const INPUT: &str = "\
        O....#....\n\
        O.OO#....#\n\
        .....##...\n\
        OO.#O....O\n\
        .O.....O#.\n\
        O.#..O.#.#\n\
        ..O..#O..O\n\
        .......O..\n\
        #....###..\n\
        #OO..#....\
    ";

    #[test]
    fn test_total_load_north() {
        let result = Dish::from(INPUT).tilt(crate::Direction::N).total_load();
        assert_eq!(result, 136);
    }

    #[test]
    fn test_total_load_cycled() {
        let result = Dish::from(INPUT).cycle_repeat(1_000_000_000).total_load();
        assert_eq!(result, 64);
    }
}
