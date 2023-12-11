use std::{str::Chars, iter::Peekable, collections::HashMap};


fn main() {
    let input = include_str!("./inputs/d03-input.txt");

    println!("{}", input);

    let (numbers, symbols) = parse_input(input);

    let mut cells = HashMap::<(i64, i64), Vec<u64>>::new();

    for num in &numbers {
        for x in -1..=num.digits {
            for y in -1..=1 {
                let pos = (num.row + y, num.column + x);

                if let Some(nums) = cells.get_mut(&pos) {
                    nums.push(num.value);
                } else {
                    let nums = vec![num.value];
                    cells.insert(pos, nums);
                }
            }
        }
    }

    let result: u64 = symbols.iter()
        .filter(|s| s.value == '*')
        .map(|s| {
            if let Some(nums) = cells.get(&(s.row, s.column)) {
                if nums.len() == 2 {
                    nums[0] * nums[1]
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();

    println!("Answer: {}", result);
}

#[derive(Debug)]
struct NumberPos {
    value: u64,
    row: i64,
    column: i64,
    digits: i64,
}

#[derive(Debug)]
struct SymbolPos {
    value: char,
    row: i64,
    column: i64,
}

fn parse_input(input: &str) -> (Vec<NumberPos>, Vec<SymbolPos>) {
    let mut numbers = vec![];
    let mut symbols = vec![];

    for (row, line) in input.lines().enumerate() {
        let mut column: i64 = 0;
        let mut iter = line.chars().peekable();
        while let Some(_) = iter.peek() {
            column += skip_dots(&mut iter);
            if let Some(c) = iter.peek() {
                if c.is_digit(10) {
                    let mut value: u64 = 0;
                    let start_col = column;
                    loop {
                        match iter.peek() {
                            Some(d) if d.is_digit(10) => {
                                let digit = d.to_digit(10).unwrap() as u64;
                                value = value * 10 + digit;
                                iter.next().unwrap();
                                column += 1;
                            },
                            _ => {
                                numbers.push(NumberPos {
                                    value,
                                    row: row as i64,
                                    column: start_col,
                                    digits: (column - start_col) as i64,
                                });
                                break;
                            }
                        }
                    }
                } else {
                    let value = *c;
                    iter.next().unwrap();
                    symbols.push(SymbolPos {
                        value,
                        row: row as i64, column,
                    });
                    column += 1;
                }
            }
        }
    }

    (numbers, symbols)
}

fn skip_dots(iter: &mut Peekable<Chars>) -> i64 {
    let mut count = 0;
    while let Some(c) = iter.peek() {
        if *c != '.' {
            break;
        } else {
            iter.next();
            count += 1;
        }
    }
    count
}
