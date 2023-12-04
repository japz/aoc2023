use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, multispace0, multispace1};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair};
use crate::custom_error::AocError;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
struct Card {
    number: u32,
    winning: Vec<u32>,
    mine: Vec<u32>,
}
impl Card {
    fn my_winning_numbers(&self) -> Vec<&u32> {
        let numbers = self.mine.iter().filter(|&i| self.winning.contains(i) ).collect();
        numbers
    }
    fn score(&self) -> u32 {
        let num_matches = self.my_winning_numbers().len() as u32;
        let score = match num_matches.checked_sub(1) {
            Some(n) => 2u32.pow(n),
            None => 0,
        };
        score
    }
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {

    let (_, cards) = parse_cards(_input).expect("Failed to parse cards");
    let score = cards.iter().map(|card| card.score()).sum::<u32>();
    Ok(score.to_string())
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, digits) = separated_list1(multispace1, digit1)(input)?;
    Ok((input, digits.iter().map(|d| d.parse::<u32>().unwrap()).collect()))
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, (card_number, (winning, mine))) = separated_pair(
        preceded(tag("Card"), preceded(multispace1, digit1)), delimited(multispace0, tag(":"), multispace0),
        separated_pair(parse_number_list, delimited(multispace0, tag("|"), multispace0), parse_number_list)
    )(input)?;
    let number = card_number.parse::<u32>().unwrap();
    Ok((input, Card { number, winning, mine }))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(line_ending, parse_card)(input)?;
    Ok((input, cards))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input)?);
        Ok(())
    }

        #[test]
    fn test_parse_cards() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = parse_cards(input).expect("Failed to parse cards");
        assert_eq!(result, ("", vec![
            Card { number: 1, winning: vec![41, 48, 83, 86, 17], mine: vec![83, 86, 6, 31, 17, 9, 48, 53] },
            Card { number: 2, winning: vec![13, 32, 20, 16, 61], mine: vec![61, 30, 68, 82, 17, 32, 24, 19] },
            Card { number: 3, winning: vec![1, 21, 53, 59, 44], mine: vec![69, 82, 63, 72, 16, 21, 14, 1] },
            Card { number: 4, winning: vec![41, 92, 73, 84, 69], mine: vec![59, 84, 76, 51, 58, 5, 54, 83] },
            Card { number: 5, winning: vec![87, 83, 26, 28, 32], mine: vec![88, 30, 70, 12, 93, 22, 82, 36] },
            Card { number: 6, winning: vec![31, 18, 13, 56, 72], mine: vec![74, 77, 10, 23, 35, 67, 36, 11] },
        ]));
        Ok(())
    }

    #[test]
    fn test_parse_number_list() -> miette::Result<()> {
        let input = "41 48 83 86 17";
        let (_, numbers) = parse_number_list(input).unwrap();
        assert_eq!(vec![41, 48, 83, 86, 17], numbers);
        Ok(())
    }
    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", Card {number: 1, winning: vec![41, 48, 83, 86, 17], mine: vec![83, 86, 6, 31, 17, 9, 48, 53]}, 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", Card {number: 2, winning: vec![13, 32, 20, 16, 61], mine: vec![61, 30, 68, 82, 17, 32, 24, 19]}, 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", Card {number: 3, winning: vec![1, 21, 53, 59, 44], mine: vec![69, 82, 63, 72, 16, 21, 14, 1]}, 2)]
    fn test_parse_card(#[case] input: &str, #[case] expected: Card, #[case] score: u32) -> miette::Result<()> {
        let (_, card) = parse_card(input).unwrap();
        assert_eq!(expected, card);
        assert_eq!(score, card.score());
        Ok(())
    }
}