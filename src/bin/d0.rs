use nom::{combinator::all_consuming, IResult};
use std::io::Read;

type Input = u32;

fn parse_input(input: &str) -> IResult<&str, Input> {
    todo!();
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("Read fail");

    let input: Input = all_consuming(parse_input)(&buffer).expect("Parse fail").1;

    #[cfg(debug_assertions)]
    dbg!(&input);

    todo!();
}
