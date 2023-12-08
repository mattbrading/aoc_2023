use std::collections::HashMap;
enum Instruction {
    L,
    R
}

type Location = [char; 3];

fn find_step_count(input: &str) -> u32 {

    let (instructions, map) = input.split_once("\n\n").unwrap();

    let instructions = instructions
        .chars()
        .map(|c| match c {
            'L' => Instruction::L,
            'R' => Instruction::R,
        })
        .collect::<Vec<_>>()
        .iter();

    let map = map
        .lines()
        .fold(HashMap::new(), |acc, l| {
            let pieces: [Location; 3]= l.replace(&['_','(',')',','], "")
                .splitn(n, " ")
                .map(|l| Location::from([
                    l.chars().next().unwrap(),
                    l.chars().next().unwrap(),
                    l.chars().next().unwrap(),
                ]))
                .collect();

            acc.insert(pieces., v)
        });

    println!("{:?}", map);

    return 2;

}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use crate::find_step_count;


    #[test]
    fn test_find_step_count() {

        let input = "\
            LLR\n\n\
            AAA = (BBB, BBB)\n\
            BBB = (AAA, ZZZ)\n\
            ZZZ = (ZZZ, ZZZ)\
        ";

        let result = find_step_count(input);

        assert_eq!(result, 6);
    }
}