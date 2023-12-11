use std::collections::HashMap;

use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::{map, all_consuming};
use nom::sequence::{delimited, separated_pair};

struct PuzzleInput {
    instructions: Vec<char>,
    desert_map: HashMap<String, (String, String)>,
}

fn main() {
    let input = include_str!("./inputs/d08-input.txt");
    let puzzle = parse_input(input);

    let start = "AAA".to_string();

    let mut count: u64 = 0;
    let mut location: &String = &start;

    'outer: loop {
        for dir in &puzzle.instructions {
            let (left, right) = puzzle.desert_map.get(location).expect("Could not find location");
            match dir {
                'L' => {
                    location = left;
                },
                'R' => {
                    location = right;
                },
                _ => panic!("Unknown direction {}", dir),
            }

            count += 1;
            
            if "ZZZ".eq(location) {
                break 'outer;
            }
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

    PuzzleInput { instructions, desert_map }
}

fn node_line(input: &str) -> IResult<&str, (String, (String, String))> {
    let node_pair = delimited(tag("("), separated_pair(node_id, tag(", "), node_id), tag(")"));
    separated_pair(node_id, tag(" = "), node_pair)(input)
}

fn node_id(input: &str) -> IResult<&str, String> {
    map(alpha1, str::to_string)(input)
}
