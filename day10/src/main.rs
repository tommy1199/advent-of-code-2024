use itertools::Itertools;
use std::fs::read_to_string;

const TRAIL_HEAD: u8 = 0;
const TOP: u8 = 9;
const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    let input = read_to_string("input.txt").expect("file read works");
    let map = extract_map(&input);
    let mut result_part1 = 0u32;
    let mut result_part2 = 0u32;
    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == TRAIL_HEAD {
                result_part1 += find_trails(height, x as i32, y as i32, &map);
                result_part2 += find_distinct_trails(height, x as i32, y as i32, &map);
            }
        }
    }

    println!("part1: {}", result_part1);
    println!("part2: {}", result_part2);
}

fn extract_map(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn find_trails(curr_height: &u8, x: i32, y: i32, map: &Vec<Vec<u8>>) -> u32 {
    find_reachable_tops(curr_height, x, y, map)
        .iter()
        .unique()
        .count() as u32
}

fn find_distinct_trails(curr_height: &u8, x: i32, y: i32, map: &Vec<Vec<u8>>) -> u32 {
    find_reachable_tops(curr_height, x, y, map).len() as u32
}

fn find_reachable_tops(curr_height: &u8, x: i32, y: i32, map: &Vec<Vec<u8>>) -> Vec<(i32, i32)> {
    if *curr_height == TOP {
        return vec![(x, y)];
    }
    let new_height = curr_height + 1;
    let mut result: Vec<(i32, i32)> = Vec::new();
    for dir in DIRECTIONS {
        let new_x = x + dir.0;
        let new_y = y + dir.1;
        match map
            .get(new_y as usize)
            .map(|line| line.get(new_x as usize))
            .flatten()
        {
            Some(height) if *height == new_height => {
                result.extend(&find_reachable_tops(&new_height, new_x, new_y, map));
            }
            _ => continue,
        }
    }
    result
}
