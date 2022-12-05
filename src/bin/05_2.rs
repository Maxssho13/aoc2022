#![feature(iter_next_chunk)]
use std::collections::VecDeque;

use nom::bytes::complete::tag;
use nom::character::complete::i32 as parse_i32;
use nom::{sequence::tuple, IResult};

fn parse_line(input: &str) -> IResult<&str, (&str, i32, &str, i32, &str, i32)> {
    tuple((
        tag("move "),
        parse_i32,
        tag(" from "),
        parse_i32,
        tag(" to "),
        parse_i32,
    ))(input)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = String::from_utf8(std::fs::read("inputs/05.txt")?)?;

    let first_line = input.lines().next().unwrap();
    let num_stacks = (first_line.len() + 1) / 4;

    let mut stacks: Vec<_> = (0..num_stacks).map(|_| VecDeque::default()).collect();

    let mut lines = input.lines();
    loop {
        let line = lines.next().unwrap();

        if !line.contains('[') {
            // consume the empty line
            lines.next();
            break;
        }

        let mut line_chars = line.chars();
        for stack in stacks.iter_mut() {
            let [_, tag, _] = line_chars.next_chunk().unwrap();
            if !tag.is_whitespace() {
                stack.push_back(tag);
            }
            line_chars.next();
        }
    }

    for line in lines {
        let (_, (_, count, _, from, _, to)) = parse_line(line).unwrap();
        let from = (from - 1) as usize;
        let to = (to - 1) as usize;

        for idx in 0..count {
            let popped = stacks[from].pop_front().unwrap();
            stacks[to].insert(idx as usize, popped);
        }
    }

    print!("output: '");
    for mut stack in stacks {
        print!("{}", stack.pop_front().unwrap());
    }
    println!("'");

    Ok(())
}
