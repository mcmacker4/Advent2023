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
            TileKind::Empty => ' ',
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

    fn is_in_bounds(&self, pos: (i64, i64)) -> bool {
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
        let tile_map = TileMap {
            size: (rows as i64, columns as i64),
            tiles,
        };

        let result = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .iter()
        .map(|dir| navigate_path(&tile_map, start, *dir))
        .filter(Option::is_some)
        .next();

        if let Some(Some(distance)) = result {
            println!("Answer: {}", (distance as f64 / 2.0).ceil());
        } else{
            println!("No answer found");
        }
    } else {
        panic!("No starting tile found");
    }
}

fn navigate_path(map: &TileMap, mut pos: (i64, i64), mut dir: Direction) -> Option<i64> {
    let mut distance: i64 = 0;
    loop {
        let next_pos = dir.walk(pos);

        if !map.is_in_bounds(next_pos) {
            return None;
        }

        let next_tile = map.tile_at(next_pos);
        match next_tile {
            TileKind::Pipe(dir1, dir2) => {
                if dir1.opposite() == dir || dir2.opposite() == dir {
                    let next_dir = if dir1.opposite() == dir { *dir2 } else { *dir1 };
                    distance += 1;
                    pos = next_pos;
                    dir = next_dir;
                } else {
                    return None;
                }
            }
            TileKind::Start => {
                return Some(distance);
            }
            TileKind::Empty => {
                return None;
            }
        }
    }
}
