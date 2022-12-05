use nom::character::complete::i32 as parse_i32;
use nom::sequence::separated_pair;
use nom::{character::complete::char, IResult};

fn number_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(parse_i32, char('-'), parse_i32)(input)
}

type NumPair = (i32, i32);
fn parse_line(input: &str) -> IResult<&str, (NumPair, NumPair)> {
    separated_pair(number_pair, char(','), number_pair)(input)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = String::from_utf8(std::fs::read("inputs/04.txt")?)?;

    let mut result = 0;
    for pair in input.lines() {
        let parsed = parse_line(pair).unwrap().1;
        let (range1, range2) = parsed;

        if range1.0 >= range2.0 && range1.1 <= range2.1
            || range2.0 >= range1.0 && range2.1 <= range1.1
        {
            result += 1
        }
    }

    println!("result: {result}");
    Ok(())
}
