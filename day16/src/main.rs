use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

use aoc;

fn main() {
    aoc::run(16, |input| {
        let part_1 = Contraption::from_str(input)
            .expect("Failed to Parse")
            .count_activated_tiles(Photon {
                position: (0, 0),
                direction: crate::Direction::Right,
            });
        return (Some(part_1), None);
    })
}

#[derive(Debug, PartialEq, Eq)]
struct Contraption {
    rows: Vec<BTreeMap<usize, char>>,
    cols: Vec<BTreeMap<usize, char>>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseContraptionError;

impl FromStr for Contraption {
    type Err = ParseContraptionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row_count = s.lines().count();
        let col_count = s.find("\n").ok_or(ParseContraptionError)?;

        let mut rows = vec![BTreeMap::new(); row_count];
        let mut cols = vec![BTreeMap::new(); col_count];

        s.lines().enumerate().for_each(|(row_idx, row)| {
            row.chars().enumerate().for_each(|(col_idx, char)| {
                if char == '.' {
                    return;
                }

                if char != '-' {
                    rows[row_idx].insert(col_idx, char);
                }

                if char != '|' {
                    cols[col_idx].insert(row_idx, char);
                }
            })
        });

        return Ok(Self { rows, cols });
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Photon {
    position: (usize, usize),
    direction: Direction,
}

impl Contraption {
    fn count_activated_tiles(&self, init: Photon) -> u64 {
        let mut visited_tiles = HashSet::from([init.position]);
        let mut photon_history = HashSet::new();

        let mut photons = match self.rows.get(0).and_then(|c| c.get(&0)) {
            Some(tile) => self.tile_beam_result(tile, init.position, init.direction),
            None => vec![init],
        };

        while !photons.is_empty() {
            let new_photons = photons.iter().copied().flat_map(|photon| {
                let (row, col) = photon.position;

                if photon_history.contains(&photon) {
                    return vec![];
                }

                let (new_row, new_col, tile) = match photon.direction {
                    Direction::Right => {
                        let next = self.rows.get(row).unwrap();
                        next.range(col + 1..)
                            .next()
                            .map_or((row, self.cols.len() - 1, '?'), |(col_idx, char)| {
                                (row, *col_idx, *char)
                            })
                    }
                    Direction::Left => {
                        let next = self.rows.get(row).unwrap();
                        next.range(..col)
                            .rev()
                            .next()
                            .map_or((row, 0, '?'), |(col_idx, char)| (row, *col_idx, *char))
                    }
                    Direction::Up => {
                        let next = self.cols.get(col).unwrap();
                        next.range(..row)
                            .rev()
                            .next()
                            .map_or((0, col, '?'), |(row_idx, char)| (*row_idx, col, *char))
                    }
                    Direction::Down => {
                        let next = self.cols.get(col).unwrap();
                        next.range(row + 1..)
                            .next()
                            .map_or((self.rows.len() - 1, col, '?'), |(row_idx, char)| {
                                (*row_idx, col, *char)
                            })
                    }
                };

                match photon.direction {
                    Direction::Up | Direction::Down => visited_tiles
                        .extend((row.min(new_row)..=row.max(new_row)).map(|i| (i, col))),
                    Direction::Right | Direction::Left => visited_tiles
                        .extend((col.min(new_col)..=col.max(new_col)).map(|i| (row, i))),
                }

                let position = (new_row, new_col);

                let new = self.tile_beam_result(&tile, position, photon.direction);

                photon_history.insert(photon);

                return new;
            });

            photons = new_photons.collect();
        }

        for r_idx in 0..self.rows.len() {
            let col_str = (0..self.cols.len())
                .map(|c_idx| {
                    if visited_tiles.contains(&(r_idx, c_idx)) {
                        "#"
                    } else {
                        "."
                    }
                })
                .collect::<Vec<&str>>()
                .join("");
            println!("{}", col_str)
        }

        return visited_tiles.len() as u64;
    }

    fn tile_beam_result(
        &self,
        tile: &char,
        position: (usize, usize),
        direction: Direction,
    ) -> Vec<Photon> {
        match (tile, direction) {
            ('/', Direction::Up) => vec![Photon {
                position,
                direction: Direction::Right,
            }],
            ('/', Direction::Down) => vec![Photon {
                position,
                direction: Direction::Left,
            }],
            ('/', Direction::Right) => vec![Photon {
                position,
                direction: Direction::Up,
            }],
            ('/', Direction::Left) => vec![Photon {
                position,
                direction: Direction::Down,
            }],
            ('\\', Direction::Up) => vec![Photon {
                position,
                direction: Direction::Left,
            }],
            ('\\', Direction::Down) => vec![Photon {
                position,
                direction: Direction::Right,
            }],
            ('\\', Direction::Right) => vec![Photon {
                position,
                direction: Direction::Down,
            }],
            ('\\', Direction::Left) => vec![Photon {
                position,
                direction: Direction::Up,
            }],
            ('|', _) => vec![
                Photon {
                    position,
                    direction: Direction::Up,
                },
                Photon {
                    position,
                    direction: Direction::Down,
                },
            ],
            ('-', _) => vec![
                Photon {
                    position,
                    direction: Direction::Left,
                },
                Photon {
                    position,
                    direction: Direction::Right,
                },
            ],
            _ => {
                vec![]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{Contraption, Photon};

    const INPUT: &str = "\
        .|...\\....\n\
        |.-.\\.....\n\
        .....|-...\n\
        ........|.\n\
        ..........\n\
        .........\\\n\
        ..../.\\\\..\n\
        .-.-/..|..\n\
        .|....-|.\\\n\
        ..//.|....\
    ";

    #[test]
    fn test_count_activated_tiles() {
        let result = Contraption::from_str(INPUT)
            .expect("Failed to Parse")
            .count_activated_tiles(Photon {
                position: (0, 0),
                direction: crate::Direction::Right,
            });

        assert_eq!(result, 46);
    }
}
