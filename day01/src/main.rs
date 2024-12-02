use std::collections::HashMap;
use std::iter::zip;

const EXAMPLE_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

fn main() {
    let input_text = EXAMPLE_INPUT;
    // let input_text = include_str!("input.txt");
    let (mut left, mut right) = input_text
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let a: u64 = parts.next().unwrap().parse().unwrap();
            let b: u64 = parts.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect::<(Vec<_>, Vec<_>)>();
    left.sort();
    right.sort();

    // -- Part 1 ---
    let total_distance = zip(left.iter(), right.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum::<u64>();
    dbg!(total_distance);

    // -- Part 2 ---
    let mut right_count: HashMap<u64, u64> = HashMap::new();
    for &x in right.iter() {
        *right_count.entry(x).or_default() += 1;
    }
    let similarity_score = left.iter().fold(0, |acc, x| {
        acc + x * right_count.get(x).copied().unwrap_or_default()
    });
    dbg!(similarity_score);
}
