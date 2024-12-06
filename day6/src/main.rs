use std::fs;

use itertools::Itertools;

#[derive(Eq, Clone, Hash, PartialEq, Debug)]
struct Pos(i32, i32);

#[derive(Debug, Clone, PartialEq)]
struct Direction(i8, i8);

struct Map {
    obstacles: Vec<Pos>,
    width: u32,
    height: u32,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("read file works");

    let obstacles = extract_obstacles(&input);
    let map = Map {
        obstacles,
        width: input.lines().next().unwrap().len() as u32,
        height: input.lines().count() as u32,
    };

    let start_pos = extract_start_pos(&input);

    println!("part1: {} ", get_path(&start_pos, &map).iter().count());
    println!("part2: {} ", get_loops_count(&start_pos, &map));
}

fn extract_start_pos(input: &str) -> Pos {
    input
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains('^'))
        .map(|(y, line)| Pos(line.find('^').unwrap() as i32, y as i32))
        .next()
        .unwrap()
}

fn extract_obstacles(input: &str) -> Vec<Pos> {
    let obstacles: Vec<Pos> = input
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains('#'))
        .flat_map(|(y, line)| {
            line.match_indices('#')
                .map(move |(x, _)| Pos(x as i32, y as i32))
        })
        .collect();
    obstacles
}

fn turn_right(direction: &Direction) -> Direction {
    Direction(-direction.1, direction.0)
}

fn get_path(start_pos: &Pos, map: &Map) -> Vec<Pos> {
    let mut path = vec![];
    let mut direction = Direction(0, -1);
    let mut pos = start_pos.clone();
    while (0..map.width).contains(&(pos.0 as u32)) && (0..map.height).contains(&(pos.1 as u32)) {
        path.push(pos.clone());
        let new_pos = Pos(pos.0 + direction.0 as i32, pos.1 + direction.1 as i32);
        if !map.obstacles.contains(&new_pos) {
            pos = new_pos;
        } else {
            direction = turn_right(&direction);
        }
    }
    path.into_iter().unique().collect::<Vec<Pos>>()
}

fn get_loops_count(start_pos: &Pos, map: &Map) -> u32 {
    let mut count = 0u32;
    let path = get_path(start_pos, map);
    for pos in path {
        if is_looping(
            start_pos,
            &Map {
                width: map.width,
                height: map.height,
                obstacles: [map.obstacles.clone(), vec![pos]].concat(),
            },
        ) {
            count += 1;
        }
    }
    count
}

fn is_looping(start_pos: &Pos, map: &Map) -> bool {
    let mut visited = vec![];
    let mut direction = Direction(0, -1);
    let mut pos = start_pos.clone();
    while (0..map.width).contains(&(pos.0 as u32)) && (0..map.height).contains(&(pos.1 as u32)) {
        if visited.contains(&(pos.clone(), direction.clone())) {
            return true;
        }
        visited.push((pos.clone(), direction.clone()));
        let new_pos = Pos(pos.0 + direction.0 as i32, pos.1 + direction.1 as i32);
        if !map.obstacles.contains(&new_pos) {
            pos = new_pos;
        } else {
            direction = turn_right(&direction);
        }
    }
    false
}
