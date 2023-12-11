use advent2023::d02::{parse_games, CubeColor, Subset, Game};

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let input = include_str!("./inputs/d02-input.txt");
    let games = parse_games(input).unwrap();

    let result: u32 = games.iter()
        .filter(|g| is_game_valid(g))
        .map(|g| g.id)
        .sum();

    println!("The answer is {}", result);
}

fn is_game_valid(game: &Game) -> bool {
    !game.subsets.iter().any(|s| !is_subset_valid(s))
}

fn is_subset_valid(subset: &Subset) -> bool {
    subset.get(&CubeColor::Red).map_or(0, |v| *v) <= MAX_RED
        && subset.get(&CubeColor::Green).map_or(0, |v| *v) <= MAX_GREEN
        && subset.get(&CubeColor::Blue).map_or(0, |v| *v) <= MAX_BLUE
}
