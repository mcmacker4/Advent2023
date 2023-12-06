use std::{iter::Peekable, str::Lines};

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<i64>,
    pub seed_ranges: Vec<SeedRange>,
    pub maps: Vec<TypeMap>,
}

#[derive(Debug)]
pub struct SeedRange {
    pub start: i64,
    pub end: i64,
}

pub type TypeMap = Vec<Mapping>;

#[derive(Debug)]
pub struct Mapping {
    pub source: i64,
    pub dest: i64,
    pub length: i64,
}

pub fn parse_input(input: &str) -> Almanac {
    let mut lines = input.lines().peekable();
    let seeds = parse_seed_list(&mut lines);

    let seed_ranges = into_ranges(&seeds);

    let maps = parse_maps(&mut lines);

    Almanac {
        seeds,
        seed_ranges,
        maps,
    }
}

fn parse_seed_list(lines: &mut Peekable<Lines>) -> Vec<i64> {
    let line = lines.next().expect("Could not read seeds line");

    let (_, seed_list) = line.split_once(':').expect("Could not split seeds at ':'");

    let seeds: Vec<i64> = seed_list
        .split_whitespace()
        .map(|v| v.parse::<i64>().expect("Could not parse seed number"))
        .collect();

    lines.next(); // Discard empty line
    seeds
}

fn parse_maps(lines: &mut Peekable<Lines>) -> Vec<TypeMap> {
    let mut maps = vec![];
    while let Some(_) = lines.peek() {
        let map = parse_single_map(lines);
        maps.push(map);
    }
    maps
}

fn parse_single_map(lines: &mut Peekable<Lines>) -> TypeMap {
    let mut mappings = TypeMap::new();

    if let Some(line) = lines.next() {
        if !line.ends_with(":") {
            panic!("Parsing map header, line does not end with ':'");
        }
    }

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mapping = parse_mapping(line);
        mappings.push(mapping);
    }

    mappings
}

fn parse_mapping(line: &str) -> Mapping {
    let mut parts = line.split_whitespace().map(|v| v.parse::<i64>().expect("Could not parse mapping number"));
    let dest = parts.next().expect("Could not get mapping destination");
    let source = parts.next().expect("Could not get mapping source");
    let length = parts.next().expect("Could not get mapping length");
    Mapping { source, dest, length }
}

fn into_ranges(seeds: &Vec<i64>) -> Vec<SeedRange> {
    let mut iter = seeds.iter();
    let mut ranges = vec![];
    while let Some(start) = iter.next() {
        if let Some(len) = iter.next() {
            ranges.push(SeedRange { start: *start, end: *start + *len });
        }
    }
    ranges
}
