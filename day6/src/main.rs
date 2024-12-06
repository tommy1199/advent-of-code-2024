use std::fs;

use itertools::Itertools;

#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Eq, Clone, Hash, PartialEq)]
struct Pos(u32, u32);

struct Map {
    obstactles: Vec<Pos>,
    width: u32,
    height: u32,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("read file works");

    let obstacles: Vec<Pos> = input
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains('#'))
        .flat_map(|(y, line)| {
            line.match_indices('#')
                .map(move |(x, _)| Pos(x as u32, y as u32))
        })
        .collect();
    let map = Map {
        obstactles: obstacles,
        width: input.lines().next().unwrap().len() as u32,
        height: input.lines().count() as u32,
    };

    let guard_pos = input
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains('^'))
        .map(|(y, line)| Pos(line.find('^').unwrap() as u32, y as u32))
        .next()
        .unwrap();

    println!(
        "part1: {} ",
        calculate_path(&guard_pos, &Direction::UP, &map)
            .into_iter()
            .unique()
            .collect::<Vec<_>>()
            .iter()
            .count()
    );
    println!("part2: {} ", find_loops(guard_pos, Direction::UP, map));
}

fn find_loops(guard_pos: Pos, direction: Direction, map: Map) -> u32 {
    let mut result = 0u32;
    let path = calculate_path(&guard_pos, &direction, &map)
        .into_iter()
        .unique()
        .collect::<Vec<Pos>>();
    for pos in path {
        if is_loop(
            &guard_pos,
            &direction,
            Map {
                obstactles: [map.obstactles.clone(), vec![pos.clone()]].concat(),
                width: map.width,
                height: map.height,
            },
            vec![],
        ) {
            result += 1;
        }
    }
    result
}

fn is_loop<'a>(
    guard_pos: &'a Pos,
    direction: &'a Direction,
    map: Map,
    mut positions: Vec<(&'a Pos, &'a Direction)>,
) -> bool {
    if (guard_pos.0 == 0 && direction == &Direction::LEFT)
        || (guard_pos.0 == map.width - 1 && direction == &Direction::RIGHT)
        || (guard_pos.1 == 0 && direction == &Direction::UP)
        || (guard_pos.1 == map.height - 1 && direction == &Direction::DOWN)
    {
        return false;
    }
    if positions.contains(&(guard_pos, direction)) {
        return true;
    }
    positions.push((guard_pos, direction));
    match direction {
        Direction::UP => {
            let new_pos = Pos(guard_pos.0, guard_pos.1 - 1);
            if map.obstactles.contains(&new_pos) {
                is_loop(guard_pos, &Direction::RIGHT, map, positions)
            } else {
                is_loop(&new_pos, direction, map, positions)
            }
        }
        Direction::DOWN => {
            let new_pos = Pos(guard_pos.0, guard_pos.1 + 1);
            if map.obstactles.contains(&new_pos) {
                is_loop(guard_pos, &Direction::LEFT, map, positions)
            } else {
                is_loop(&new_pos, direction, map, positions)
            }
        }
        Direction::LEFT => {
            let new_pos = Pos(guard_pos.0 - 1, guard_pos.1);
            if map.obstactles.contains(&new_pos) {
                is_loop(guard_pos, &Direction::UP, map, positions)
            } else {
                is_loop(&new_pos, direction, map, positions)
            }
        }
        Direction::RIGHT => {
            let new_pos = Pos(guard_pos.0 + 1, guard_pos.1);
            if map.obstactles.contains(&new_pos) {
                is_loop(guard_pos, &Direction::DOWN, map, positions)
            } else {
                is_loop(&new_pos, direction, map, positions)
            }
        }
    }
}

fn calculate_path(guard_pos: &Pos, direction: &Direction, map: &Map) -> Vec<Pos> {
    let path = vec![guard_pos.clone()];
    match direction {
        Direction::UP => {
            if guard_pos.1 == 0 {
                return path;
            }
            let new_pos = Pos(guard_pos.0, guard_pos.1 - 1);
            if map.obstactles.contains(&new_pos) {
                return calculate_path(guard_pos, &Direction::RIGHT, map);
            } else {
                return [path, calculate_path(&new_pos, direction, map)].concat();
            }
        }
        Direction::DOWN => {
            if guard_pos.1 == map.height - 1 {
                return path;
            }
            let new_pos = Pos(guard_pos.0, guard_pos.1 + 1);
            if map.obstactles.contains(&new_pos) {
                return calculate_path(guard_pos, &Direction::LEFT, map);
            } else {
                return [path, calculate_path(&new_pos, direction, map)].concat();
            }
        }
        Direction::LEFT => {
            if guard_pos.0 == 0 {
                return path;
            }
            let new_pos = Pos(guard_pos.0 - 1, guard_pos.1);
            if map.obstactles.contains(&new_pos) {
                return calculate_path(guard_pos, &Direction::UP, map);
            } else {
                return [path, calculate_path(&new_pos, direction, map)].concat();
            }
        }
        Direction::RIGHT => {
            if guard_pos.0 == map.width - 1 {
                return path;
            }
            let new_pos = Pos(guard_pos.0 + 1, guard_pos.1);
            if map.obstactles.contains(&new_pos) {
                return calculate_path(guard_pos, &Direction::DOWN, map);
            } else {
                return [path, calculate_path(&new_pos, direction, map)].concat();
            }
        }
    }
}
