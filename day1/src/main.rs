use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("read file works"); 
    let lines = contents.lines();
    let mut left = Vec::<i64>::new();
    let mut right = Vec::<i64>::new();
    for line in lines {
        let mut numbers = line.split_whitespace();
        left.push(numbers.next().unwrap().parse::<i64>().unwrap());
        right.push(numbers.next().unwrap().parse::<i64>().unwrap());
    }
    left.sort();
    right.sort();
    let mut result_part1 = 0i64;
    for (i, v) in left.iter().enumerate() {
        result_part1 = result_part1 + (v.abs_diff(right[i]) as i64);
    }
    println!("part1: {}", result_part1);
    
    let result_part2: i64 = left.iter().map(|v| v * (right.iter().filter(|&n| *n == *v).count() as i64)).sum();
    println!("part2: {}", result_part2);
}
