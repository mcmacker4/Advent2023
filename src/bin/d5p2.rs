use advent2023::d5::{parse_input, Almanac, IdRange, Mapping, TypeMap};

fn main() {
    let input = include_str!("./inputs/d5-input.txt");
    let Almanac {
        seed_ranges,
        maps,
        seeds: _,
    } = parse_input(input);

    let mapped_ranges = maps.iter().fold(seed_ranges, |ranges, type_map| {
        map_ranges(type_map, &ranges)
    });

    let result = mapped_ranges.iter().map(|r| r.start).min().expect("No answer found");

    println!("Answer: {}", result);
}

fn map_ranges(type_map: &TypeMap, ranges: &Vec<IdRange>) -> Vec<IdRange> {
    ranges
        .iter()
        .flat_map(|range| map_single_range(type_map, range))
        .collect()
}

fn map_single_range(type_map: &TypeMap, range: &IdRange) -> Vec<IdRange> {
    let mut mapped_ranges: Vec<(IdRange, i64)> = type_map
        .iter()
        .flat_map(|mapping| find_overlap(mapping, range))
        .collect();

    mapped_ranges.sort_by_key(|(r, _)| r.start);
    let gaps = find_gaps(range, &mapped_ranges);

    let mut all_ranges: Vec<IdRange> = mapped_ranges.iter().map(apply_offset).collect();

    all_ranges.extend(&gaps);
    all_ranges
}

fn find_gaps(origin: &IdRange, ranges: &Vec<(IdRange, i64)>) -> Vec<IdRange> {
    
    if ranges.is_empty() {
        return vec![origin.clone()];
    }

    let mut gaps = vec![];

    if let Some((first, _)) = ranges.first() {
        if origin.start < first.start {
            let gap = IdRange {
                start: origin.start,
                end: first.start,
            };
            gaps.push(gap);
        }
    }

    if let Some((last, _)) = ranges.last() {
        if last.end < origin.end {
            let gap = IdRange {
                start: last.end,
                end: origin.end,
            };
            gaps.push(gap);
        }
    }

    let mut iter = ranges.iter();
    if let Some((range_a, _)) = iter.next() {
        let mut range_a = range_a;
        while let Some((range_b, _)) = iter.next() {
            if range_a.end < range_b.start {
                let gap = IdRange {
                    start: range_a.end,
                    end: range_a.start,
                };
                gaps.push(gap);
            }
            range_a = range_b;
        }
    }

    gaps
}

fn apply_offset((range, offset): &(IdRange, i64)) -> IdRange {
    IdRange {
        start: range.start + *offset,
        end: range.end + *offset,
    }
}

fn find_overlap(mapping: &Mapping, range: &IdRange) -> Option<(IdRange, i64)> {
    let m_start = mapping.source;
    let m_end = mapping.source + mapping.length;

    let r_start = range.start;
    let r_end = range.end;

    let diff = mapping.dest - mapping.source;

    if r_end > m_start && r_start < m_end {
        let range = IdRange {
            start: m_start.max(r_start),
            end: m_end.min(r_end),
        };
        Some((range, diff))
    } else {
        None
    }
}
