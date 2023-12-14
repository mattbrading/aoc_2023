use array2d::Array2D;
use std::usize;

use aoc;

struct Pattern {
    map: Array2D<char>,
    allowed_smudges: u64,
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        let map: Vec<Vec<char>> = value.lines().map(|r| r.chars().collect()).collect();
        let map = Array2D::from_rows(&map).unwrap();

        return Self { map, allowed_smudges: 0 };
    }
}

enum Direction {
    Vertical,
    Horizontal,
}

impl Pattern {

    fn allow_smudges(mut self, amount: u64) -> Self{
        self.allowed_smudges = amount;
        return self
    }

    fn is_reflection(&self, idx: usize, direction: &Direction) -> bool {
        let search = match direction {
            Direction::Vertical => self.map.as_columns(),
            Direction::Horizontal => self.map.as_rows(),
        };

        let before = search[..idx + 1].to_vec();
        let after = search[idx + 1..].to_vec();

        let smudges: u64 = before.iter().rev()
            .zip(after.iter())
            .map(|(a, b)| a.iter().zip(b.iter()).filter(|(x, y)| x != y).count() as u64)
            .sum();

        return smudges == self.allowed_smudges;

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

        let part_2 = input
            .split("\n\n")
            .map(|l| Pattern::from(l).allow_smudges(1).find_reflection_score())
            .sum::<u32>() as u64;

        return (Some(part_1), Some(part_2));
    });
}

#[cfg(test)]
mod tests {
    use crate::Pattern;

    #[test]
    fn test_find_reflection_pattern_a() {
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

        let result = Pattern::from(input).allow_smudges(1).find_reflection_score();

        assert_eq!(result, 300);
    }

    #[test]
    fn test_find_reflection_pattern_b() {
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

        let result = Pattern::from(input).allow_smudges(1).find_reflection_score();

        assert_eq!(result, 100);
    }
}
