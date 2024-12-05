use std::{cmp::Ordering, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("read file works");
    let rules = extract_lines(&input, '|');
    let updates = extract_lines(&input, ',');
    println!("part1: {}", calc_correct_updates_result(&rules, &updates));
    println!("part2: {}", calc_incorrect_updates_result(&rules, &updates));
}

fn extract_lines(input: &str, delimiter: char) -> Vec<Vec<&str>> {
    input
        .lines()
        .filter(|line| line.contains(delimiter))
        .map(|line| line.split(delimiter).collect())
        .collect()
}

fn calc_correct_updates_result(rules: &Vec<Vec<&str>>, updates: &Vec<Vec<&str>>) -> u32 {
    updates
        .iter()
        .filter(|update| is_correct_update(*update, &rules))
        .map(|update| get_middle_page(&update))
        .sum::<u32>()
}

fn calc_incorrect_updates_result(rules: &Vec<Vec<&str>>, updates: &Vec<Vec<&str>>) -> u32 {
    updates
        .iter()
        .filter(|update| !is_correct_update(*update, &rules))
        .map(|update| sort_update(update, &rules))
        .map(|update| get_middle_page(&update))
        .sum::<u32>()
}

fn get_middle_page(update: &Vec<&str>) -> u32 {
    update[update.len() / 2].parse::<u32>().unwrap()
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

fn is_correct_update(update: &Vec<&str>, rules: &Vec<Vec<&str>>) -> bool {
    for (i, page) in update.iter().enumerate() {
        let next = &update[i..update.len()];
        if rules
            .iter()
            .filter(|rule| *page == rule[1])
            .any(|rule| next.contains(&rule[0]))
        {
            return false;
        }
    }
    true
}
