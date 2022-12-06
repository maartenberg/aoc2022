use nom::{
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::many1,
    sequence::terminated,
    IResult,
};
use std::collections::HashSet;
use std::io::Read;

type Rucksack = HashSet<char>;
type Input = Vec<Rucksack>;

fn parse_rucksack(input: &str) -> IResult<&str, Rucksack> {
    let (input, line) = terminated(alpha1, line_ending)(input)?;

    let mut set = HashSet::new();

    for c in line.chars() {
        set.insert(c);
    }

    IResult::Ok((input, set))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    many1(parse_rucksack)(input)
}

fn value(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - '`' as u32
    } else {
        c as u32 - '@' as u32 + 26
    }
}

fn main() {
    #[cfg(debug_assertions)]
    {
        assert_eq!(value('p'), 16);
        assert_eq!(value('L'), 38);
    }
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("Read fail");

    let input: Input = all_consuming(parse_input)(&buffer).expect("Parse fail").1;

    #[cfg(debug_assertions)]
    dbg!(&input);

    let mut score = 0;
    for chunk in input.chunks_exact(3) {
        if let [a, b, c] = chunk {
            let inter1: HashSet<char> = a.intersection(b).map(|&x| x).collect();
            let overlap = inter1.intersection(c).next().expect("No overlap");

            score += value(*overlap);
        } else {
            unreachable!();
        }
    }

    println!("{}", score);
}
