use nom::{
    character::complete::{line_ending, u32},
    combinator::all_consuming,
    multi::{fold_many1, separated_list1},
    sequence::terminated,
    IResult,
};
use std::io::Read;

type Input = Vec<u32>;

fn parse_input(input: &str) -> IResult<&str, Input> {
    separated_list1(
        line_ending,
        fold_many1(terminated(u32, line_ending), || 0, |x, y| x + y),
    )(input)
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("Read fail");

    let input: Input = all_consuming(parse_input)(&buffer).expect("Parse fail").1;

    #[cfg(debug_assertions)]
    dbg!(&input);

    let fattest_elf: &u32 = input.iter().max().expect("No elves");

    println!("{}", fattest_elf);
}
