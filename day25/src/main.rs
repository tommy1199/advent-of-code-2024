use std::{fs, iter::zip};

fn main() {
    let input = fs::read_to_string("input.txt").expect("works");
    let (keys, locks) = extract_keys_and_locks(&input);
    println!("part1: {}", find_non_overlapping(&keys, &locks));
}

fn find_non_overlapping(keys: &Vec<Vec<u8>>, locks: &Vec<Vec<u8>>) -> u64 {
    let mut result = 0u64;
    for key in keys {
        for lock in locks {
            if !zip(key, lock).any(|(k, l)| k + l > 5) {
                result += 1;
            }
        }
    }
    result
}

fn extract_keys_and_locks(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut keys: Vec<Vec<u8>> = vec![];
    let mut locks: Vec<Vec<u8>> = vec![];
    for el in input.split("\n\n") {
        if el.starts_with("#") {
            locks.push(extract(el));
        } else {
            keys.push(extract(el));
        }
    }
    (keys, locks)
}

fn extract(element: &str) -> Vec<u8> {
    let mut result: Vec<Vec<char>> = element.lines().map(|line| line.chars().collect()).collect();
    result = (0..result[0].len())
        .map(|i| {
            result
                .iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<char>>()
        })
        .collect();
    result
        .into_iter()
        .map(|v| (v.iter().filter(|c| **c == '#').count()) as u8 - 1)
        .collect()
}
