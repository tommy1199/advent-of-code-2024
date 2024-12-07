use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("read file works");
    let equations = get_equations(&input);

    let multiply = |a: i64, b: i64| a * b;
    let add = |a: i64, b: i64| a + b;
    let concat = |a: i64, b: i64| (a.to_string() + &b.to_string()).parse::<i64>().unwrap();
    let result_part1 = equations
        .iter()
        .filter(|equation| is_valid(equation, &vec![multiply, add]))
        .map(|equation| equation.0)
        .sum::<i64>();
    println!("{}", result_part1);
    let result_part2 = equations
        .iter()
        .filter(|equation| is_valid(equation, &vec![multiply, add, concat]))
        .map(|equation| equation.0)
        .sum::<i64>();
    println!("{}", result_part2);
}

fn get_equations(input: &str) -> Vec<(i64, Vec<i64>)> {
    let equations: Vec<(i64, Vec<i64>)> = input
        .lines()
        .map(|line| line.split(":").collect::<Vec<&str>>())
        .map(|parts| {
            (
                parts[0].parse::<i64>().unwrap(),
                parts[1]
                    .split_whitespace()
                    .map(|operand| operand.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .collect();
    equations
}

fn is_valid(equation: &(i64, Vec<i64>), ops: &Vec<fn(i64, i64) -> i64>) -> bool {
    any_valid(equation.0, equation.1[0], &equation.1[1..], ops)
}

fn any_valid(expected: i64, current: i64, tail: &[i64], ops: &Vec<fn(i64, i64) -> i64>) -> bool {
    if tail.is_empty() {
        return current == expected;
    } else if current > expected {
        return false;
    }
    ops.iter()
        .any(|op| any_valid(expected, op(current, tail[0]), &tail[1..], ops))
}
