use itertools::Itertools;
use std::fs;

#[derive(Debug)]
struct Warehouse {
    robot: (i64, i64),
    boxes: Vec<(i64, i64)>,
    walls: Vec<(i64, i64)>,
    wide_boxes: Vec<[(i64, i64); 2]>,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("works");
    let (mut warehouse, mut movements) = extract(&input);
    move_robot(&mut warehouse, &movements);

    println!(
        "part1: {}",
        warehouse
            .boxes
            .iter()
            .map(|(x, y)| y * 100 + x)
            .sum::<i64>()
    );

    (warehouse, movements) = extract_extended(&input);
    move_robot(&mut warehouse, &movements);
    println!(
        "part2: {}",
        warehouse
            .wide_boxes
            .iter()
            .map(|w_box| w_box[0].1 * 100 + w_box[0].0)
            .sum::<i64>()
    );
}

fn move_robot(warehouse: &mut Warehouse, movements: &String) {
    for c in movements.chars() {
        let dir = get_direction(&c);
        if !is_wall_in_path(&warehouse, &dir, &warehouse.robot) {
            let boxes_to_move = find_boxes_in_path(&warehouse, &dir, &warehouse.robot);
            warehouse
                .boxes
                .retain(|w_box| !boxes_to_move.contains(w_box));
            warehouse.boxes.extend(
                boxes_to_move
                    .iter()
                    .map(|w_box| (w_box.0 + dir.0, w_box.1 + dir.1)),
            );
            let wide_boxes_to_move = find_wide_boxes_in_path(&warehouse, &dir, &warehouse.robot);
            warehouse
                .wide_boxes
                .retain(|w_box| !wide_boxes_to_move.contains(w_box));
            warehouse
                .wide_boxes
                .extend(wide_boxes_to_move.iter().map(|w_box| {
                    [
                        (w_box[0].0 + dir.0, w_box[0].1 + dir.1),
                        (w_box[1].0 + dir.0, w_box[1].1 + dir.1),
                    ]
                }));
            warehouse.robot = (warehouse.robot.0 + dir.0, warehouse.robot.1 + dir.1);
        }
    }
}

fn is_wall_in_path(warehouse: &Warehouse, dir: &(i64, i64), pos: &(i64, i64)) -> bool {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if warehouse.walls.contains(&new_pos) {
        true
    } else if warehouse
        .wide_boxes
        .iter()
        .any(|w_box| w_box[0] == new_pos || w_box[1] == new_pos)
    {
        let wide_box = warehouse
            .wide_boxes
            .iter()
            .find(|w_box| w_box[0] == new_pos || w_box[1] == new_pos)
            .unwrap();
        if *dir == (1, 0) {
            is_wall_in_path(warehouse, dir, &wide_box[1])
        } else if *dir == (-1, 0) {
            is_wall_in_path(warehouse, dir, &wide_box[0])
        } else {
            is_wall_in_path(warehouse, dir, &wide_box[0])
                || is_wall_in_path(warehouse, dir, &wide_box[1])
        }
    } else if warehouse.boxes.contains(&new_pos) {
        is_wall_in_path(warehouse, dir, &new_pos)
    } else {
        false
    }
}

fn find_wide_boxes_in_path(
    warehouse: &Warehouse,
    dir: &(i64, i64),
    pos: &(i64, i64),
) -> Vec<[(i64, i64); 2]> {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if warehouse
        .wide_boxes
        .iter()
        .any(|w_box| w_box[0] == new_pos || w_box[1] == new_pos)
    {
        let wide_box = warehouse
            .wide_boxes
            .iter()
            .find(|w_box| w_box[0] == new_pos || w_box[1] == new_pos)
            .unwrap();
        let mut wide_boxes: Vec<[(i64, i64); 2]> = vec![*wide_box];
        if *dir == (1, 0) {
            wide_boxes.extend(find_wide_boxes_in_path(&warehouse, &dir, &wide_box[1]));
        } else if *dir == (-1, 0) {
            wide_boxes.extend(find_wide_boxes_in_path(&warehouse, &dir, &wide_box[0]));
        } else {
            wide_boxes.extend(find_wide_boxes_in_path(&warehouse, &dir, &wide_box[0]));
            wide_boxes.extend(find_wide_boxes_in_path(&warehouse, &dir, &wide_box[1]));
        }

        wide_boxes.into_iter().unique().collect()
    } else {
        vec![]
    }
}

fn find_boxes_in_path(
    warehouse: &Warehouse,
    dir: &(i64, i64),
    pos: &(i64, i64),
) -> Vec<(i64, i64)> {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if warehouse.boxes.contains(&new_pos) {
        let mut boxes = vec![new_pos];
        boxes.extend(find_boxes_in_path(&warehouse, &dir, &new_pos));
        boxes
    } else {
        vec![]
    }
}

fn get_direction(c: &char) -> (i64, i64) {
    if *c == '^' {
        (0, -1)
    } else if *c == '>' {
        (1, 0)
    } else if *c == 'v' {
        (0, 1)
    } else {
        (-1, 0)
    }
}

fn extract(input: &str) -> (Warehouse, String) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let warehouse = extract_warehouse(parts[0]);
    (warehouse, parts[1].replace("\n", "").to_string())
}

fn extract_extended(input: &str) -> (Warehouse, String) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let extended_map: String = parts[0]
        .chars()
        .flat_map(|c| {
            if c == '#' {
                vec!['#', '#']
            } else if c == '@' {
                vec!['@', '.']
            } else if c == 'O' {
                vec!['[', ']']
            } else if c == '\n' {
                vec!['\n']
            } else {
                vec!['.', '.']
            }
        })
        .collect();
    let warehouse = extract_warehouse(&extended_map);
    (warehouse, parts[1].replace("\n", "").to_string())
}

fn extract_warehouse(input: &str) -> Warehouse {
    let mut boxes: Vec<(i64, i64)> = Vec::new();
    let mut walls: Vec<(i64, i64)> = Vec::new();
    let mut robot: (i64, i64) = (0, 0);
    let mut wide_boxes: Vec<[(i64, i64); 2]> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                walls.push((x as i64, y as i64));
            } else if c == 'O' {
                boxes.push((x as i64, y as i64));
            } else if c == '@' {
                robot = (x as i64, y as i64);
            } else if c == '[' {
                wide_boxes.push([(x as i64, y as i64), (x as i64 + 1, y as i64)]);
            }
        }
    }
    Warehouse {
        robot,
        walls,
        boxes,
        wide_boxes,
    }
}
