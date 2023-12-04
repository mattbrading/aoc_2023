use std::{fs, time::Instant};

fn score_card (card_str: &str) -> u32 {
    let (_, card_contents) = card_str.split_once(": ").expect("Couldn't parse card");

    let (winning_numbers, actual_numbers) = card_contents.split_once(" | ").expect("Couldn't split card sections");

    let winning_numbers: Vec<u32> = winning_numbers
        .split_whitespace()
        .map(|i| u32::from_str_radix(i, 10).expect("Couldn't parse item"))
        .collect();

    let matches: u32 = actual_numbers
        .split_whitespace()
        .map(|i| u32::from_str_radix(i, 10).expect("Couldn't parse item"))
        .filter(|i| winning_numbers.contains(i))
        .count()
        .try_into().expect("Invalid Count Size");

    let card_score = match matches {
        0 => 0,
        _ => 2u32.checked_pow(matches - 1).expect("Score overflowed")
    };

    return card_score
}

fn score_set(input: &str) -> u32 {
    return input.lines()
        .map(|card| score_card(card))
        .fold(0, |acc, x| acc + x);
}

fn main() {
    println!("Advent of Code, Day 4!");

    let input =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");

    let timer = Instant::now();

    let result = score_set(input.as_str());

    let time_taken = timer.elapsed();

    println!("Day 3 Result, Part 1: {}", result);
    println!("Time Taken: {:?}", time_taken);
}


#[cfg(test)]
mod tests {
    use crate::{score_card, score_set};


    #[test]
    fn test_score_card () {

        let card_str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = score_card(card_str);

        assert_eq!(result, 8);
    }

    fn test_score_set () {
        let input = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\
        ";


        let result = score_set(input);

        assert_eq!(result, 13)

    }
}