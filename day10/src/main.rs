use std::collections::{HashMap, HashSet};

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

    fn find_path(&self) -> HashSet<(usize, usize)> {
        let (mut position, mut direction) = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .iter()
        .find_map(|d| self.next_tile(&self.start, d))
        .expect("No first move found!");

        let mut path = HashSet::from([self.start, position]);

        loop {
            match self.next_tile(&position, &direction) {
                Some(v) => {
                    path.insert(v.0);
                    position = v.0;
                    direction = v.1;
                }
                _ => {
                    break;
                }
            };
        }

        return path;
    }

    fn find_farthest_point(&self) -> Option<u64> {
        let path = self.find_path();
        return (path.len() as u64).checked_div(2);
    }

    fn find_path_area(&self) -> u64 {
        let path = self.find_path();

        let counter = self
            .map
            .rows_iter()
            .enumerate()
            .map(|(row_index, row)| {
                let (counter, _, _) = row.enumerate().fold(
                    (0, false, None),
                    |(counter, inside, last_boundry), (col_index, char)| {
                        let position = (row_index, col_index);
                        let is_pipe = path.contains(&position);

                        let is_boundry = match (char, last_boundry) {
                            ('|', _) => true,
                            ('S', _) => true,
                            ('F', _) => true,
                            ('L', _) => true,
                            ('7', Some('L')) => false,
                            ('J', Some('F')) => false,
                            ('7', Some('S')) => false,
                            ('J', Some('S')) => false,
                            ('7', _) => true,
                            ('J', _) => true,
                            _ => false,
                        };

                        match (is_pipe, is_boundry, inside) {
                            (true, true, _) => (counter, !inside, Some(*char)),
                            (true, false, _) => (counter, inside, last_boundry),
                            (false, _, true) => (counter + 1, inside, last_boundry),
                            _ => (counter, inside, last_boundry),
                        }
                    },
                );
                return counter;
            })
            .sum();

        return counter;
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

fn main() {
    aoc::run(10, |input| {
        let map = Map::from(input);
        let part_1 = map.find_farthest_point();
        let part_2 = map.find_path_area();
        return (part_1, Some(part_2));
    })
}

#[cfg(test)]
mod tests {
    use crate::Map;

    #[test]
    fn test_find_farthest_point() {
        let input = "\
            -L|F7\n\
            7S-7|\n\
            L|7||\n\
            -L-J|\n\
            L|-JF\
        ";

        let result = Map::from(input).find_farthest_point();

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

        let result = Map::from(input).find_farthest_point();

        assert_eq!(result, Some(8));
    }

    #[test]
    fn find_path_area() {
        let input = "\
            ...........\n\
            .S-------7.\n\
            .|F-----7|.\n\
            .||.....||.\n\
            .||.....||.\n\
            .|L-7.F-J|.\n\
            .|..|.|..|.\n\
            .L--J.L--J.\n\
            ...........\
        ";

        let result = Map::from(input).find_path_area();

        assert_eq!(result, 4);
    }
    #[test]
    fn find_path_area_2() {
        let input = "\
            .F----7F7F7F7F-7....\n\
            .|F--7||||||||FJ....\n\
            .||.FJ||||||||L7....\n\
            FJL7L7LJLJ||LJ.L-7..\n\
            L--J.L7...LJS7F-7L7.\n\
            ....F-J..F7FJ|L7L7L7\n\
            ....L7.F7||L7|.L7L7|\n\
            .....|FJLJ|FJ|F7|.LJ\n\
            ....FJL-7.||.||||...\n\
            ....L---J.LJ.LJLJ...\
        ";

        let result = Map::from(input).find_path_area();

        assert_eq!(result, 8);
    }

    #[test]
    fn find_path_area_complex() {
        let input = "\
            FF7FSF7F7F7F7F7F---7\n\
            L|LJ||||||||||||F--J\n\
            FL-7LJLJ||||||LJL-77\n\
            F--JF--7||LJLJ7F7FJ-\n\
            L---JF-JLJ.||-FJLJJ7\n\
            |F|F-JF---7F7-L7L|7|\n\
            |FFJF7L7F-JF7|JL---7\n\
            7-L-JL7||F7|L7F-7F7|\n\
            L.L7LFJ|||||FJL7||LJ\n\
            L7JLJL-JLJLJL--JLJ.L\
        ";

        let result = Map::from(input).find_path_area();

        assert_eq!(result, 10);
    }
}
