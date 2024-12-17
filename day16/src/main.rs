use std::{collections::HashMap, fs};
use itertools::Itertools;

type Pos = (i64, i64);
type Direction = (i64, i64);

#[derive(Debug)]
struct Maze {
    reindeer: Pos,
    grid: Vec<Vec<char>>,
    end: Pos,
}

const DIRS: [Direction; 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn main() {
    let input = fs::read_to_string("input.txt").expect("works");
    let mut maze = extract_maze(&input);
    let (result_part1, path) = djikstra(&maze);
    println!("part1: {}", result_part1);

    let mut paths: Vec<Pos> = Vec::new();
    paths.extend(path.clone());
    for (i, pos) in path.iter().enumerate() {
        println!("run djikstra for {} from {}", i, path.len() );
        if *pos == maze.reindeer || *pos == maze.end {
            continue;
        }
        maze.grid[pos.1 as usize][pos.0 as usize] = '#';
        let (result, more_path) = djikstra(&maze);
        if result == result_part1 {
            paths.extend(more_path);
        }
        maze.grid[pos.1 as usize][pos.0 as usize] = '.';
    }

    println!("part2: {}", paths.iter().unique().count());

}

fn djikstra(maze: &Maze) -> (u64, Vec<Pos>) {
    let mut queue: Vec<(u64, Pos, Direction, Vec<Pos>)> = vec![(0, maze.reindeer, (1, 0), vec![maze.reindeer])];
    let mut seen: HashMap<Pos, (u64, Vec<Pos>)> = HashMap::new();
    let mut min_cost = u64::MAX;
    while let Some((dst, pos, dir, path)) = queue.pop() {
        if dst > min_cost {
            continue
        }
        if pos == maze.end {
            min_cost = dst;
            continue;
        }
        for next_dir in DIRS {
            if next_dir.0 == -dir.0 && next_dir.1 == -dir.1 {
                continue;
            }
            let next_pos = (pos.0 + next_dir.0, pos.1 + next_dir.1);
            let is_straight = next_dir.0.abs() == dir.0.abs() && next_dir.1.abs() == dir.1.abs();
            let cost = 1 + if is_straight { 0 } else { 1000 };
            if maze.grid[next_pos.1 as usize][next_pos.0 as usize] == '#' {
                continue;
            }
            if seen.contains_key(&next_pos) && seen.get(&next_pos).unwrap().0 < cost + dst {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(next_pos);
            seen.insert(next_pos, (dst + cost, new_path.clone()));
            queue.push((dst + cost, next_pos, next_dir, new_path));
        } 
    }
    seen.get(&maze.end).unwrap_or(&(0, vec![])).clone() 
}

fn extract_maze(input: &str) -> Maze {
    let mut grid = Vec::new();
    let mut reindeer = (0, 0);
    let mut end = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'E' {
                end = (x as i64, y as i64);
            } else if c == 'S' {
                reindeer = (x as i64, y as i64);
            }
        }
        grid.push(line.chars().collect());
    }
    Maze {
        grid,
        reindeer,
        end,
    }
}
