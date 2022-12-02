use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::{all_consuming, value},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::io::Read;

#[derive(Clone, Debug)]
enum Move {
    Paper,
    Rock,
    Scissors,
}
use Move::*;

type Input = Vec<Match>;
type Match = (Move, Move);

fn parse_input(input: &str) -> IResult<&str, Input> {
    many1(terminated(
        separated_pair(parse_oppmove, char(' '), parse_ownmove),
        line_ending,
    ))(&input)
}

fn parse_oppmove(input: &str) -> IResult<&str, Move> {
    alt((
        value(Rock, char('A')),
        value(Paper, char('B')),
        value(Scissors, char('C')),
    ))(input)
}

fn parse_ownmove(input: &str) -> IResult<&str, Move> {
    alt((
        value(Rock, char('X')),
        value(Paper, char('Y')),
        value(Scissors, char('Z')),
    ))(input)
}

fn score(m: &Match) -> u32 {
    let shape_score = match m.1 {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    let outcome_score = match m {
        (Rock, Paper) => 6,
        (Paper, Scissors) => 6,
        (Scissors, Rock) => 6,

        (Rock, Rock) => 3,
        (Paper, Paper) => 3,
        (Scissors, Scissors) => 3,

        _ => 0,
    };

    shape_score + outcome_score
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("Read fail");

    let input: Input = all_consuming(parse_input)(&buffer).expect("Parse fail").1;

    #[cfg(debug_assertions)]
    dbg!(&input);

    let score_sum: u32 = input.iter().map(|x| score(x)).sum();

    println!("{}", score_sum);
}
