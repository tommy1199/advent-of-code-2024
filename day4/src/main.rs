use std::{char, fs, usize};

use diagonal::{diagonal_pos_neg, diagonal_pos_pos, straight_x, straight_y};

fn main() {
    let input = fs::read_to_string("input.txt").expect("read file works");
    println!("part1: {}", find_xmas(&input));
    println!("part2: {}", find_crossed_mas(&input));
}

fn find_xmas(input: &str) -> usize {
    let matrix = to_matrix(input);
    let all_directions = get_all_directions(&matrix);

    all_directions
        .iter()
        .map(|dir| String::from_iter(dir.clone()))
        .map(|dir| dir.matches("XMAS").count() + dir.matches("SAMX").count())
        .sum::<usize>()
}

fn find_crossed_mas(input: &str) -> usize {
    let matrix = to_matrix(input);
    let mut result = 0usize;
    for i in 1..(matrix.len() - 1) {
        for j in 1..(matrix[0].len() - 1) {
            let center = matrix[i][j];
            if center == 'A' {
                let mut dir_pos = [matrix[i - 1][j - 1], matrix[i + 1][j + 1]];
                let mut dir_neg = [matrix[i + 1][j - 1], matrix[i - 1][j + 1]];
                dir_pos.sort();
                dir_neg.sort();
                if dir_pos == ['M', 'S'] && dir_neg == ['M', 'S'] {
                    result += 1;
                }
            }
        }
    }
    result
}

fn get_all_directions(matrix: &Vec<Vec<char>>) -> Vec<Vec<&char>> {
    [
        &straight_x(matrix)[..],
        &straight_y(matrix)[..],
        &diagonal_pos_pos(matrix)[..],
        &diagonal_pos_neg(matrix)[..],
    ]
    .concat()
}

fn to_matrix(input: &str) -> Vec<Vec<char>> {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    matrix
}
