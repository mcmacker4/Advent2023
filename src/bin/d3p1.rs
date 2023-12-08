use std::{str::Chars, iter::Peekable, collections::HashSet};


fn main() {
    let input = include_str!("./inputs/d3-input.txt");

    println!("{}", input);

    let (mut numbers, symbols) = parse_input(input);

    let mut actives = HashSet::new();

    for sym in &symbols {
        for x in -1..=1 {
            for y in -1..=1 {
                actives.insert((sym.row + y, sym.column + x));
            }
        }
    }

    println!("Symbols count: {}", symbols.len());
    println!("Actives count: {}", actives.len());

    for num in numbers.iter_mut() {
        for x in 0..num.digits {
            if actives.contains(&(num.row, num.column + x)) {
                num.visited = true;
            }
        }
    }

    let visited: Vec<&NumberPos> = numbers.iter().filter(|n| n.visited).collect();
    println!("Total Numbers: {}", numbers.len());
    for num in &numbers {
        println!("\t{:?}", num);
    }
    println!("Visited Numbers: {}", visited.len());
    for visited in &visited {
        println!("\t{:?}", *visited);
    }
    let result: u64 = visited.iter().map(|n| n.value).sum();

    println!("Answer: {}", result);
}

#[derive(Debug)]
struct NumberPos {
    value: u64,
    row: i64,
    column: i64,
    digits: i64,
    visited: bool,
}

#[derive(Debug)]
struct SymbolPos {
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
                                    visited: false,
                                });
                                break;
                            }
                        }
                    }
                } else {
                    iter.next().unwrap();
                    symbols.push(SymbolPos {
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
