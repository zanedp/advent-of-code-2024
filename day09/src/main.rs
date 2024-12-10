use std::collections::VecDeque;

const EXAMPLE_INPUT: &str = r#"2333133121414131402"#;

fn main() {
    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt").trim();
    let disk_description = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .map(|(file_id, chunk)| {
            let (length, blanks_after) = (chunk[0], chunk.get(1).unwrap_or(&0));
            (file_id, length, *blanks_after)
        })
        .collect::<Vec<_>>();

    let mut disk_layout = VecDeque::new();
    for (file_id, length, blanks_after) in disk_description.iter() {
        for _ in 0..*length {
            disk_layout.push_back(Some(*file_id));
        }
        for _ in 0..*blanks_after {
            disk_layout.push_back(None);
        }
    }

    let mut original_disk_layout = disk_layout.clone();
    let mut new_disk_layout: Vec<usize> = Vec::with_capacity(disk_layout.len());

    'copy: loop {
        let cur_sector = original_disk_layout.pop_front();
        if cur_sector.is_none() {
            // we have consumed every sector in the original disk layout
            break 'copy;
        }
        if let Some(cur_file_id) = cur_sector.unwrap() {
            new_disk_layout.push(cur_file_id);
        } else {
            // we have a blank sector
            // so let's get a sector from the end of the disk to fill it.
            let file_chunk_to_move: usize;
            'search: loop {
                let last_sector = original_disk_layout.pop_back();
                if last_sector.is_none() {
                    // there is nothing left in the original disk layout to move
                    break 'copy;
                }
                if let Some(file_id) = last_sector.unwrap() {
                    file_chunk_to_move = file_id;
                    break 'search;
                }
            }
            new_disk_layout.push(file_chunk_to_move);
        }
    }
    println!(
        "{:?}",
        new_disk_layout
            .iter()
            .enumerate()
            .map(|(i, x)| i * x)
            .sum::<usize>()
    );
}
