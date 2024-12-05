use std::{cmp::Ordering, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("read file works");
    let (rules, updates) = (extract_lines(&input, '|'), extract_lines(&input, ','));
    let (part1_result, part2_result) = calc_results(&rules, &updates);
    println!("part1: {}", part1_result);
    println!("part2: {}", part2_result);
}

fn extract_lines(input: &str, delimiter: char) -> Vec<Vec<&str>> {
    input
        .lines()
        .filter(|line| line.contains(delimiter))
        .map(|line| line.split(delimiter).collect())
        .collect()
}

fn calc_results(rules: &Vec<Vec<&str>>, updates: &Vec<Vec<&str>>) -> (u32, u32) {
    let (mut result1, mut result2) = (0u32, 0u32);
    for update in updates {
        let sorted = sort_update(update, rules);
        let middle = sorted[sorted.len() / 2].parse::<u32>().unwrap();
        if sorted == *update {
            result1 += middle;
        } else {
            result2 += middle;
        }
    }
    (result1, result2)
}

fn sort_update<'a>(update: &Vec<&'a str>, rules: &Vec<Vec<&str>>) -> Vec<&'a str> {
    let mut result = update.clone();
    result.sort_by(|a, b| {
        if rules.contains(&vec![*a, *b]) {
            Ordering::Less
        } else if rules.contains(&vec![*b, *a]) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    result
}
