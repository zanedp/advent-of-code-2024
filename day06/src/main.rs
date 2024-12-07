const EXAMPLE_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

fn main() {
    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let map = walk(&map);
    println!("Final map:");
    map.iter().for_each(|row| {
        println!("{}", row.iter().collect::<String>());
    });
    let distinct_positions = map
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| cell == 'X')
        .count();
    dbg!(distinct_positions);
}

fn find_start(map: &[Vec<char>]) -> Position {
    for (r, row) in map.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == '^' {
                return Position(r as isize, c as isize);
            }
        }
    }
    panic!("No start found");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Direction(isize, isize);
impl Direction {
    const UP: Direction = Direction(-1, 0);
    const DOWN: Direction = Direction(1, 0);
    const LEFT: Direction = Direction(0, -1);
    const RIGHT: Direction = Direction(0, 1);

    fn next(&self) -> Self {
        match *self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position(isize, isize);
impl Position {
    fn step(&self, dir: Direction) -> Self {
        Self(self.0 + dir.0, self.1 + dir.1)
    }
}

fn find_next_pos(
    start_pos: Position,
    start_dir: Direction,
    map: &[Vec<char>],
) -> Option<(Position, Direction)> {
    let mut dir = start_dir;
    loop {
        let next_pos = start_pos.step(dir);
        match map
            .get(next_pos.0 as usize)
            .and_then(|row| row.get(next_pos.1 as usize))
        {
            Some('#') => {
                dir = dir.next();
            }
            Some(_) => {
                return Some((next_pos, dir));
            }
            None => return None,
        }
    }
}

fn walk(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut map = map.to_vec();
    let start = find_start(&map);
    let mut pos = start;
    let mut dir = Direction::UP;

    loop {
        if let Some((new_pos, new_dir)) = find_next_pos(pos, dir, &map) {
            pos = new_pos;
            dir = new_dir;
            map[pos.0 as usize][pos.1 as usize] = 'X';
            // map.iter().for_each(|row| {
            //     println!("{}", row.iter().collect::<String>());
            // });
            // println!();
        } else {
            return map;
        }
    }
}
