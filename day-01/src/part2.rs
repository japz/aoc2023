use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut sum = 0;
    for line in _input.lines() {
        sum += process_line(line)
    }

    return Ok(sum.to_string());
}

#[tracing::instrument]
fn process_line(line: &str) -> u32 {
    let mut i = 0;
    // Eat up the line character by character and check if it starts with a number. If so, return the number.
    let line_iter = std::iter::from_fn(move || {
        let new_line = &line[i..];
        let result = if new_line.starts_with("one") {
            Some('1')
        } else if new_line.starts_with("two") {
            Some('2')
        } else if new_line.starts_with("three") {
            Some('3')
        } else if new_line.starts_with("four") {
            Some('4')
        } else if new_line.starts_with("five") {
            Some('5')
        } else if new_line.starts_with("six") {
            Some('6')
        } else if new_line.starts_with("seven") {
            Some('7')
        } else if new_line.starts_with("eight") {
            Some('8')
        } else if new_line.starts_with("nine") {
            Some('9')
        } else {
            let result = new_line.chars().next();
            result
        };
        i += 1;
        result
    });
    let mut digits: Vec<u32> = vec![];
    for c in line_iter {
        // check if c is a digit
        if c.is_digit(10) {
            digits.push(c.to_digit(10).unwrap());
        }
    }
    let number = format!("{}{}", digits[0], digits[digits.len() - 1]);
    return number.parse::<u32>().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn line_test(#[case] line: &str, #[case] expected: u32) -> miette::Result<()> {
        let result = process_line(line);
        assert_eq!(result, expected);
        Ok(())
    }
}