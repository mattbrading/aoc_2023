use aoc;
use core::panic;
use std::ops::Mul;

fn main() {
    aoc::run(18, |input| {
        let dig = Dig::from(input);
        let part_1 = dig.get_lava_capacity();

        return (Some(part_1), None);
    })
}

struct Dig {
    instructions: Vec<((isize, isize), u64, String)>,
}

impl From<&str> for Dig {
    fn from(value: &str) -> Self {
        let instructions = value
            .lines()
            .map(|l| {
                let mut parts = l.split_whitespace();
                let dir = match parts.next() {
                    Some("U") => (-1, 0),
                    Some("D") => (1, 0),
                    Some("L") => (0, -1),
                    Some("R") => (0, 1),
                    _ => panic!("Invalid Direction")
                };
                let dist = parts.next().unwrap().parse().unwrap();
                let col = parts.next().unwrap();
                let col = col.get(2..col.len() - 1).unwrap().to_string();

                (dir, dist, col)
            })
            .collect();

        return Self { instructions };
    }
}

impl Dig {
    fn get_lava_capacity(&self) -> u64 {
        let (_, x_sum, y_sum, perim) = self.instructions.iter().fold(
            ((0,0), 0, 0, 0),
            |(last, x_sum, y_sum, perim), (dir, dist, _)| {

                let next_row = last.0 + (dir.0.mul(*dist as isize));
                let next_col = last.1 + (dir.1.mul(*dist as isize));

                
                // Shoelace theorem
                let x_sum = x_sum + (last.0 * next_col);
                let y_sum = y_sum + (last.1 * next_row);
                let perim = perim + dist;

                return ((next_row, next_col), x_sum, y_sum, perim);
            },
        );

        let area = ((x_sum.abs_diff(y_sum))/2 )as u64;
        
        let area = area + (perim /2) + 1;

        return area as u64;
    }
}

#[cfg(test)]
mod tests {
    use crate::Dig;

    const INPUT: &str = "\
        R 6 (#70c710)\n\
        D 5 (#0dc571)\n\
        L 2 (#5713f0)\n\
        D 2 (#d2c081)\n\
        R 2 (#59c680)\n\
        D 2 (#411b91)\n\
        L 5 (#8ceee2)\n\
        U 2 (#caa173)\n\
        L 1 (#1b58a2)\n\
        U 2 (#caa171)\n\
        R 2 (#7807d2)\n\
        U 3 (#a77fa3)\n\
        L 2 (#015232)\n\
        U 2 (#7a21e3)\
    ";

    #[test]
    fn test_calculate_lava_capacity() {
        let result = Dig::from(INPUT).get_lava_capacity();

        assert_eq!(result, 62)
    }
}
