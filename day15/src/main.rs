use core::panic;

use aoc;

fn hash(input: &str) -> u64 {
    input.chars().fold(0_u64, |acc, val| {
        let ascii = (val.to_ascii_lowercase() as u8) as u64;
        return ((acc + ascii) * 17) % 256;
    })
}

fn sum_hashed(input: &str) -> u64 {
    input.split(",").map(hash).sum()
}

fn box_focusing_power(idx: usize, lenses: &Vec<(&str, u64)>) -> u64 {
    return lenses
        .iter()
        .enumerate()
        .map(|(s, (_, f))| (idx as u64 + 1) * (s as u64 + 1) * f)
        .sum();
}

fn lens_power(input: &str) -> u64 {
    let mut boxes: Vec<Vec<(&str, u64)>> = vec![vec![]; 256];

    for x in input.split(",") {
        let split_point = x.chars().position(|c| c == '-' || c == '=').unwrap();
        let (label, operation) = x.split_at(split_point);
        let target_box = hash(label) as usize;
        let mut chars = operation.chars();

        match chars.next() {
            Some('-') => {
                boxes[target_box].retain(|(l, _)| *l != label);
            }
            Some('=') => {
                let current_index = boxes[target_box].iter().position(|(l, _)| *l == label);

                boxes[target_box].push((label, chars.next().unwrap().to_digit(10).unwrap() as u64));

                _ = current_index.and_then(|idx| Some(boxes[target_box].swap_remove(idx)))
            }
            _ => panic!("Invalid Operation"),
        }
    }

    return boxes
        .iter()
        .enumerate()
        .map(|(idx, lenses)| box_focusing_power(idx, lenses))
        .sum();
}

fn main() {
    aoc::run(15, |input| {
        let part_1 = sum_hashed(input);
        let part_2 = lens_power(input);

        return (Some(part_1), Some(part_2));
    })
}

#[cfg(test)]
mod tests {
    use crate::{lens_power, sum_hashed};

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_sum_hashed() {
        let result = sum_hashed(INPUT);
        assert_eq!(result, 1320);
    }

    #[test]
    fn test_lens_power() {
        let result = lens_power(INPUT);
        assert_eq!(result, 145);
    }
}
