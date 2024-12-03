const EXAMPLE_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

fn main() {
    let input_text = EXAMPLE_INPUT;
    // let input_text = include_str!("input.txt");
    let reports = input_text
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let safe_count = reports.iter().filter(|report| is_safe(report)).count();
    dbg!(safe_count);

    let start_time = std::time::Instant::now();
    let safe_count_with_single_sample_removed = reports
        .iter()
        .filter(|report| {
            is_safe(report)
                || generate_cases_with_single_sample_removed(report)
                    .iter()
                    .any(|short_list| is_safe(short_list))
        })
        .count();
    dbg!(safe_count_with_single_sample_removed, start_time.elapsed());
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
        .unwrap()
}

fn is_safe(report: &[i64]) -> bool {
    (is_decreasing(report) || is_increasing(report)) && max_delta(report).abs() <= 3
}

fn remove_sample(report: &[i64], index: usize) -> Vec<i64> {
    let mut retval = Vec::new();
    let xs = report.iter().enumerate();
    for (i, &x) in xs {
        if i != index {
            retval.push(x);
        }
    }
    retval
}

fn generate_cases_with_single_sample_removed(report: &[i64]) -> Vec<Vec<i64>> {
    (0..report.len())
        .map(|i| remove_sample(report, i))
        .collect()
}

