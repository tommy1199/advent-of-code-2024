use std::collections::HashMap;
use std::fs;

use itertools::Itertools;
use memoize::memoize;

fn main() {
    let input = fs::read_to_string("input.txt").expect("file read works");
    let stones: HashMap<u64, u64> = extract_stones_by_value(&input);
    println!("part1: {}", calculate_stones(&stones, 25));
    println!("part2: {}", calculate_stones(&stones, 75));
}

fn calculate_stones(stones: &HashMap<u64, u64>, blinks: u8) -> u64 {
    let mut result = stones.clone();
    for _ in 0..blinks {
        result = result
            .iter()
            .flat_map(|(k, v)| {
                apply_rule(*k)
                    .iter()
                    .map(move |value| (value.clone(), *v))
                    .collect::<Vec<(u64, u64)>>()
            })
            .into_group_map_by(|(k, _)| *k)
            .into_iter()
            .map(|(k, group)| (k, group.into_iter().fold(0, |acc, (_, value)| acc + value)))
            .collect();
    }
    
    result.values().sum::<u64>()
}

fn extract_stones_by_value(input: &str) -> HashMap<u64, u64> {
    input
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .fold(HashMap::new(), |mut acc, value| {
            acc.insert(value, acc.get(&value).unwrap_or(&0u64) + 1);
            acc
        })
}

#[memoize]
fn apply_rule(number: u64) -> Vec<u64> {
    if number == 0 {
        [1].to_vec()
    } else if number.to_string().len() % 2 == 0 {
        [
            number.to_string()[0..number.to_string().len() / 2]
                .parse::<u64>()
                .unwrap(),
            number.to_string()[number.to_string().len() / 2..]
                .parse::<u64>()
                .unwrap(),
        ]
        .to_vec()
    } else {
        [number * 2024].to_vec()
    }
}
