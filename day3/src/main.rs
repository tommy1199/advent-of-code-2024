use std::fs;
use std::str::FromStr;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").expect("read file works");
    println!("part1: {}", calc_product_sum(&input));
    println!("part2: {}", calc_product_sum_conditionally(&input));
}

fn calc_product_sum(input: &str) -> i64 {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    mul_regex
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| [i64::from_str(a).unwrap(), i64::from_str(b).unwrap()])
        .map(|[a, b]| a * b)
        .sum()
}

fn calc_product_sum_conditionally(input: &str) -> i64 {
    Regex::new(r"don't\(\)(.|\r|\n)+?($|do\(\))")
        .unwrap()
        .split(input)
        .map(|active| calc_product_sum(active))
        .sum()
}
