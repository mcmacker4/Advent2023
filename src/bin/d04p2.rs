use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./inputs/d04-input.txt");

    let mut copies = HashMap::<usize, usize>::new();

    let mut result: usize = 0;
    for (i, line) in input.lines().enumerate() {
        result += game_score(i, line, &mut copies);
    }

    println!("Answer: {}", result);
}

fn game_score(idx: usize, input: &str, copies: &mut HashMap<usize, usize>) -> usize {
    let (_, input) = input.split_once(':').expect("Could not split by ':'");
    let (left, right) = input.split_once('|').expect("Could not split by '|'");

    let winners = parse_numbers_list(left);
    let numbers = parse_numbers_list(right);

    let win_count = winners.intersection(&numbers).count();
    let current_copies = copies.get(&idx).map_or(1, |v| *v);

    for i in 1..=win_count {
        let cidx = idx + i;
        let n = copies.get(&cidx).map_or(1, |v| *v);
        copies.insert(cidx, n + current_copies);
    }

    current_copies
}

fn parse_numbers_list(input: &str) -> HashSet<u32> {
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}
