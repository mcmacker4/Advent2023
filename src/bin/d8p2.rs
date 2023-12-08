use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::{all_consuming, map};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

struct PuzzleInput {
    instructions: Vec<char>,
    desert_map: HashMap<String, (String, String)>,
}

impl PuzzleInput {
    fn apply_instr(&self, instr: char, pos: &String) -> &String {
        let (left, right) = self.desert_map.get(pos).unwrap();

        match instr {
            'L' => left,
            'R' => right,
            _ => panic!("Unknown instruction {}", instr),
        }
    }
}

struct InstrIter<'a> {
    instr: &'a Vec<char>,
    position: usize,
}

impl<'a> InstrIter<'a> {
    fn new(instr: &'a Vec<char>) -> Self {
        Self { instr, position: 0 }
    }
}

impl<'a> Iterator for InstrIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.position;
        self.position = pos + 1;

        Some(self.instr[pos % self.instr.len()])
    }
}

fn main() {
    let input = include_str!("./inputs/d8-p2-example.txt");
    let puzzle = parse_input(input);

    let instr_iter = InstrIter::new(&puzzle.instructions);
    let mut positions: Vec<&String> = puzzle
        .desert_map
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect();

    println!("Starting positions: {}", positions.len());

    let mut count: u64 = 0;

    for instr in instr_iter {
        for i in 0..positions.len() {
            let pos = positions.get(i).unwrap();
            let new_pos = puzzle.apply_instr(instr, pos);
            positions[i] = new_pos;
        }

        count += 1;

        if positions.iter().all(|pos| pos.ends_with('Z')) {
            break;
        }

        if count % 100000 == 0 {
            println!("Count: {}", count);
        }
    }

    println!("Answer: {}", count);
}

fn parse_input(input: &str) -> PuzzleInput {
    let mut lines = input.lines();

    let instructions: Vec<char> = lines
        .next()
        .expect("Instructions line expected")
        .chars()
        .collect();

    // Discard blank line
    lines.next();

    let mut desert_map = HashMap::new();

    for line in lines {
        let (_, (origin, dest)) = all_consuming(node_line)(line).expect("Could not parse input");
        desert_map.insert(origin, dest);
    }

    PuzzleInput {
        instructions,
        desert_map,
    }
}

fn node_line(input: &str) -> IResult<&str, (String, (String, String))> {
    let node_pair = delimited(
        tag("("),
        separated_pair(node_id, tag(", "), node_id),
        tag(")"),
    );
    separated_pair(node_id, tag(" = "), node_pair)(input)
}

fn node_id(input: &str) -> IResult<&str, String> {
    map(alphanumeric1, str::to_string)(input)
}
