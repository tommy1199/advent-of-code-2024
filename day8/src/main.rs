use std::collections::HashMap;
use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("read file works");
    let (all, antennas) = extract_antennas(&input);
    println!("part1: {}", calculate_antinodes_count(&all, &antennas, false));
    println!("part2: {}", calculate_antinodes_count(&all, &antennas, true));
}

fn extract_antennas(input: &str) -> (Vec<(i32, i32)>, HashMap<char, Vec<(i32, i32)>>) {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut all_pos = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let pos = (j as i32, i as i32);
            all_pos.push(pos);
            if c != '.' {
                antennas.entry(c).or_default().push(pos);
            }
        }
    }
    (all_pos, antennas)
}

fn calculate_antinodes_count(
    all: &Vec<(i32, i32)>,
    antennas: &HashMap<char, Vec<(i32, i32)>>,
    looping: bool
) -> i32 {
    let mut result: Vec<(i32, i32)> = Vec::new();
    for (_, positions) in antennas.iter() {
        result.extend(calc_antinodes(all, positions, looping));
        if looping &&  positions.len() > 1 {
            result.extend(positions);
        }
    }
    result
        .iter()
        .filter(|antinode| all.contains(antinode))
        .unique()
        .count() as i32
}

fn calc_antinodes(all: &Vec<(i32, i32)>, positions: &Vec<(i32, i32)>, looping: bool) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new();
    if positions.len() < 2 {
        return antinodes;
    } else if positions.len() >= 2 {
        antinodes.extend(&calc_antinodes(all, &positions[1..].to_vec(), looping));
    }
    let first = positions[0];
    for second in positions.iter().skip(1) {
        let x_diff = second.0 - first.0;
        let y_diff = second.1 - first.1;
        antinodes.extend(calc_antinode(all, looping, first, -x_diff, -y_diff));
        antinodes.extend(calc_antinode(all, looping, *second, x_diff, y_diff));
    }
    antinodes
}

fn calc_antinode(all: &Vec<(i32, i32)>, looping: bool, pos: (i32, i32), x_offset: i32, y_offset: i32) -> Vec<(i32, i32)>{
    let mut result = Vec::new();
    let antinode = (pos.0 + x_offset, pos.1 + y_offset);
    if !all.contains(&antinode) {
        return result;
    }
    result.push(antinode);
    if looping {
        result.extend(calc_antinode(all, looping, antinode, x_offset, y_offset));
    }
    result
}
