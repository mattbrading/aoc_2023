use std::collections::HashMap;

use aoc;
use array2d::Array2D;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}
struct Pipe {
    directions: HashMap<Direction, Direction>,
}

impl Pipe {
    fn from_char(value: &char) -> Option<Self> {
        match value {
            '-' => Some(Self {
                directions: HashMap::from([
                    (Direction::East, Direction::East),
                    (Direction::West, Direction::West),
                ]),
            }),
            '|' => Some(Self {
                directions: HashMap::from([
                    (Direction::North, Direction::North),
                    (Direction::South, Direction::South),
                ]),
            }),
            'J' => Some(Self {
                directions: HashMap::from([
                    (Direction::East, Direction::North),
                    (Direction::South, Direction::West),
                ]),
            }),
            '7' => Some(Self {
                directions: HashMap::from([
                    (Direction::East, Direction::South),
                    (Direction::North, Direction::West),
                ]),
            }),
            'L' => Some(Self {
                directions: HashMap::from([
                    (Direction::South, Direction::East),
                    (Direction::West, Direction::North),
                ]),
            }),
            'F' => Some(Self {
                directions: HashMap::from([
                    (Direction::North, Direction::East),
                    (Direction::West, Direction::South),
                ]),
            }),
            _ => None,
        }
    }

    fn next_direction(&self, direction: &Direction) -> Option<Direction> {
        self.directions.get(direction).copied()
    }
}

struct Map {
    map: Array2D<char>,
    start: (usize, usize),
}

impl Map {
    fn next_tile(
        &self,
        current: &(usize, usize),
        direction: &Direction,
    ) -> Option<((usize, usize), Direction)> {
        let (row, column) = match direction {
            Direction::North => (current.0.checked_sub(1), Some(current.1)),
            Direction::East => (Some(current.0), current.1.checked_add(1)),
            Direction::South => (current.0.checked_add(1), Some(current.1)),
            Direction::West => (Some(current.0), current.1.checked_sub(1)),
        };

        if row.and(column).is_none() {
            return None;
        }

        let next_location = self.map.get(row?, column?);

        if next_location.is_none() {
            return None;
        }

        let next_direction =
            Pipe::from_char(next_location.unwrap()).and_then(|p| p.next_direction(direction));

        if next_direction.is_none() {
            return None;
        }

        return Some(((row?, column?), next_direction?));
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let rows: Vec<Vec<char>> = value
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();

        let map = Array2D::from_rows(&rows).unwrap();

        let (start, _) = map.enumerate_row_major().find(|v| *v.1 == 'S').unwrap();

        return Self { map, start };
    }
}

fn find_farthest_point(input: &str) -> Option<u64> {
    let map = Map::from(input);

    let mut paths = vec![
        (map.start, Direction::North),
        (map.start, Direction::South),
        (map.start, Direction::East),
        (map.start, Direction::West),
    ];

    let mut counter = 0;
    let mut final_count: Option<u64> = None;

    while !paths.is_empty() {
        counter += 1;

        let mut new_paths = vec![];
        let mut visited: HashMap<(usize, usize), u64> = HashMap::new();

        for (current, direction) in paths {
            visited.insert(current, counter - 1);

            match map.next_tile(&current, &direction) {
                Some(v) => {
                    if visited.contains_key(&v.0) {
                        final_count = visited.get(&v.0).copied();
                    } else {
                        visited.insert(v.0, counter);
                        new_paths.push(v);
                    }
                }
                _ => {}
            };
        }

        if final_count.is_some() {
            break;
        }

        paths = new_paths;
    }

    return final_count;
}

fn main() {
    aoc::run(10, |input| {
        let part_1 = find_farthest_point(input);

        return (part_1, None);
    })
}

#[cfg(test)]
mod tests {
    use crate::find_farthest_point;

    #[test]
    fn test_find_farthest_point() {
        let input = "\
            -L|F7\n\
            7S-7|\n\
            L|7||\n\
            -L-J|\n\
            L|-JF\
        ";

        let result = find_farthest_point(input);

        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_find_farthest_point_complex() {
        let input = "\
            ..F7.\n\
            .FJ|.\n\
            SJ.L7\n\
            |F--J\n\
            LJ...\
        ";

        let result = find_farthest_point(input);

        assert_eq!(result, Some(8));
    }
}
