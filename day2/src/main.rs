use std::str::FromStr;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("read file works");
    println!("part1: {}", contents.lines().filter(|line| is_safe(line, false)).count());
    println!("part2: {}", contents.lines().filter(|line| is_safe(line, true)).count());
}

fn is_safe(line: &str, with_tolerance: bool) -> bool {
    let report: Vec<i64> = line.split_whitespace().map(|s| i64::from_str(s).unwrap()).collect();
    if with_tolerance {
        create_variants(&report).iter().any(|variant| is_safe_report(variant))
    } else {
        is_safe_report(&report)
    }
}

fn is_safe_report(report: &Vec<i64>) -> bool {
    let mut safe = true;
    let increasing = report[1] - report[0] > 0; 
    for (a, b) in report.iter().zip(report.iter().skip(1)) {
        if a.abs_diff(*b) < 1 || a.abs_diff(*b) > 3 || increasing != (b - a > 0) {
            safe = false;
            break;
        }
    }
    safe
}

fn create_variants(report: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut report_variants: Vec<Vec<i64>> = Vec::new();
    for (idx, _) in report.iter().enumerate() {
        let mut variant = report.clone(); 
        variant.remove(idx);
        report_variants.push(variant);
    }
    report_variants.push(report.clone());
    report_variants
}
