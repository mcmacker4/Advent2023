fn main() {
    let input = include_str!("./inputs/d01-input.txt");

    let mut result = 0;

    for line in input.lines() {
        let first = find_digit_left(line).expect("Could not find left");
        let last = find_digit_right(line).expect("Could not find right");

        result = result + first * 10 + last;
    }

    println!("{}", result);
}

fn find_digit_left(input: &str) -> Option<u32> {
    for i in 0..input.len() {
        let part = &input[i..];
        if let Some(num) = to_digit(part) {
            return Some(num);
        }
    }
    None
}

fn find_digit_right(input: &str) -> Option<u32> {
    for i in (0..input.len()).rev() {
        let part = &input[i..];
        if let Some(num) = to_digit(part) {
            return Some(num);
        }
    }
    None
}

const PATTERNS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn to_digit(input: &str) -> Option<u32> {
    if let Some(first_char) = input.chars().next() {
        if let Some(n) = first_char.to_digit(10) {
            return Some(n);
        }
    }

    for (i, pat) in PATTERNS.iter().enumerate() {
        if input.starts_with(pat) {
            return Some(i as u32);
        }
    }
    None
}
