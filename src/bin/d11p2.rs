use std::{
    collections::HashMap,
    hash::Hash,
};

#[derive(Debug)]
struct Space {
    rows: u64,
    cols: u64,
    galaxies: Vec<GalaxyPos>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct GalaxyPos(u64, u64);

fn main() {
    let input = include_str!("./inputs/d11-input.txt");
    let space = parse_input(input);

    let result: u64 = unique_pairs(&space.galaxies).into_iter().map(|(gal1, gal2)| {
        gal1.0.abs_diff(gal2.0) + gal1.1.abs_diff(gal2.1)
    }).sum();

    println!("Answer: {}", result);
}

fn parse_input(input: &str) -> Space {
    let chars: Vec<char> = input.lines().flat_map(str::chars).collect();

    let rows = input.lines().count() as u64;
    let cols = chars.len() as u64 / rows;

    let mut galaxies = Vec::new();

    for (i, c) in chars.iter().enumerate() {
        if *c == '#' {
            galaxies.push(GalaxyPos(i as u64 / cols, i as u64 % cols));
        }
    }

    expand_space(Space { rows, cols, galaxies }, &chars)
}

fn expand_space(Space { rows, cols, galaxies }: Space, chars: &Vec<char>) -> Space {
    let mut row_maps = HashMap::<u64, u64>::new();
    let mut col_maps = HashMap::<u64, u64>::new();

    let mut row_offset: u64 = 0;
    for row in 0..rows {
        if is_row_empty(row, cols, chars) {
            row_offset += 999999;
        }
        row_maps.insert(row, row + row_offset);
    }

    let mut col_offset: u64 = 0;
    for col in 0..cols {
        if is_col_empty(col, cols, chars) {
            col_offset += 999999;
        }
        col_maps.insert(col, col + col_offset);
    }

    let galaxies = galaxies
        .into_iter()
        .map(|GalaxyPos(row, col)| GalaxyPos(row_maps[&row], col_maps[&col]))
        .collect();

    Space {
        rows: rows + row_offset,
        cols: cols + col_offset,
        galaxies,
    }
}

fn is_row_empty(row: u64, cols: u64, chars: &Vec<char>) -> bool {
    chars.iter().enumerate().filter_map(|(i, c)| {
        if i as u64 / cols == row { Some(c) } else { None }
    }).all(|c| *c == '.')
}

fn is_col_empty(col: u64, cols: u64, chars: &Vec<char>) -> bool {
    chars.iter().enumerate().filter_map(|(i, c)| {
        if i as u64 % cols == col { Some(c) } else { None }
    }).all(|c| *c == '.')
}

fn unique_pairs<'a>(galaxies: &'a Vec<GalaxyPos>) -> Vec<(&'a GalaxyPos, &'a GalaxyPos)> {
    let mut pairs = Vec::new();
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            pairs.push((&galaxies[i], &galaxies[j]));
        }
    }
    pairs
}
