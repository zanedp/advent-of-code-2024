use regex::Regex;

const EXAMPLE_INPUT: &str =
    r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
fn main() {
    let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    let re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    let mul_sum = re
        .captures_iter(input)
        .map(|captures| {
            let a = captures.name("a").unwrap().as_str().parse::<u64>().unwrap();
            let b = captures.name("b").unwrap().as_str().parse::<u64>().unwrap();
            (a, b)
        })
        .map(|(a, b)| a * b)
        .sum::<u64>();
    dbg!(mul_sum);
}
