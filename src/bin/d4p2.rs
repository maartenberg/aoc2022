use core::ops::RangeInclusive;
use nom::{
    character::complete::{char, line_ending, u32},
    combinator::all_consuming,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::io::Read;

type Input = Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>;

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, (l, r)) = separated_pair(u32, char('-'), u32)(input)?;
    IResult::Ok((input, RangeInclusive::new(l, r)))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    many1(terminated(
        separated_pair(parse_range, char(','), parse_range),
        line_ending,
    ))(input)
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("Read fail");

    let input: Input = all_consuming(parse_input)(&buffer).expect("Parse fail").1;

    #[cfg(debug_assertions)]
    dbg!(&input);

    let score = input
        .iter()
        .filter(|(lr, rr)| {
            (lr.start() >= rr.start() && lr.start() <= rr.end())
                || (rr.start() >= lr.start() && rr.start() <= lr.end())
        })
        .count();

    println!("{}", score);
}
