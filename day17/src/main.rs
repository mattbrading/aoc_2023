use aoc;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    usize,
};

fn main() {
    aoc::run(17, |input| {
        let map = CityMap::from(input);

        let part_1 = map.find_best_path();

        return (part_1, None);
    })
}

struct CityMap {
    grid: Vec<Vec<u32>>,
}

impl From<&str> for CityMap {
    fn from(value: &str) -> Self {
        let grid = value
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        return Self { grid };
    }
}

impl CityMap {
    fn get(&self, pos: (usize, usize)) -> Option<u32> {
        self.grid.get(pos.0).and_then(|l| l.get(pos.1)).copied()
    }

    fn find_best_path(&self) -> Option<u64> {
        let row_len = self.grid.len();
        let col_len = self.grid[0].len();

        let end_position = (row_len - 1, col_len - 1);

        let start: (u32, usize, usize, isize, isize, u32) = (0, 0, 0, 0, 0, 0);

        let mut open_set = BinaryHeap::from([Reverse(start)]);
        let mut result = None;
        let mut seen = HashSet::new();

        while !open_set.is_empty() {
            let Reverse((hl, row, col, d_row, d_col, distance)) = open_set.pop().unwrap();

            if seen.contains(&(row, col, d_row, d_col, distance)) {
                continue;
            }
            seen.insert((row, col, d_row, d_col, distance));

            println!("{} {} {} {} {} {}", hl, row, col, d_row, d_col, distance);

            if (row, col) == end_position {
                println!("DONE!");
                result = Some(hl as u64);
                break;
            }

            if distance < 3 && (d_row, d_col) != (0, 0) {
                let next_row = row.checked_add_signed(d_row);
                let next_col = col.checked_add_signed(d_col);

                let next = next_row
                    .and_then(|row| next_col.and_then(|col| Some((row, col))))
                    .and_then(|pos| self.get(pos));

                match next {
                    Some(cost) => open_set.push(Reverse((
                        hl + cost,
                        next_row.unwrap(),
                        next_col.unwrap(),
                        d_row,
                        d_col,
                        distance + 1,
                    ))),
                    None => {}
                }
            }

            let next_directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]
                .iter()
                .filter(|a| a != &&(d_row, d_col) && a != &&(-d_row, -d_col));

            for (nd_row, nd_col) in next_directions.copied() {
                let next_row = row.checked_add_signed(nd_row);
                let next_col = col.checked_add_signed(nd_col);

                let next = next_row
                    .and_then(|row| next_col.and_then(|col| Some((row, col))))
                    .and_then(|pos| self.get(pos));

                match next {
                    Some(cost) => open_set.push(Reverse((
                        hl + cost,
                        next_row.unwrap(),
                        next_col.unwrap(),
                        nd_row,
                        nd_col,
                        1,
                    ))),
                    None => {}
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::CityMap;

    const INPUT: &str = "\
        2413432311323\n\
        3215453535623\n\
        3255245654254\n\
        3446585845452\n\
        4546657867536\n\
        1438598798454\n\
        4457876987766\n\
        3637877979653\n\
        4654967986887\n\
        4564679986453\n\
        1224686865563\n\
        2546548887735\n\
        4322674655533\
    ";

    #[test]
    fn test_find_best_path() {
        let result = CityMap::from(INPUT).find_best_path();

        assert_eq!(result, Some(102));
    }
}
