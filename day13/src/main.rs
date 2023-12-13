use array2d::Array2D;
use std::usize;

use aoc;

struct Pattern {
    map: Array2D<char>,
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        let map: Vec<Vec<char>> = value.lines().map(|r| r.chars().collect()).collect();
        let map = Array2D::from_rows(&map).unwrap();

        return Self { map };
    }
}

enum Direction {
    Vertical,
    Horizontal,
}

impl Pattern {
    fn is_reflection(&self, idx: usize, direction: &Direction) -> bool {
        let total_len = match direction {
            Direction::Vertical => self.map.row_len(),
            Direction::Horizontal => self.map.column_len(),
        };

        let range_before = (0..idx + 1).len();
        let range_after = (idx + 1..total_len).len();
        let smallest_range = range_before.min(range_after);

        let search = match direction {
            Direction::Vertical => self.map.as_columns(),
            Direction::Horizontal => self.map.as_rows(),
        };

        let before = search[idx + 1 - smallest_range..idx + 1].to_vec();

        let after = search[idx + 1..idx + 1 + smallest_range].to_vec();

        !before.iter().rev().zip(after.iter()).any(|(a, b)| a != b)
    }

    fn find_reflection(&self, direction: Direction) -> u32 {
        let total_len = match direction {
            Direction::Vertical => self.map.row_len(),
            Direction::Horizontal => self.map.column_len(),
        };

        (0..total_len - 1)
            .filter(|c| self.is_reflection(*c, &direction))
            .map(|v| v + 1)
            .sum::<usize>() as u32
    }

    fn find_reflection_score(&self) -> u32 {
        let v_score = self.find_reflection(Direction::Vertical);
        let h_score = self.find_reflection(Direction::Horizontal) * 100;
        return v_score + h_score;
    }
}

fn main() {
    aoc::run(13, |input| {
        let part_1 = input
            .split("\n\n")
            .map(|l| Pattern::from(l).find_reflection_score())
            .sum::<u32>() as u64;

        return (Some(part_1), None);
    });
}

#[cfg(test)]
mod tests {
    use crate::Pattern;

    #[test]
    fn test_find_reflection_vertical() {
        let input = "\
            #.##..##.\n\
            ..#.##.#.\n\
            ##......#\n\
            ##......#\n\
            ..#.##.#.\n\
            ..##..##.\n\
            #.#.##.#.\
        ";

        let result = Pattern::from(input).find_reflection_score();

        assert_eq!(result, 5);
    }

    #[test]
    fn test_find_reflection_horizontal() {
        let input = "\
            #...##..#\n\
            #....#..#\n\
            ..##..###\n\
            #####.##.\n\
            #####.##.\n\
            ..##..###\n\
            #....#..#\
        ";

        let result = Pattern::from(input).find_reflection_score();

        assert_eq!(result, 400);
    }
}
