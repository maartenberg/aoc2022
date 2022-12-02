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
    Rock,
    Paper,
    Scissors,
}
use Move::*;

#[derive(Clone, Debug)]
enum Outcome {
    Win,
    Draw,
    Loss,
}
use Outcome::*;

type Input = Vec<Match>;
type Match = (Move, Outcome);

fn parse_input(input: &str) -> IResult<&str, Input> {
    many1(terminated(
        separated_pair(parse_oppmove, char(' '), parse_outcome),
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

fn parse_outcome(input: &str) -> IResult<&str, Outcome> {
    alt((
        value(Loss, char('X')),
        value(Draw, char('Y')),
        value(Win, char('Z')),
    ))(input)
}

fn own_move(m: &Match) -> Move {
    let (opponent_move, outcome) = m;
    match outcome {
        Draw => opponent_move.clone(),
        Win => match opponent_move {
            Scissors => Rock,
            Paper => Scissors,
            Rock => Paper,
        },
        Loss => match opponent_move {
            Scissors => Paper,
            Paper => Rock,
            Rock => Scissors,
        },
    }
}

fn score(m: &Match) -> u32 {
    let own_move = own_move(m);

    let shape_score = match own_move {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    let outcome_score = match m.1 {
        Win => 6,
        Draw => 3,
        Loss => 0,
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
