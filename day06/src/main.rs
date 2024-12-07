use std::collections::HashSet;

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

#[allow(dead_code, reason = "For debugging purposes")]
fn print_map(map: &[Vec<char>]) {
    map.iter().for_each(|row| {
        println!("{}", row.iter().collect::<String>());
    });
}

fn main() {
    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let final_map = walk_until_off_map(&map);
    let distinct_positions = final_map
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter().enumerate().filter_map(move |(c, &cell)| {
                if cell == 'X' {
                    Some(Position(r as isize, c as isize))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<Position>>();
    let distinct_positions_count = distinct_positions.len();
    dbg!(distinct_positions_count);

    let start_pos = find_start(&map); // let's not look for it every single time!
    let num_positions_that_cause_loops = distinct_positions
        .iter()
        .filter(|&pos| *pos != start_pos)
        .map(|pos| {
            // add the obstacle at every point on the original route
            let mut map = map.clone();
            map[pos.0 as usize][pos.1 as usize] = '#';
            map
        })
        .filter(|x| has_loop(x, start_pos))
        .count();
    dbg!(num_positions_that_cause_loops);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn walk_until_off_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
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

fn has_loop(map: &[Vec<char>], start: Position) -> bool {
    let mut pos = start;
    let mut dir = Direction::UP;
    let mut visited_positions: HashSet<(Position, Direction)> = HashSet::new();
    loop {
        if let Some((new_pos, new_dir)) = find_next_pos(pos, dir, map) {
            if !visited_positions.insert((new_pos, new_dir)) {
                // this wasn't the first time we visited this position in this direction,
                // so we have a loop.
                return true;
            } else {
                // let's keep walking
                pos = new_pos;
                dir = new_dir;
            }
        } else {
            return false;
        }
    }
}
