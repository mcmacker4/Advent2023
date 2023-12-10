use std::collections::HashSet;

use colored::Colorize;

// Solved by the Even-Odd rule
// https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    fn walk(&self, (row, col): (i64, i64)) -> (i64, i64) {
        match self {
            Self::North => (row - 1, col),
            Self::South => (row + 1, col),
            Self::West => (row, col - 1),
            Self::East => (row, col + 1),
        }
    }
}

#[derive(Clone, Copy)]
enum TileKind {
    Pipe(Direction, Direction),
    Start,
    Empty,
}

impl From<char> for TileKind {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Pipe(Direction::North, Direction::South),
            '-' => Self::Pipe(Direction::West, Direction::East),
            'L' => Self::Pipe(Direction::North, Direction::East),
            'J' => Self::Pipe(Direction::North, Direction::West),
            '7' => Self::Pipe(Direction::South, Direction::West),
            'F' => Self::Pipe(Direction::South, Direction::East),
            '.' => Self::Empty,
            'S' => Self::Start,
            _ => panic!("Invalid character {}", value),
        }
    }
}

impl From<&TileKind> for char {
    fn from(value: &TileKind) -> char {
        match value {
            TileKind::Pipe(Direction::North, Direction::South) => '│',
            TileKind::Pipe(Direction::West, Direction::East) => '─',
            TileKind::Pipe(Direction::North, Direction::East) => '└',
            TileKind::Pipe(Direction::North, Direction::West) => '┘',
            TileKind::Pipe(Direction::South, Direction::West) => '┐',
            TileKind::Pipe(Direction::South, Direction::East) => '┌',
            TileKind::Empty => 'O',
            TileKind::Start => '╋',
            _ => '?',
        }
    }
}

struct TileMap {
    size: (i64, i64),
    tiles: Vec<TileKind>,
}

impl TileMap {
    fn tile_at(&self, pos: (i64, i64)) -> &TileKind {
        &self.tiles[self.idx(pos)]
    }

    fn is_in_bounds(&self, pos: &(i64, i64)) -> bool {
        pos.0 >= 0 && pos.0 < self.size.0 && pos.1 >= 0 && pos.1 < self.size.1
    }

    fn idx(&self, pos: (i64, i64)) -> usize {
        (pos.0 * self.size.1 + pos.1) as usize
    }
}

fn main() {
    let input = include_str!("./inputs/d10-input.txt");

    let mut lines = input.lines().peekable();
    let columns = lines.peek().expect("First line expected").len();
    let rows = lines.count();

    let mut tiles: Vec<TileKind> = vec![TileKind::Empty; rows * columns];

    let mut start: Option<(i64, i64)> = None;

    for (row, line) in input.lines().enumerate() {
        for (col, tile_char) in line.chars().enumerate() {
            let tile = tile_char.into();
            tiles[row * columns + col] = tile;
            if let TileKind::Start = tile {
                start = Some((row as i64, col as i64));
            }
        }
    }

    if let Some(start) = start {
        let mut tile_map = TileMap {
            size: (rows as i64, columns as i64),
            tiles,
        };

        let (chosen_dir, start_pipe) = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .iter()
        .flat_map(|dir| {
            check_path(&tile_map, start, *dir).map(|dir2| {
                (*dir, TileKind::Pipe(*dir, dir2.opposite()))
            })
        })
        .next().expect("Loop not found");

        let start_idx = tile_map.idx(start);
        tile_map.tiles[start_idx] = start_pipe;

        let loop_tiles = collect_known_loop(&tile_map, start, chosen_dir);
        let contained_tiles = calculate_area_within(&tile_map, &loop_tiles);

        print_colored_map(&tile_map, &loop_tiles, &contained_tiles, start);

        println!("Area within: {}", contained_tiles.len());
    } else {
        panic!("No starting tile found");
    }
}

fn check_path(map: &TileMap, mut pos: (i64, i64), mut dir: Direction) -> Option<Direction> {
    loop {
        let next_pos = dir.walk(pos);

        if !map.is_in_bounds(&next_pos) {
            return None;
        }

        let next_tile = map.tile_at(next_pos);
        match next_tile {
            TileKind::Pipe(dir1, dir2) => {
                if dir1.opposite() == dir || dir2.opposite() == dir {
                    let next_dir = if dir1.opposite() == dir { *dir2 } else { *dir1 };
                    pos = next_pos;
                    dir = next_dir;
                } else {
                    return None;
                }
            }
            TileKind::Start => {
                return Some(dir);
            }
            TileKind::Empty => {
                return None;
            }
        }
    }
}

fn collect_known_loop(
    map: &TileMap,
    mut pos: (i64, i64),
    mut dir: Direction,
) -> HashSet<(i64, i64)> {
    let mut loop_tiles = HashSet::new();
    loop {
        let next_pos = dir.walk(pos);
        if loop_tiles.contains(&next_pos) {
            break;
        }

        if let TileKind::Pipe(dir1, dir2) = map.tile_at(next_pos) {
            loop_tiles.insert(next_pos);

            pos = next_pos;
            dir = if dir1.opposite() == dir { *dir2 } else { *dir1 };
        } else {
            panic!("Next is not a pipe");
        }
    }
    loop_tiles
}

fn calculate_area_within(map: &TileMap, loop_tiles: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut contained_tiles = HashSet::<(i64, i64)>::new();

    for row in 0..map.size.0 {
        for col in 0..map.size.1 {
            if !loop_tiles.contains(&(row, col)) && is_tile_contained((row, col), map, loop_tiles) {
                contained_tiles.insert((row, col));
            }
        }
    }

    contained_tiles
}

fn is_tile_contained(mut pos: (i64, i64), map: &TileMap, loop_tiles: &HashSet<(i64, i64)>) -> bool {
    let mut crossings: u64 = 0;
    loop {
        pos = (pos.0 - 1, pos.1 + 1);

        if !map.is_in_bounds(&pos) {
            break;
        }

        if !loop_tiles.contains(&pos) {
            continue;
        }

        if is_loop_tile_crossed(map.tile_at(pos)) {
            crossings += 1;
        }
    }
    crossings % 2 == 1
}

fn is_loop_tile_crossed(tile_kind: &TileKind) -> bool {
    if let TileKind::Pipe(dir1, dir2) = tile_kind {
        dir1.opposite() == *dir2
            || *dir1 == Direction::North && *dir2 == Direction::East
            || *dir2 == Direction::North && *dir1 == Direction::East
            || *dir1 == Direction::South && *dir2 == Direction::West
            || *dir2 == Direction::South && *dir1 == Direction::West
    } else {
        false
    }
}

fn print_colored_map(map: &TileMap, loop_tiles: &HashSet<(i64, i64)>, contained_tiles: &HashSet<(i64, i64)>, start: (i64, i64)) {
    for row in 0..map.size.0 {
        for col in 0..map.size.1 {
            let tile = map.tile_at((row, col));
            let c = format!("{}", char::from(tile));
            let colored = if (row, col) == start {
                c.red()
            } else if contained_tiles.contains(&(row, col)) {
                c.blue()
            } else if loop_tiles.contains(&(row, col)) {
                c.green()
            } else {
                c.normal()
            };
            print!("{}", colored);
        }
        println!();
    }
}
