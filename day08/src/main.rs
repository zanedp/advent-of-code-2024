use std::collections::{HashMap, HashSet};

const EXAMPLE_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

fn main() {
    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    let input_grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // maps frequency to list of locations
    let mut antenna_locations = HashMap::new();
    for (r, row) in input_grid.iter().enumerate() {
        for (c, antenna_id) in row.iter().enumerate() {
            if antenna_id.is_alphabetic() || antenna_id.is_numeric() {
                antenna_locations
                    .entry(*antenna_id)
                    .or_insert(Vec::new())
                    .push((r, c));
            }
        }
    }

    let map_dimensions = (input_grid.len(), input_grid[0].len());

    // --- Part 1 ---
    let mut antinodes_pt1 = HashSet::new();
    for (_freq_name, locations) in antenna_locations.iter() {
        for loc0 in locations.iter() {
            for loc1 in locations.iter() {
                if loc0 != loc1 {
                    // dbg!(loc0, loc1);
                    // (rise, run)
                    let slope = (
                        loc1.0 as isize - loc0.0 as isize,
                        loc1.1 as isize - loc0.1 as isize,
                    );
                    let antinode0 = (loc0.0 as isize - slope.0, loc0.1 as isize - slope.1);
                    let antinode1 = (loc1.0 as isize + slope.0, loc1.1 as isize + slope.1);
                    if is_in_range(antinode0, map_dimensions) {
                        antinodes_pt1.insert(antinode0);
                    }
                    if is_in_range(antinode1, map_dimensions) {
                        antinodes_pt1.insert(antinode1);
                    }
                }
            }
        }
    }
    // antinodes.iter().for_each(|row| {
    //     println!("{:?}", row);
    // });
    dbg!(antinodes_pt1.len());

    // --- Part 2 ---
    let mut antinodes_pt2 = HashSet::new();
    for (_freq_name, locations) in antenna_locations.iter() {
        for loc0 in locations.iter() {
            for loc1 in locations.iter() {
                if loc0 != loc1 {
                    // dbg!(loc0, loc1);
                    // (rise, run)
                    let slope = (
                        loc1.0 as isize - loc0.0 as isize,
                        loc1.1 as isize - loc0.1 as isize,
                    );
                    // let's go one way, until we get off the map
                    let mut cur_pos = (loc0.0 as isize, loc0.1 as isize);
                    loop {
                        let antinode = (cur_pos.0 - slope.0, cur_pos.1 - slope.1);
                        if !is_in_range(antinode, map_dimensions) {
                            break;
                        }
                        antinodes_pt2.insert(antinode);
                        cur_pos = antinode;
                    }
                    // now, let's go the other way, until we get off the map
                    let mut cur_pos = (loc0.0 as isize, loc0.1 as isize);
                    loop {
                        let antinode = (cur_pos.0 + slope.0, cur_pos.1 + slope.1);
                        if !is_in_range(antinode, map_dimensions) {
                            break;
                        }
                        antinodes_pt2.insert(antinode);
                        cur_pos = antinode;
                    }
                }
            }
        }
    }
    // antinodes.iter().for_each(|row| {
    //     println!("{:?}", row);
    // });
    dbg!(antinodes_pt2.len());
}

fn is_in_range(loc: (isize, isize), map_dimensions: (usize, usize)) -> bool {
    loc.0 >= 0
        && loc.0 < map_dimensions.0 as isize
        && loc.1 >= 0
        && loc.1 < map_dimensions.1 as isize
}
