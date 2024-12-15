use itertools::Itertools;
use regex::Regex;
use std::fs;

const WIDTH: i64 = 101;
const WIDTH_MID: i64 = 50;
const HEIGHT: i64 = 103;
const HEIGHT_MID: i64 = 51;

#[derive(Debug)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("works");
    let mut robots = extract_robots(&input);
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;
    let mut result_part2 = 0;
    for i in 0..10000 {
        robots = robots.iter().map(move_robot).collect();
        if i == 99 {
            top_left = robots
                .iter()
                .filter(|robot| robot.pos.0 < WIDTH_MID && robot.pos.1 < HEIGHT_MID)
                .count();
            top_right = robots
                .iter()
                .filter(|robot| robot.pos.0 > WIDTH_MID && robot.pos.1 < HEIGHT_MID)
                .count();
            bottom_left = robots
                .iter()
                .filter(|robot| robot.pos.0 < WIDTH_MID && robot.pos.1 > HEIGHT_MID)
                .count();
            bottom_right = robots
                .iter()
                .filter(|robot| robot.pos.0 > WIDTH_MID && robot.pos.1 > HEIGHT_MID)
                .count();
        }
        if robots
            .iter()
            .counts_by(|robot| robot.pos.1)
            .into_iter()
            .any(|(_, size)| size >= 31)
            && contains_easter_egg(&robots)
        {
            result_part2 = i + 1;
            break;
        }
    }

    println!(
        "part1: {}",
        top_left * top_right * bottom_left * bottom_right
    );
    println!("part2: {}", result_part2);
}

fn contains_easter_egg(robots: &Vec<Robot>) -> bool {
    let mut map: String = String::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if robots
                .iter()
                .any(|robot| robot.pos.0 == x && robot.pos.1 == y)
            {
                map += "#";
            } else {
                map += ".";
            }
        }
        map += "\n";
    }
    map.contains("###############################")
}

fn move_robot(robot: &Robot) -> Robot {
    let mut pos_x = robot.pos.0 + robot.vel.0;
    if pos_x < 0 {
        pos_x = WIDTH + pos_x;
    } else if pos_x >= WIDTH {
        pos_x = pos_x - WIDTH;
    }
    let mut pos_y = robot.pos.1 + robot.vel.1;
    if pos_y < 0 {
        pos_y = HEIGHT + pos_y;
    } else if pos_y >= HEIGHT {
        pos_y = pos_y - HEIGHT;
    }
    Robot {
        pos: (pos_x, pos_y),
        vel: robot.vel,
    }
}

fn extract_robots(input: &str) -> Vec<Robot> {
    input.lines().map(extract_robot).collect()
}

fn extract_robot(line: &str) -> Robot {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let caps = regex.captures(line).unwrap();

    let pos = (
        caps.get(1)
            .map_or(0, |val| val.as_str().parse::<i64>().unwrap()),
        caps.get(2)
            .map_or(0, |val| val.as_str().parse::<i64>().unwrap()),
    );
    let vel = (
        caps.get(3)
            .map_or(0, |val| val.as_str().parse::<i64>().unwrap()),
        caps.get(4)
            .map_or(0, |val| val.as_str().parse::<i64>().unwrap()),
    );
    Robot { pos, vel }
}
