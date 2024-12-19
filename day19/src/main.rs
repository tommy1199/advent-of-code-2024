use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("works");
    let (towels, designs) = extract(&input);
    println!("part1: {}", find_possible_designs(&designs, &towels));
    println!("part2: {}", find_possible_design_combis(&designs, &towels));
}

fn extract(input: &str) -> (Vec<&str>, Vec<&str>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let towels = parts[0].split(",").map(|towel| towel.trim()).collect();
    let designs = parts[1].lines().collect();
    (towels, designs)
}

fn find_possible_designs(designs: &Vec<&str>, towels: &Vec<&str>) -> usize {
    designs
        .iter()
        .filter(|design| is_design_possible(design, towels, &mut HashMap::new()) > 0)
        .count()
}

fn find_possible_design_combis(designs: &Vec<&str>, towels: &Vec<&str>) -> usize {
    designs
        .iter()
        .map(|design| is_design_possible(design, towels, &mut HashMap::new()))
        .sum()
}

fn is_design_possible(design: &str, towels: &Vec<&str>, pattern_cache: &mut HashMap<String, usize>) -> usize {
    if let Some(count) = pattern_cache.get(design) {
        return *count;
    }
    if design.is_empty() {
        return 1;
    }
    let mut result = 0;
    for towel in towels {
        if let Some(design_rest) = design.strip_prefix(towel) {
            result += is_design_possible(design_rest, towels, pattern_cache)
        }
    }
    pattern_cache.insert(design.to_string(), result);
    result
}
