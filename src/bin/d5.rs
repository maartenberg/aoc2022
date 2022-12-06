use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, line_ending, multispace1, u32},
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};
use std::io::Read;

#[derive(Debug)]
struct Instruction {
    amount: u32,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from(x: (u32, (u32, u32))) -> Self {
        Self {
            amount: x.0,
            from: x.1 .0 as usize,
            to: x.1 .1 as usize,
        }
    }
}

type InitialState = Vec<Vec<char>>;
type Input = (InitialState, Vec<Instruction>);

fn parse_input(input: &str) -> IResult<&str, Input> {
    pair(parse_stacks, parse_instructions)(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(map(
        terminated(
            pair(
                preceded(tag("move "), u32),
                pair(preceded(tag(" from "), u32), preceded(tag(" to "), u32)),
            ),
            line_ending,
        ),
        Instruction::from,
    ))(input)
}

fn parse_box(input: &str) -> IResult<&str, Option<char>> {
    alt((
        value(None, tag("   ")),
        map(delimited(char('['), anychar, char(']')), |x| Some(x)),
    ))(input)
}

fn parse_stacks(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, boxes) = terminated(
        separated_list1(alt((tag(" "), line_ending)), parse_box),
        line_ending,
    )(input)?;

    let (input, labels) = preceded(
        char(' '),
        terminated(separated_list1(multispace1, u32), multispace1),
    )(input)?;
    let num_labels = labels.len();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for i in 0..num_labels {
        stacks.push(
            boxes
                .iter()
                .skip(i)
                .step_by(num_labels)
                .filter_map(|&s| s)
                .rev()
                .collect(),
        );
    }

    IResult::Ok((input, stacks))
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("Read fail");

    let input: Input = all_consuming(parse_input)(&buffer).expect("Parse fail").1;

    #[cfg(debug_assertions)]
    dbg!(&input);

    let (mut stacks, instrs) = input;

    for instr in instrs {
        for _ in 0..instr.amount {
            let b = stacks[instr.from - 1].pop().expect("Empty stack");
            stacks[instr.to - 1].push(b);
        }
    }

    for mut stack in stacks {
        print!("{}", stack.pop().expect("Empty stack"));
    }
    print!("\n");
}
