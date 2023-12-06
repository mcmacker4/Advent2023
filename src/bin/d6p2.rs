use advent2023::d6::{Race, calculate_race};



fn main() {
    let input = include_str!("./inputs/d6-input.txt");
    let race = parse_input(input);

    let (min, max) = calculate_race(&race);

    println!("Answer: {}", (max - min - 1.0));
}


pub fn parse_input(input: &str) -> Race {
    let mut lines = input.lines();

    let time_line = lines.next().expect("Expected a line with the times");
    let dist_line = lines.next().expect("Expected a line with the distances");

    let time_str = time_line.split_once(' ').unwrap().1.split_whitespace().collect::<String>();
    let dist_str = dist_line.split_once(' ').unwrap().1.split_whitespace().collect::<String>();

    Race {
        time: time_str.parse().unwrap(),
        record: dist_str.parse().unwrap(),
    }
}
