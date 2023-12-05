use advent2023::d2::{parse_games, Game, Subset, CubeColor};



fn main() {
    let input = include_str!("./inputs/d2-input.txt");
    let games = parse_games(input).unwrap();

    let result: u32 = games.iter()
        .map(game_minimums)
        .map(|(red, green, blue)| red * green * blue)
        .sum();

    println!("Answer: {}", result);
}


/// Red, Green, Blue
fn game_minimums(game: &Game) -> (u32, u32, u32) {
    game.subsets.iter().fold((0, 0, 0), subset_reducer)
}

fn subset_reducer((r, g, b): (u32, u32, u32), subset: &Subset) -> (u32, u32, u32) {
    let sr = subset.get(&CubeColor::Red).map_or(0, |v| *v);
    let sg = subset.get(&CubeColor::Green).map_or(0, |v| *v);
    let sb = subset.get(&CubeColor::Blue).map_or(0, |v| *v);

    (sr.max(r), sg.max(g), sb.max(b))
}
