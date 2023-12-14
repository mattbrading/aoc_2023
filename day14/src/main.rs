use std::ops::Mul;

use array2d::Array2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Dish {
    fn tilt(mut self) -> Self {
        let new_cols: Vec<Vec<Tile>> = self
            .grid
            .columns_iter()
            .map(|row| {
                let mut free_space = None;
                row.enumerate().fold(vec![], |mut acc, (row_idx, tile)| {
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
                })
            })
            .collect();

        self.grid = Array2D::from_columns(&new_cols).unwrap();

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
}

fn main() {
    aoc::run(14, |input| {
        let part_1 = Dish::from(input).tilt().total_load();

        return (Some(part_1), None);
    });
}
#[cfg(test)]
mod tests {
    use crate::Dish;

    #[test]
    fn test_total_load_north() {
        let input = "\
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

        let result = Dish::from(input).tilt().total_load();

        assert_eq!(result, 136);
    }
}
