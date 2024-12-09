use std::fs;

const FREE_SPACE: &str = ".";

fn main() {
    let input = fs::read_to_string("input.txt").expect("file read works");
    let mut filesystem: Vec<Vec<String>> = input
        .char_indices()
        .map(|(pos, length)| (pos, (length.to_string()).parse::<usize>().unwrap()))
        .map(|(pos, length)| {
            if pos % 2 == 0 {
                vec![(pos / 2).to_string(); length]
            } else {
                vec![FREE_SPACE.to_string(); length]
            }
        })
        .collect();

    let mut blocks: Vec<String> = filesystem.clone().into_iter().flatten().collect();

    for (i, block) in blocks.clone().iter().enumerate() {
        if !blocks[i..].iter().any(|block| block != FREE_SPACE) {
            break;
        }
        if block == FREE_SPACE {
            let block_idx = blocks
                .iter()
                .enumerate()
                .filter(|(_, block)| *block != FREE_SPACE)
                .map(|(idx, _)| idx)
                .last()
                .unwrap();
            blocks.swap(i, block_idx);
        }
    }

    let result_1: usize = blocks
        .iter()
        .enumerate()
        .filter(|(_, block)| *block != FREE_SPACE)
        .map(|(idx, block)| idx * block.parse::<usize>().unwrap())
        .sum();
    println!("part1: {}", result_1);

    for file_id in (1..=9999).rev() {
        if let Some((file_idx, file)) = filesystem
            .clone()
            .into_iter()
            .enumerate()
            .find(|(_, block)| block.contains(&file_id.to_string()))
        {
            if let Some((free_idx, free_block)) = filesystem
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(_, block)| block.contains(&FREE_SPACE.to_string()))
                .filter(|(idx, _)| idx < &file_idx)
                .find(|(_, block)| block.len() >= file.len())
            {
                if free_block.len() == file.len() {
                    filesystem.swap(file_idx, free_idx);
                } else {
                    filesystem[free_idx] = file.clone();
                    filesystem[file_idx] = vec![FREE_SPACE.to_string(); file.len()];
                    filesystem.insert(
                        free_idx + 1,
                        vec![FREE_SPACE.to_string(); free_block.len() - file.len()],
                    );
                }
            }
        }
    }

    let result_2: usize = filesystem
        .into_iter()
        .flatten()
        .enumerate()
        .filter(|(_, block)| *block != FREE_SPACE)
        .map(|(idx, block)| idx * block.parse::<usize>().unwrap())
        .sum();
    println!("part2: {}", result_2);
}
