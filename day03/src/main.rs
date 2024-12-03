use regex::Regex;

// const EXAMPLE_INPUT: &str =
//     r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
const EXAMPLE_INPUT: &str =
    r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

fn main() {
    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    let mul_sum_pt1 = get_pairs(input).iter().fold(0, |acc, (x, y)| x * y + acc);
    dbg!(mul_sum_pt1);

    let start_time = std::time::Instant::now();
    let enabled_at_start = input.split("do()");
    let disables_removed = enabled_at_start.map(|x| x.split("don't()").next().unwrap());
    let recombined = disables_removed.collect::<String>();
    let mul_sum_pt2 = get_pairs(&recombined)
        .iter()
        .fold(0, |acc, (x, y)| x * y + acc);
    dbg!(start_time.elapsed());
    dbg!(mul_sum_pt2);
}

fn get_pairs(input: &str) -> Vec<(u64, u64)> {
    Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|captures| {
            let a = captures.name("a").unwrap().as_str().parse::<u64>().unwrap();
            let b = captures.name("b").unwrap().as_str().parse::<u64>().unwrap();
            (a, b)
        })
        .collect()
}
