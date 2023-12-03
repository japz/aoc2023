use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut sum = 0;
    for line in _input.lines() {

        let mut digits: Vec<u32> = vec![];
        for c in line.chars() {
            // check if c is a digit
            if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap() as u32);
            }
        }
        let number = format!("{}{}", digits[0], digits[digits.len() - 1]);
        sum += number.parse::<u32>().unwrap();
    }

    return Ok(sum.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}