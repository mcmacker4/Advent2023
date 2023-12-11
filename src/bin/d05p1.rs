use advent2023::d05::{parse_input, Almanac, TypeMap, Mapping};

fn main() {
    let input = include_str!("./inputs/d05-input.txt");
    let almanac = parse_input(input);

    let result = almanac
        .seeds
        .iter()
        .map(|id| translate_id(*id, &almanac))
        .min()
        .expect("Could not find answer");

    println!("Answer: {}", result);
}

fn translate_id(mut id: i64, almanac: &Almanac) -> i64 {
    for map in almanac.maps.iter() {
        id = apply_map(id, map);
    }
    id
}

fn apply_map(id: i64, map: &TypeMap) -> i64 {
    for mapping in map.iter() {
        if let Some(new_id) = apply_mapping(id, mapping) {
            return new_id;
        }
    }
    id
}

fn apply_mapping(id: i64, mapping: &Mapping) -> Option<i64> {
    if id >= mapping.source && id < mapping.source + mapping.length {
        Some(id + (mapping.dest - mapping.source))
    } else {
        None
    }
}
