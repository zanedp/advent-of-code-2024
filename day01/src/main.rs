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
    let input = input_text.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let a: u64 = parts.next().unwrap().parse().unwrap();
        let b: u64 = parts.next().unwrap().parse().unwrap();
        (a, b)
    });
    let mut a = vec![];
    let mut b = vec![];
    for (x, y) in input {
        a.push(x);
        b.push(y);
    }
    a.sort();
    b.sort();

    let distances = zip(a.iter(), b.iter()).map(|(&x, &y)| x.abs_diff(y));
    let total_distance: u64 = distances.sum();
    dbg!(total_distance);

    let mut right_count: HashMap<u64, u64> = HashMap::new();
    for &x in b.iter() {
        // *right_count.entry(x).or_insert(0) += 1;
        *right_count.entry(x).or_default() += 1;
    }
    let similarity_score = a.iter().fold(0, |acc, x| {
        acc + x * right_count.get(x).copied().unwrap_or_default()
    });
    dbg!(similarity_score);
}
