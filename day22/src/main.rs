use std::collections::HashMap;
use std::fs;
use std::iter::zip;

const PRUNE: u64 = 16777216;

fn main() {
    let input = fs::read_to_string("input.txt").expect("works");
    let mut secrets = read_initial_secrets(&input);
    let best_sequence = calculate_best_sequence(&secrets);
    for _ in 0..2000 {
        secrets = secrets.into_iter().map(next_secret).collect();
    }
    println!("part1: {}", secrets.iter().sum::<u64>());
    println!("part2: {}", best_sequence);
}

fn deltas(secrets: &Vec<u64>) -> Vec<[i8; 2000]> {
    let mut deltas: Vec<[i8; 2000]> = Vec::new();
    for secret in secrets {
        let mut curr_secret = secret.clone();
        let mut ds = [0; 2000];
        for i in 0..2000 {
            let next_secret = next_secret(curr_secret);
            ds[i] = ((next_secret % 10) as i8 - (curr_secret % 10) as i8) as i8;
            curr_secret = next_secret;
        }
        deltas.push(ds);
    }
    deltas
}

fn calculate_best_sequence(secrets: &Vec<u64>) -> u64 {
    let mut sums: HashMap<(i8, i8, i8, i8), i64> = HashMap::new();
    let deltas = deltas(secrets);
    for (secret, ds) in zip(secrets, deltas) {
        let mut out: HashMap<(i8, i8, i8, i8), i64> = HashMap::new();
        let [mut d0, mut d1, mut d2] = ds[0..3] else {
            panic!("")
        };
        let mut v: i8 = secret.rem_euclid(10) as i8 + d0 + d1 + d2;
        for d3 in &ds[3..2000] {
            v = v + *d3;
            out.entry((d0, d1, d2, *d3)).or_insert(v as i64);
            d0 = d1;
            d1 = d2;
            d2 = *d3;
        }
        for (k, v) in out.into_iter() {
            *sums.entry(k).or_insert(0) += v;
        }
    }
    *sums.values().max().unwrap() as u64
}

fn read_initial_secrets(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

fn next_secret(secret: u64) -> u64 {
    let mut next_secret = ((secret * 64) ^ secret).rem_euclid(PRUNE);
    next_secret = ((next_secret as f64 / 32.0).floor() as u64 ^ next_secret).rem_euclid(PRUNE);
    next_secret = ((next_secret * 2048) ^ next_secret).rem_euclid(PRUNE);
    next_secret
}
