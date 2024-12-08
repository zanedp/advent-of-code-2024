const EXAMPLE_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn multiply(a: u64, b: u64) -> u64 {
    a * b
}

fn concatenate(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse().unwrap()
}

fn has_solution(operators: &[fn(u64, u64) -> u64], goal: u64, accum: u64, list: &[u64]) -> bool {
    if list.is_empty() {
        return accum == goal;
    }
    let (operand, rest) = list.split_first().unwrap();
    for operator in operators {
        let new_accum = operator(accum, *operand);
        if has_solution(operators, goal, new_accum, rest) {
            return true;
        }
    }
    false
}

fn main() {
    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");
    let equations = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts.next().unwrap().parse::<u64>().unwrap();
            let operands = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|operand| operand.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (result, operands)
        })
        .collect::<Vec<_>>();
    // dbg!(&equations);

    let total_calibration_result_pt1 = equations
        .iter()
        .filter(|(result, operands)| has_solution(&[add, multiply], *result, 0, operands))
        .map(|(result, _)| result)
        .sum::<u64>();
    dbg!(total_calibration_result_pt1);

    let total_calibration_result_pt2 = equations
        .iter()
        .filter(|(result, operands)| {
            has_solution(&[add, multiply, concatenate], *result, 0, operands)
        })
        .map(|(result, _)| result)
        .sum::<u64>();
    dbg!(total_calibration_result_pt2);
}
