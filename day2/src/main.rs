use std::{fs, time::Instant};

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    max_blue: u32,
    max_red: u32,
    max_green: u32,
    power: u32,
}

fn parse_game(input: &str) -> Game {
    let (game_str, rounds_str) = input.split_once(": ").expect("Failed to parse game");

    let (_, game_id) = game_str.split_once(" ").expect("Mising Game ID");

    let game_id = u32::from_str_radix(game_id, 10).expect("Game ID not an int");

    let mut max_blue = 0;
    let mut max_red = 0;
    let mut max_green = 0;

    for round_str in rounds_str.split("; ") {
        for colour_str in round_str.split(", ") {
            let (amount, colour) = colour_str
                .split_once(" ")
                .expect("Failed to parse colour_str");
            let amount = u32::from_str_radix(amount, 10).unwrap();

            match colour {
                "red" => {
                    if amount > max_red {
                        max_red = amount;
                    };
                }
                "green" => {
                    if amount > max_green {
                        max_green = amount;
                    };
                }
                "blue" => {
                    if amount > max_blue {
                        max_blue = amount;
                    };
                }
                &_ => {}
            };
        }
    }

    return Game {
        id: game_id,
        max_blue,
        max_red,
        max_green,
        power: max_blue * max_green * max_red,
    };
}

fn sum_games(input: &str) -> (u32, u32) {
    let mut sum_matching: u32 = 0;
    let mut sum_powers: u32 = 0;

    for game_str in input.lines() {
        let game = parse_game(game_str);

        sum_powers += game.power;

        if game.max_red <= 12 && game.max_green <= 13 && game.max_blue <= 14 {
            sum_matching += game.id;
        }
    }

    return (sum_matching, sum_powers);
}

fn main() {
    println!("Advent of Code, Day 2!");

    let input =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");

    let timer = Instant::now();

    let (sum_matching, sum_powers) = sum_games(input.as_str());

    let time_taken = timer.elapsed();

    println!("Day 2 Result, Part 1: {}", sum_matching);
    println!("Day 2 Result, Part 2: {}", sum_powers);
    println!("Time Taken: {:?}", time_taken);
}

#[cfg(test)]
mod tests {

    use crate::{parse_game, sum_games, Game};

    #[test]
    fn parse_game_test() {
        let examples = [(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            Game {
                id: 1,
                max_blue: 6,
                max_green: 2,
                max_red: 4,
                power: 48,
            },
        )];

        for (input, expected) in examples {
            let result = parse_game(input);

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_games_sum() {
        let games = "\
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\
        ";

        let result = sum_games(games);

        assert_eq!(result, (8, 2286));
    }
}
