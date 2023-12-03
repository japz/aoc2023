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
    fn max_cubes_per_color(&self) -> HashMap<String, u32> {
        let mut result = HashMap::new();
        for round in &self.rounds {
            for cube in &round.cubes {
                let color = cube.color.clone();
                let quantity = cube.quantity;
                let current_value = result.get(&color);
                match current_value {
                    Some(value) => {
                        if quantity > *value {
                            result.insert(color, quantity);
                        }
                    },
                    None => {
                        result.insert(color, quantity);
                    }
                }
            }
        }
        result
    }
    fn set_power(&self) -> u32 {
        self.max_cubes_per_color().values().product()
    }
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let (_, games) = parse_games(_input).expect("Failed to parse games");
    let result = games.iter().map(|game| {
        game.set_power()
    });
    Ok(result.sum::<u32>().to_string())
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
    use rstest::rstest;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("2286", process(input)?);
        Ok(())
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630)]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn test_game_set_power(#[case] input: &str, #[case] expected: u32) {
        let (_, games) = parse_games(input).expect("Failed to parse games");
        assert_eq!(games[0].set_power(), expected);
    }
}