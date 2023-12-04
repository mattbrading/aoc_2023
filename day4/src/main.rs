use std::{fs, time::Instant, collections::HashMap};

struct CardScore {
    id: u32,
    matches: u32,
    power_score: u32,
}
fn score_card(card_str: &str) -> CardScore {
    let (card_description, card_contents) = card_str.split_once(": ").expect("Couldn't parse card");

    let (_, id_str) = card_description
        .split_once(" ")
        .expect("Couldn't parse description");
    let id_str = id_str.trim();
    let id = u32::from_str_radix(id_str, 10).expect("Invalid Card Id");

    let (winning_numbers, actual_numbers) = card_contents
        .split_once(" | ")
        .expect("Couldn't split card sections");

    let winning_numbers: Vec<u32> = winning_numbers
        .split_whitespace()
        .map(|i| u32::from_str_radix(i, 10).expect("Couldn't parse item"))
        .collect();

    let matches: u32 = actual_numbers
        .split_whitespace()
        .map(|i| u32::from_str_radix(i, 10).expect("Couldn't parse item"))
        .filter(|i| winning_numbers.contains(i))
        .count()
        .try_into()
        .expect("Invalid Count Size");

    let power_score = match matches {
        0 => 0,
        _ => 2u32.checked_pow(matches - 1).expect("Score overflowed"),
    };

    return CardScore {
        id,
        matches,
        power_score,
    };
}

struct Score {
    part_1: u32,
    part_2: u32,
}
fn score_set(input: &str) -> Score {
    let part_1 = input
        .lines()
        .map(|card| score_card(card).power_score)
        .fold(0, |acc, x| acc + x);

    let part_2 = input
        .lines()
        .map(|card| score_card(card))
        .fold(HashMap::new(), |mut acc: HashMap<u32, u32>, card| {
            let copies: u32 = acc.get(&card.id)
                .unwrap_or(&0).to_owned() + 1;

            acc.insert(card.id, copies);

            if card.matches > 0 {
                for id in  card.id+1..card.id+card.matches+1 {
                    acc.entry(id)
                        .and_modify(|x| *x += copies)
                        .or_insert(copies);
                };
            };
            
            return acc
        })
        .values()
        .fold(0, |acc, x| acc + x);

    return Score { part_1, part_2 };
}

fn main() {
    println!("Advent of Code, Day 4!");

    let input =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");

    let timer = Instant::now();

    let result = score_set(input.as_str());

    let time_taken = timer.elapsed();

    println!("Day 3 Result, Part 1: {}", result.part_1);
    println!("Day 3 Result, Part 2: {}", result.part_2);
    println!("Time Taken: {:?}", time_taken);
}

#[cfg(test)]
mod tests {
    use crate::{score_card, score_set};

    #[test]
    fn test_score_card() {
        let card_str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = score_card(card_str);

        assert_eq!(result.id, 1);
        assert_eq!(result.matches, 4);
        assert_eq!(result.power_score, 8);
    }

    #[test]
    fn test_score_set() {
        let input = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\
        ";

        let result = score_set(input);

        assert_eq!(result.part_1, 13);
        assert_eq!(result.part_2, 30);
    }
}
