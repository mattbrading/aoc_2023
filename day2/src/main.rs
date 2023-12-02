use std::{fs, time::Instant};

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    max_blue: u32,
    max_red: u32,
    max_green: u32,
}

fn parse_game(input: &str) -> Game {
    let result: Vec<&str> = input.split(": ").collect();

    let game_id: Vec<&str> = result[0].split(" ").collect();
    let game_id = game_id[1];
    let game_id = u32::from_str_radix(game_id, 10).unwrap();

    let mut game = Game {
        id: game_id,
        max_blue: 0,
        max_green: 0,
        max_red: 0,
    };

    for round_str in result[1].split("; ") {
        for colour_str in round_str.split(", ") {
            let elements: Vec<&str> = colour_str.split(" ").collect();

            let amount = u32::from_str_radix(elements[0], 10).unwrap();

            match elements[1] {
                "red" => {
                    if amount > game.max_red {
                        game.max_red = amount;
                    };
                }
                "green" => {
                    if amount > game.max_green {
                        game.max_green = amount;
                    };
                }
                "blue" => {
                    if amount > game.max_blue {
                        game.max_blue = amount;
                    };
                }
                &_ => {}
            };
        }
    }

    return game;
}

fn sum_matching_games(input: &str) -> u32 {
    let mut sum: u32 = 0;

    for game_str in input.split("\n") {
        let game = parse_game(game_str);

        if game.max_red <= 12 && game.max_green <= 13 && game.max_blue <= 14 {
            sum += game.id;
        }
    }

    return sum;
}

fn main() {
    println!("Advent of Code, Day 2!");

    let input =
        fs::read_to_string("./src/day2.txt").expect("Should have been able to read the file");

    let timer = Instant::now();

    let result = sum_matching_games(input.as_str());

    let time_taken = timer.elapsed();

    println!("Day 2 Result: {}", result);
    println!("Time Taken: {}ms", time_taken.as_millis());
}

#[cfg(test)]
mod tests {

    use crate::{parse_game, sum_matching_games, Game};

    #[test]
    fn parse_game_test() {
        let examples = [(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            Game {
                id: 1,
                max_blue: 6,
                max_green: 2,
                max_red: 4,
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

        let result = sum_matching_games(games);

        assert_eq!(result, 8);
    }
}
