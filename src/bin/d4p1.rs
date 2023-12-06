use std::collections::HashSet;


fn main() {
    let input = include_str!("./inputs/d4-input.txt");
    let result: usize = input.lines().map(game_score).sum();

    println!("Answer: {}", result);
}

fn game_score(input: &str) -> usize {
    let (_, input) = input.split_once(':').expect("Could not split by ':'");
    let (left, right) = input.split_once('|').expect("Could not split by '|'");

    let winners = parse_numbers_list(left);
    let numbers = parse_numbers_list(right);

    let count = winners.intersection(&numbers).count();

    real_score(count)
}

fn parse_numbers_list(input: &str) -> HashSet<u32> {
    input.trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect()
}

fn real_score(wins: usize) -> usize {
    if wins > 0 {
        1 << (wins - 1)
    } else {
        0
    }
}
