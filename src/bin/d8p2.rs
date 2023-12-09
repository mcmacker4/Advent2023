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
    fn apply_instr<'a>(&'a self, instr: char, pos: &'a String) -> &'a String {
        let (left, right) = self.desert_map.get(pos).unwrap();

        match instr {
            'L' => left,
            'R' => right,
            _ => panic!("Unknown instruction {}", instr),
        }
    }
}

fn main() {
    let input = include_str!("./inputs/d8-input.txt");
    let puzzle = parse_input(input);

    let mut primes = Primes::new();

    let positions: Vec<&String> = puzzle
        .desert_map
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect();

    println!("Starting positions: {}", positions.len());

    let mut lcm_factors = HashMap::<usize, usize>::new();

    for start in positions {
        let (loop_start, loop_length) = find_loop(&puzzle, start);
        println!(
            "Start '{}' has loop at offset {} of length {}",
            start, loop_start, loop_length
        );

        let factors = factorize(loop_length, &mut primes);
        for (factor, c) in &factors {
            let count = *lcm_factors.get(factor).unwrap_or(&0);
            lcm_factors.insert(*factor, count.max(*c));
        }

        println!("Prime factors of {}: {:?}", loop_length, factors);
    }

    let lcm = lcm_factors
        .iter()
        .flat_map(|(factor, count)| std::iter::repeat(*factor).take(*count))
        .fold(1usize, |acc, f| acc * f);

    println!("LCM: {}", lcm);
}

type Primes = Vec<usize>;

struct PrimeIter<'a> {
    primes: &'a mut Primes,
    candidate: usize,
}

impl<'a> PrimeIter<'a> {
    fn new(primes: &'a mut Primes) -> Self {
        Self {
            primes,
            candidate: 2,
        }
    }

    fn next(&mut self) -> usize {
        if let Some(prime) = self.primes.iter().filter(|x| **x >= self.candidate).next() {
            self.candidate = *prime + 1;
            return *prime;
        } else {
            loop {
                if self.primes.iter().all(|x| self.candidate % x != 0) {
                    let prime = self.candidate;
                    self.primes.push(prime);
                    self.candidate += 1;
                    return prime;
                } else {
                    self.candidate += 1;
                }
            }
        }
    }
}

fn factorize(num: usize, primes: &mut Primes) -> HashMap<usize, usize> {
    let mut rem: usize = num;
    let mut prime_iter = PrimeIter::new(primes);

    let mut factors = HashMap::<usize, usize>::new();

    while rem > 1 {
        let prime = prime_iter.next();
        while rem % prime == 0 {
            let count = *factors.get(&prime).unwrap_or(&0);
            factors.insert(prime, count + 1);
            rem = rem / prime;
        }
    }

    factors
}

#[derive(Hash, Eq, PartialEq)]
struct Visit<'a> {
    node: &'a String,
    instr_id: usize,
}

fn find_loop(puzzle: &PuzzleInput, start: &String) -> (usize, usize) {
    let mut visits = HashMap::<Visit, usize>::new();
    let mut position = start;

    let instr_iter = puzzle.instructions.iter().cycle();
    for (i, instr) in instr_iter.enumerate() {
        let visit = Visit {
            node: position,
            instr_id: i % puzzle.instructions.len(),
        };

        if let Some(start) = visits.get(&visit) {
            let loop_length = i - start;
            let inner_offset = next_z_distance(puzzle, position, i);
            return (*start + inner_offset, loop_length);
        } else {
            visits.insert(visit, i);
            position = puzzle.apply_instr(*instr, position);
        }
    }

    panic!("Should not get here");
}

fn next_z_distance<'a>(puzzle: &'a PuzzleInput, mut node: &'a String, instr_id: usize) -> usize {
    let instr_iter = puzzle.instructions.iter().cycle().skip(instr_id);
    let mut distance = 0;
    for instr in instr_iter {
        if node.ends_with('Z') {
            return distance;
        } else {
            node = puzzle.apply_instr(*instr, node);
            distance += 1;
        }
    }
    panic!("What the hell!");
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
