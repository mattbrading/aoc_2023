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

fn main() {
    aoc::run(15, |input| {
        let part_1 = sum_hashed(input);

        return (Some(part_1), None);
    })
}

#[cfg(test)]
mod tests {
    use crate::sum_hashed;

    #[test]
    fn test_sum_hashed() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let result = sum_hashed(input);

        assert_eq!(result, 1320);
    }
}
