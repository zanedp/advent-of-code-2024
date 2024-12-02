use std::collections::HashMap;
use std::iter::zip;

const EXAMPLE_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

fn main() {
    let input_text = EXAMPLE_INPUT;
    let input_text = include_str!("input.txt");
    let reports = input_text
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    reports
        .iter()
        .for_each(|report| println!("{:?} -> {}", report, is_safe(report)));
    let safe_count = reports.iter().filter(|report| is_safe(report)).count();
    dbg!(safe_count);
}

fn is_decreasing(report: &[i64]) -> bool {
    report.windows(2).all(|pair| pair[0] > pair[1])
}

fn is_increasing(report: &[i64]) -> bool {
    report.windows(2).all(|pair| pair[0] < pair[1])
}

fn max_delta(report: &[i64]) -> i64 {
    report
        .windows(2)
        .map(|pair| pair[0].abs_diff(pair[1]) as i64)
        .max()
        .inspect(|&x| println!("max_delta: {}", x))
        .unwrap()
}

fn is_safe(report: &[i64]) -> bool {
    (is_decreasing(report) || is_increasing(report)) && max_delta(report).abs() <= 3
}
