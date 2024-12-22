use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

type Pos = (u32, u32);

const DIRS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    let input = fs::read_to_string("input.txt").expect("works");
    let (start, end, walls, width, height) = extract_race_track(&input);
    let path = find_shortest_path(&start, &end, width, height, &walls);
    println!("part1: {}", find_working_cheats(&path, 100, 2));
    println!("part2: {}", find_working_cheats(&path, 100, 20));
}

fn find_working_cheats(path: &Vec<Pos>, saves_min: u32, allowed_jumps: u32) -> u32 {
    let mut result = 0u32;

    for (i, pos) in path.iter().enumerate() {
        for (j, next) in path.iter().enumerate().skip(i + 1) {
            let dst = (pos.0 as i32 - next.0 as i32).abs() + (pos.1 as i32 - next.1 as i32).abs();
            if dst <= allowed_jumps as i32 && (j as i32 - i as i32) as i32 - dst >= saves_min as i32
            {
                result += 1;
            }
        }
    }
    result
}

fn extract_race_track(input: &str) -> (Pos, Pos, Vec<Pos>, u32, u32) {
    let mut start: Pos = (0, 0);
    let mut end: Pos = (0, 0);
    let mut walls: Vec<Pos> = Vec::new();
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x as u32, y as u32);
            } else if c == 'E' {
                end = (x as u32, y as u32);
            } else if c == '#' {
                walls.push((x as u32, y as u32));
            }
        }
    }
    (start, end, walls, width as u32, height as u32)
}

fn find_shortest_path(start: &Pos, end: &Pos, width: u32, height: u32, walls: &[Pos]) -> Vec<Pos> {
    let mut dsts: HashMap<Pos, u32> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(u32, Pos, Vec<Pos>)>> = BinaryHeap::new();

    for x in 0..width {
        for y in 0..height {
            dsts.insert((x, y), u32::MAX);
        }
    }
    dsts.insert(*start, 0);
    queue.push(Reverse((0, *start, vec![*start])));
    while let Some(Reverse((dst, pos, path))) = queue.pop() {
        if pos == *end {
            return path;
        }
        for dir in DIRS {
            let x = pos.0 as i32 + dir.0 as i32;
            let y = pos.1 as i32 + dir.1 as i32;

            if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                continue;
            }

            let new_pos = (x as u32, y as u32);
            if walls.contains(&new_pos) {
                continue;
            }
            let new_dst = dst + 1;
            if new_dst < *dsts.get(&new_pos).unwrap() {
                let mut new_path = path.clone();
                new_path.push(new_pos);

                dsts.insert(new_pos, new_dst);
                queue.push(Reverse((new_dst, new_pos, new_path)));
            }
        }
    }
    vec![]
}
