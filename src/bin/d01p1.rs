fn main() {
    let input = include_str!("./inputs/d01-input.txt");

    let mut result = 0;

    for line in input.lines() {
        let first_digit = line
            .chars()
            .find_map(|c| c.to_digit(10))
            .expect("Digit not found in line");
        let last_digit = line
            .chars()
            .rev()
            .find_map(|c| c.to_digit(10))
            .expect("Digit not found in line");
        result = result + (first_digit * 10 + last_digit);
    }

    println!("{}", result);
}
