use std::collections::HashMap;
use crate::custom_error::AocError;
use nom::{IResult};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, line_ending};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};

#[derive(Debug)]
#[derive(PartialEq)]
struct Cube {
    color: String,
    quantity: u32,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Round {
    cubes: Vec<Cube>,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Game {
    number: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn sum_cubes_per_color(&self) -> HashMap<String, u32> {
        // This was unnecessary, the cubes go back into the bag after each round
        let mut result = HashMap::new();
        for round in &self.rounds {
            for cube in &round.cubes {
                let color = cube.color.clone();
                let quantity = cube.quantity;
                let current_value = result.get(&color);
                match current_value {
                    Some(value) => {
                        result.insert(color, value + quantity);
                    },
                    None => {
                        result.insert(color, quantity);
                    }
                }
            }
        }
        result
    }
    fn is_valid_old(&self) -> bool {
        // This is not what was asked, but i'll leave it here -_-
        let mut maximum_cubes_per_color = HashMap::new();
        maximum_cubes_per_color.insert("red".to_string(), 12);
        maximum_cubes_per_color.insert("green".to_string(), 13);
        maximum_cubes_per_color.insert("blue".to_string(), 14);
        let cubes_per_color = self.sum_cubes_per_color();
        for (color, quantity) in cubes_per_color {
            let maximum_quantity = maximum_cubes_per_color.get(&color).unwrap();
            if quantity > *maximum_quantity {
                return false;
            }

        }
        true
    }
    fn is_valid(&self) -> bool {
        let mut maximum_cubes_per_color = HashMap::new();
        maximum_cubes_per_color.insert("red".to_string(), 12);
        maximum_cubes_per_color.insert("green".to_string(), 13);
        maximum_cubes_per_color.insert("blue".to_string(), 14);
        self.rounds.iter().all(|round| {
            round.cubes.iter().all(|cube| {
                let maximum_quantity = maximum_cubes_per_color.get(&cube.color).unwrap();
                cube.quantity <= *maximum_quantity
            })

        })
    }
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let games = parse_games(_input).expect("Failed to parse games");
    let mut game_id_sum = 0;
    for game in games.1 {
        if game.is_valid() {
            game_id_sum += game.number;
        }
    }
    Ok(game_id_sum.to_string())
}

// input: 3 blue
fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, (quantity, color)) = separated_pair(
        digit1, tag(" "), alpha1
    )(input)?;
    Ok((input, Cube { color: color.to_string(), quantity: quantity.parse::<u32>().unwrap() }))
}

// input: 1 red, 2 green, 6 blue
fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, cubes) = separated_list1(tag(", "), parse_cube)(input)?;
    Ok((input, Round { cubes }))
}

// input: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, (game_number, rounds)) = separated_pair(
        preceded(tag("Game "), digit1), tag(": "), separated_list1(tag("; "), parse_round)
    )(input)?;
    Ok((input, Game { number: game_number.parse::<u32>().unwrap(), rounds }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, parse_game)(input)?;
    Ok((input, games))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("8", process(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_cube() -> miette::Result<()> {
        let input = "3 blue";
        let result = parse_cube(input);
        assert_eq!(result, Ok(("", Cube { color: "blue".to_string(), quantity: 3 })));
        Ok(())
    }

    #[test]
    fn test_parse_round() -> miette::Result<()> {
        let input = "3 blue, 4 red";
        let result = parse_round(input);
        assert_eq!(result, Ok(("", Round { cubes: vec![
            Cube { color: "blue".to_string(), quantity: 3 },
            Cube { color: "red".to_string(), quantity: 4 },
        ] })));
        Ok(())
    }

    #[test]
    fn test_parse_game() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_game(input);
        assert_eq!(result, Ok(("", Game { number: 1, rounds: vec![
            Round { cubes: vec![
                Cube { color: "blue".to_string(), quantity: 3 },
                Cube { color: "red".to_string(), quantity: 4 },
            ] },
            Round { cubes: vec![
                Cube { color: "red".to_string(), quantity: 1 },
                Cube { color: "green".to_string(), quantity: 2 },
                Cube { color: "blue".to_string(), quantity: 6 },
            ] },
            Round { cubes: vec![
                Cube { color: "green".to_string(), quantity: 2 },
            ] },
        ] })));
        Ok(())
    }

    #[test]
    fn test_parse_games() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let result = parse_games(input);
        assert_eq!(result, Ok(("", vec![
            Game {
                number: 1,
                rounds: vec![
                    Round {
                        cubes: vec![
                            Cube { color: "blue".to_string(), quantity: 3 },
                            Cube { color: "red".to_string(), quantity: 4 },
                        ]
                    },
                    Round {
                        cubes: vec![
                            Cube { color: "red".to_string(), quantity: 1 },
                            Cube { color: "green".to_string(), quantity: 2 },
                            Cube { color: "blue".to_string(), quantity: 6 },
                        ]
                    },
                    Round {
                        cubes: vec![
                            Cube { color: "green".to_string(), quantity: 2 },
                        ]
                    },
                ]
            },
            Game {
                number: 2,
                rounds: vec![
                    Round {
                        cubes: vec![
                            Cube { color: "blue".to_string(), quantity: 1 },
                            Cube { color: "green".to_string(), quantity: 2 },
                        ]
                    },
                    Round {
                        cubes: vec![
                            Cube { color: "green".to_string(), quantity: 3 },
                            Cube { color: "blue".to_string(), quantity: 4 },
                            Cube { color: "red".to_string(), quantity: 1 },
                        ]
                    },
                    Round {
                        cubes: vec![
                            Cube { color: "green".to_string(), quantity: 1 },
                            Cube { color: "blue".to_string(), quantity: 1 },
                        ]
                    },
                ]
            },
        ])));
        Ok(())
    }

    #[test]
    fn test_cubes_per_color() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (_, game) = parse_game(input).expect("Failed to parse game");
        let result = game.sum_cubes_per_color();
        let mut expected = HashMap::new();
        expected.insert("blue".to_string(), 9);
        expected.insert("red".to_string(), 5);
        expected.insert("green".to_string(), 4);
        assert_eq!(result, expected);
        Ok(())
    }
}