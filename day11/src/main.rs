use aoc;
use array2d::Array2D;

struct Universe {
    map: Array2D<char>,
}

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        let rows: Vec<Vec<char>> = value
            .lines()
            .flat_map(|r| {
                let row: Vec<char> = r.chars().map(|c| c.clone()).collect();
                match row.iter().any(|s| *s == '#') {
                    true => vec![row],
                    false => vec![row.clone(), row],
                }
            })
            .collect();

        let cols: Vec<Vec<char>> = Array2D::from_rows(&rows)
            .unwrap()
            .columns_iter()
            .flat_map(|c| {
                let col: Vec<char> = c.map(|z| z.clone()).collect();
                match col.iter().any(|s| *s == '#') {
                    true => vec![col],
                    false => vec![col.clone(), col],
                }
            })
            .collect();

        let map = Array2D::from_columns(&cols).unwrap();

        return Self { map };
    }
}

impl Universe {
    fn sum_distances(&self) -> u64 {
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
                    .map(|locb| (locb.0.abs_diff(loc.0) + locb.1.abs_diff(loc.1)) as u64)
                    .sum::<u64>()
            })
            .sum()
    }
}

fn main() {
    aoc::run(11, |input| {
        let part_1 = Universe::from(input).sum_distances();

        return (Some(part_1), None);
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

        let result = Universe::from(input).sum_distances();

        assert_eq!(result, 374);
    }
}
