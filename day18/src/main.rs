use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
};

type Pos = (u16, u16);

const DIRS: [(i16, i16); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    let input = fs::read_to_string("input.txt").expect("works");
    let bytes = extract_bytes(&input);
    let part1 = find_shortest_path(71, 71, &bytes[..1024]);
    let (part2_x, part2_y) = find_blocking(71, 71, &bytes);
    println!("part1: {:?}", part1);
    println!("part2: {},{}", part2_x, part2_y);
}

fn find_blocking(width: u16, height: u16, bytes: &Vec<Pos>) -> Pos {
    for (i, _) in bytes.iter().enumerate().skip(1024) {
        let result = find_shortest_path(width, height, &bytes[..i]);
        if result == 0 {
            return bytes[i - 1];
        }
    }
    (0, 0)
}

fn find_shortest_path(width: u16, height: u16, corrupted: &[Pos]) -> u16 {
    let mut dsts: HashMap<Pos, u16> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(u16, Pos)>> = BinaryHeap::new();

    for x in 0..width {
        for y in 0..height {
            dsts.insert((x, y), u16::MAX);
        }
    }
    dsts.insert((0, 0), 0);
    queue.push(Reverse((0, (0, 0))));
    while let Some(Reverse((dst, pos))) = queue.pop() {
        if pos == (width - 1, height - 1) {
            return dst;
        }
        for dir in DIRS {
            let x = pos.0 as i16 + dir.0;
            let y = pos.1 as i16 + dir.1;

            if x < 0 || x >= width as i16 || y < 0 || y >= height as i16 {
                continue;
            }

            let new_pos = (x as u16, y as u16);
            if corrupted.contains(&new_pos) {
                continue;
            }
            let new_dst = dst + 1;
            if new_dst < *dsts.get(&new_pos).unwrap() {
                dsts.insert(new_pos, new_dst);
                queue.push(Reverse((new_dst, new_pos)));
            }
        }
    }
    0
}

fn extract_bytes(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .map(|values| {
            (
                values[0].parse::<u16>().unwrap(),
                values[1].parse::<u16>().unwrap(),
            )
        })
        .collect()
}
