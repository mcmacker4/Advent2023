use advent2023::d06::{Race, calculate_race};


fn main() {
    let input = include_str!("./inputs/d06-input.txt");
    let races = parse_input(input);

    let result = races.iter().map(|race| {
        let (min, max) = calculate_race(race);
        max - min - 1.0
    }).reduce(|acc, v| acc * v).expect("No answer found???");

    println!("Answer: {}", result);
}

pub fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();

    let time_line = lines.next().expect("Expected a line with the times");
    let dist_line = lines.next().expect("Expected a line with the distances");

    let times = time_line.split_whitespace().skip(1).map(|v| v.parse::<i64>().expect("Could not parse i64"));
    let records = dist_line.split_whitespace().skip(1).map(|v| v.parse::<i64>().expect("Could not parse i64"));

    times.zip(records).map(|(time, record)| Race { time, record }).collect()
}
