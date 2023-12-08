use aoc;
use std::collections::{BTreeMap, HashMap};

fn parse_hand(cards: &str, jokers: bool) -> u8 {
    let mut counts = cards
        .chars()
        .fold(HashMap::new(), |mut acc: HashMap<char, u8>, char| {
            acc.entry(char).and_modify(|v| *v += 1).or_insert(1);
            return acc;
        });

    let joker_count: u8 = match jokers {
        true => counts.remove(&'J').unwrap_or(0),
        false => 0,
    };

    if joker_count == 5 {
        return 6;
    };

    let mut counts = counts.values().collect::<Vec<&u8>>();

    counts.sort();

    let (&last, elements) = counts.split_last().expect("Couldn't split!");

    let last = last + joker_count;

    let mut elements = elements.to_owned();
    elements.push(&last);

    elements.iter().fold(0_u8, |acc, count| {
        match (acc, count) {
            (0, 2) => 1, // One pair
            (1, 2) => 2, // Two pairs
            (0, 3) => 3, // Three of a Kind
            (3, 2) => 4, // Full House
            (1, 3) => 4, // Full House
            (_, 4) => 5, // Four of a Kind
            (_, 5) => 6, // Five of a Kind
            _ => acc,
        }
    })
}

fn card_value(card: char, jokers: bool) -> u32 {
    match (card, jokers) {
        ('T', _) => 10,
        ('J', true) => 1,
        ('J', false) => 11,
        ('Q', _) => 12,
        ('K', _) => 13,
        ('A', _) => 14,
        _ => card.to_digit(10).unwrap(),
    }
}

fn sum_winnings(hands: &str, jokers: bool) -> u32 {
    hands
        .lines()
        .map(|line| {
            let parts = line.split_once(" ").unwrap();
            return (parts.0, parts.1.parse::<u32>().unwrap());
        })
        .fold(
            BTreeMap::new(),
            |mut acc: BTreeMap<u64, (&str, u32)>, (hand, bid)| {
                let type_score = parse_hand(hand, jokers) as u64;

                let hand_value =
                    hand.chars()
                        .rev()
                        .enumerate()
                        .fold(0, |score: u64, (index, card)| {
                            score + card_value(card, jokers) as u64 * 100_u64.pow(index as u32)
                        });

                let score = type_score * 100_u64.pow(6) + hand_value;

                acc.insert(score, (hand, bid));
                return acc;
            },
        )
        .values()
        .cloned()
        .enumerate()
        .fold(0, |acc, (index, (_, bid))| acc + bid * (index as u32 + 1))
}

fn main() {
    aoc::run(7, |input| {
        (
            Some(sum_winnings(input, false) as u64),
            Some(sum_winnings(input, true) as u64),
        )
    });
}

#[cfg(test)]
mod tests {
    use crate::{card_value, parse_hand, sum_winnings};

    #[test]
    fn test_parse_hand() {
        let examples = [
            ("23456", 0),
            ("A23A4", 1),
            ("23432", 2),
            ("TTT98", 3),
            ("23332", 4),
            ("AA8AA", 5),
            ("AAAAA", 6),
        ];

        for (cards, expected) in examples {
            let result = parse_hand(cards, false);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_parse_hand_jokers() {
        let examples = [("QJJQ2", 5), ("JKKK2", 5)];

        for (cards, expected) in examples {
            let result = parse_hand(cards, true);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_card_value() {
        let examples = [
            ('2', 2),
            ('3', 3),
            ('9', 9),
            ('T', 10),
            ('J', 11),
            ('Q', 12),
            ('K', 13),
            ('A', 14),
        ];

        for (card, expected) in examples {
            let result = card_value(card, false);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_card_value_jokers() {
        let examples = [
            ('2', 2),
            ('3', 3),
            ('9', 9),
            ('T', 10),
            ('J', 1),
            ('Q', 12),
            ('K', 13),
            ('A', 14),
        ];

        for (card, expected) in examples {
            let result = card_value(card, true);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_sum_winnings() {
        let input = "\
            32T3K 765\n\
            T55J5 684\n\
            KK677 28\n\
            KTJJT 220\n\
            QQQJA 483\
        ";

        let result = sum_winnings(input, false);

        assert_eq!(result, 6440);
    }

    #[test]
    fn test_sum_winnings_jokers() {
        let input = "\
            32T3K 765\n\
            T55J5 684\n\
            KK677 28\n\
            KTJJT 220\n\
            QQQJA 483\
        ";

        let result = sum_winnings(input, true);

        assert_eq!(result, 5905);
    }
}
