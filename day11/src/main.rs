use std::collections::BTreeSet;

use aoc;
use array2d::Array2D;

struct Universe {
    map: Array2D<char>,
    empty_rows: BTreeSet<usize>,
    empty_cols: BTreeSet<usize>,
}

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        let map: Vec<Vec<char>> = value.lines().map(|r| r.chars().collect()).collect();

        let map = Array2D::from_rows(&map).unwrap();

        let empty_rows = BTreeSet::from_iter(
            map.rows_iter()
                .enumerate()
                .filter(|row| !map.row_iter(row.0).unwrap().any(|s| *s == '#'))
                .map(|r| r.0),
        );

        let empty_cols = BTreeSet::from_iter(
            map.columns_iter()
                .enumerate()
                .filter(|col| !map.column_iter(col.0).unwrap().any(|s| *s == '#'))
                .map(|c| c.0),
        );

        return Self {
            map,
            empty_cols,
            empty_rows,
        };
    }
}

impl Universe {
    fn sum_distances(&self, age: usize) -> u64 {
        let galaxies: Vec<(usize, usize)> = self
            .map
            .enumerate_row_major()
            .filter(|c| *c.1 == '#')
            .map(|c| c.0)
            .collect();

        galaxies
            .iter()
            .enumerate()
            .map(|(index, loc)| {
                galaxies[index..]
                    .iter()
                    .map(|locb| {
                        let mut row_ends = [loc.0, locb.0];
                        row_ends.sort();

                        let mut col_ends = [loc.1, locb.1];
                        col_ends.sort();

                        let row_range = row_ends[0]..row_ends[1];
                        let col_range = col_ends[0]..col_ends[1];

                        let rows_expansion = self.empty_rows.range(row_range).count() * (age - 1);
                        let cols_expansion = self.empty_cols.range(col_range).count() * (age - 1);

                        let row_diff = locb.0.abs_diff(loc.0) + rows_expansion;
                        let col_diff = locb.1.abs_diff(loc.1) + cols_expansion;

                        return (row_diff + col_diff) as u64;
                    })
                    .sum::<u64>()
            })
            .sum()
    }
}

fn main() {
    aoc::run(11, |input| {
        let part_1 = Universe::from(input).sum_distances(2);
        let part_2 = Universe::from(input).sum_distances(1000000);

        return (Some(part_1), Some(part_2));
    })
}

#[cfg(test)]
mod tests {
    use crate::Universe;

    #[test]
    fn test_sum_distances() {
        let input = "\
            ...#......\n\
            .......#..\n\
            #.........\n\
            ..........\n\
            ......#...\n\
            .#........\n\
            .........#\n\
            ..........\n\
            .......#..\n\
            #...#.....\
        ";

        let result = Universe::from(input).sum_distances(2);

        assert_eq!(result, 374);

        let result = Universe::from(input).sum_distances(100);

        assert_eq!(result, 8410);
    }
}
