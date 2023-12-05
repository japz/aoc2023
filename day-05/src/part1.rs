use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use crate::custom_error::AocError;

#[derive(Debug)]
struct Range {
    min: u64,
    max: u64,
    destination: u64,
}
impl Range {
    fn new(destination: u64, start: u64, length: u64) -> Self {
        Self {
            min: start,
            max: start + length - 1,
            destination,
        }
    }
    fn resolve(&self, input: u64) -> Option<u64> {
        if (input >= self.min && input <= self.max) {
            Some(input - self.min + self.destination)
        } else {
            None
        }
    }
}

// A map contains multiple ranges
#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}
impl Map {
    fn new(map: Vec<(u64, u64, u64)>) -> Self {
        let ranges = map
            .iter().map(
            |&row| Range::new(row.0, row.1,row.2))
            .collect();
        Self {
            ranges
        }
    }
    fn resolve(&self, input: u64) -> u64 {
        for range in &self.ranges {
            let output = range.resolve(input);
            match output {
                Some(output) => return output,
                None => continue,
            }
        }
        input
    }
}
#[derive(Debug)]
// A sequence of maps
struct Maps {
    maps: Vec<Map>,
}
impl Maps {
    // We walk through our maps and resolve the input through each map
    fn resolve(&self, seed: u64) -> u64 {
        let mut current = seed;
        for map in &self.maps {
            current = map.resolve(current);
        }
        current
    }
}
fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, map) = separated_list1(
        tag("\n"),
        separated_list1(
            tag(" "),
            map_res(digit1, |digit_str: &str| digit_str.parse::<u64>()),
        ),
    )(input)?;
    // Only consider the first 3 columns, if we get extra data we discard it
    let map: Vec<(u64, u64, u64)> = map
        .iter()
        .map(|row| (row[0], row[1], row[2]))
        .collect();
    Ok((input, Map::new(map)))
}

// Lazy parser, don't @ me
fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Maps)> {
    let (input, seeds) = preceded(
        tag("seeds: "),
        separated_list1(
            tag(" "),
            map_res(digit1, |digit_str: &str| digit_str.parse::<u64>()),
        ),
    )(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, seed_to_soil_map) = preceded(
        tag("seed-to-soil map:\n"),
        parse_map
    )(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, soil_to_fertilizer_map) = preceded(
        tag("soil-to-fertilizer map:\n"),
        parse_map

    )(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, fertilizer_to_water_map) = preceded(
        tag("fertilizer-to-water map:\n"),
        parse_map
    )(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, water_to_light_map) = preceded(
        tag("water-to-light map:\n"),
        parse_map
    )(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, light_to_temperature_map) = preceded(
        tag("light-to-temperature map:\n"),
        parse_map
    )(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, temperature_to_humidity_map) = preceded(
        tag("temperature-to-humidity map:\n"),
        parse_map
    )(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, humidity_to_location_map) = preceded(
        tag("humidity-to-location map:\n"),
        parse_map
    )(input)?;
    let maps = Maps {
        maps: vec![
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        ],
    };
    Ok((input, (seeds, maps)))
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let (_, (seeds, maps)) = parse_input(_input).expect("failed to parse input");
    let outputs: Vec<u64> = seeds.iter().map(|&seed| maps.resolve(seed)).collect();
    Ok(outputs.iter().min().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!("35", process(input)?);
        Ok(())
    }
    #[test]
    fn test_range_resolve() {
        let range = Range {
            min: 0,
            max: 10,
            destination: 100,
        };
        assert_eq!(Some(100), range.resolve(0));
        assert_eq!(Some(101), range.resolve(1));
        assert_eq!(Some(110), range.resolve(10));
        assert_eq!(None, range.resolve(11));
    }
    #[test]
    fn test_map() {
        let map = Map::new(vec![
            (50, 98, 2),
            (52, 50, 48),
        ]);
        assert_eq!(10, map.resolve(10));
        assert_eq!(49, map.resolve(49));
        assert_eq!(52, map.resolve(50));
        assert_eq!(99, map.resolve(97));
        assert_eq!(50, map.resolve(98));
    }
}