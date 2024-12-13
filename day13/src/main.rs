use itertools::Itertools;
use regex::Regex;
use std::fs;

struct Arcade {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("file read works");
    let arcades = extract_input(&input);
    println!(
        "part1: {}",
        arcades
            .iter()
            .map(|arcade| find_cheapest_way(arcade, 0))
            .sum::<u64>()
    );
    println!(
        "part2: {}",
        arcades
            .iter()
            .map(|arcade| find_cheapest_way(arcade, 10_000_000_000_000))
            .sum::<u64>()
    );
}

fn find_cheapest_way(arcade: &Arcade, offset: u64) -> u64 {
    let prize_x: f64 = (arcade.prize.0 + offset) as f64;
    let prize_y: f64 = (arcade.prize.1 + offset) as f64;
    let a_x: f64 = arcade.button_a.0 as f64;
    let a_y: f64 = arcade.button_a.1 as f64;
    let b_x: f64 = arcade.button_b.0 as f64;
    let b_y: f64 = arcade.button_b.1 as f64;

    let det: f64 = a_x * b_y - a_y * b_x;
    let a_count: f64 = (b_y * prize_x - b_x * prize_y) / det;
    let b_count: f64 = (-a_y * prize_x + a_x * prize_y) / det;

    if b_count.fract() != 0.0 || a_count.fract() != 0.0 {
        0
    } else {
        a_count as u64 * 3 + b_count as u64
    }
}

fn extract_input(input: &str) -> Vec<Arcade> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .chunks(3)
        .into_iter()
        .map(|chunk| extract_arcade(chunk.collect()))
        .collect()
}

fn extract_arcade(lines: Vec<&str>) -> Arcade {
    Arcade {
        button_a: extract_pos(lines[0]),
        button_b: extract_pos(lines[1]),
        prize: extract_pos(lines[2]),
    }
}

fn extract_pos(line: &str) -> (u64, u64) {
    let regex = Regex::new(r"X.(\d+), Y.(\d+)").unwrap();
    let caps = regex.captures(line).unwrap();
    (
        caps.get(1)
            .map_or(0, |val| val.as_str().parse::<u64>().unwrap()),
        caps.get(2)
            .map_or(0, |val| val.as_str().parse::<u64>().unwrap()),
    )
}
