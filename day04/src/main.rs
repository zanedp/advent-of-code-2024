const EXAMPLE_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Direction(isize, isize);
impl Direction {
    const NORTH: Self = Self(0, -1);
    const NORTH_EAST: Self = Self(1, -1);
    const EAST: Self = Self(1, 0);
    const SOUTH_EAST: Self = Self(1, 1);
    const SOUTH: Self = Self(0, 1);
    const SOUTH_WEST: Self = Self(-1, 1);
    const WEST: Self = Self(-1, 0);
    const NORTH_WEST: Self = Self(-1, -1);

    pub fn next(&self) -> Self {
        match *self {
            Self::NORTH => Self::NORTH_EAST,
            Self::NORTH_EAST => Self::EAST,
            Self::EAST => Self::SOUTH_EAST,
            Self::SOUTH_EAST => Self::SOUTH,
            Self::SOUTH => Self::SOUTH_WEST,
            Self::SOUTH_WEST => Self::WEST,
            Self::WEST => Self::NORTH_WEST,
            Self::NORTH_WEST => Self::NORTH,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    pub row: isize,
    pub col: isize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row: row as isize,
            col: col as isize,
        }
    }

    pub fn step(&self, direction: Direction) -> Self {
        Self {
            row: (self.row + direction.1),
            col: (self.col + direction.0),
        }
    }
}

fn main() {
    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let needle = "XMAS";
    let result = find_word(&grid, needle);
    // dbg!(&result);
    dbg!(result.map(|x| x.len()));
}

fn find_word(grid: &[Vec<char>], word: &str) -> Option<Vec<(Position, Direction)>> {
    let mut result = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let cur_pos = Position::new(row, col);
            let start_direction = Direction::NORTH;
            let mut cur_direction = start_direction;
            loop {
                if is_word_at(grid, word, cur_pos, cur_direction) {
                    result.push((cur_pos, cur_direction));
                }

                cur_direction = cur_direction.next();
                if cur_direction == start_direction {
                    break;
                }
            }
        }
    }

    if !result.is_empty() {
        Some(result)
    } else {
        None
    }
}

fn is_in_grid(grid: &[Vec<char>], pos: Position) -> bool {
    pos.row >= 0
        && pos.col >= 0
        && pos.row < (grid.len() as isize)
        && pos.col < (grid[pos.row as usize].len() as isize)
}

fn is_word_at(grid: &[Vec<char>], word: &str, start_pos: Position, direction: Direction) -> bool {
    let mut cur_pos = start_pos;
    for ch in word.chars() {
        if !is_in_grid(grid, cur_pos) {
            return false;
        }

        if grid[cur_pos.row as usize][cur_pos.col as usize] != ch {
            return false;
        }

        cur_pos = cur_pos.step(direction);
    }
    true
}
