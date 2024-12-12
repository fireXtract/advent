use std::io;
use std::io::BufRead;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct File {
    id: usize,
    idx: usize,
    length: usize,
}

const PADDING_ID: usize = usize::MAX;

fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();

    let mut files: Vec<File> = Vec::new();
    let mut id = 0usize;
    let mut idx = 0usize;
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let mut is_file = true;
        for c in puzzle_line.chars() {
            // print!("{c}");
            let length = c.to_digit(10).unwrap() as usize;
            let f_id;
            if is_file {
                f_id = id;
                id += 1;
            } else {
                f_id = PADDING_ID;
            }
            files.push(File { id: f_id, idx, length });
            idx += length;
            is_file = !is_file;
        }
    }
    println!("A");

    let mut considering_id = id - 1;
    while considering_id > 0 {
        let mut left_idx = 0usize;
        let mut right_idx = files.len() - 1;
        while left_idx < right_idx {
            let left_file = &files[left_idx];
            if left_file.id == PADDING_ID {
                let right_file = &files[right_idx];
                if right_file.id == considering_id {
                    let padding_width = left_file.length;
                    let block_width = right_file.length;
                    if padding_width >= block_width {
                        let leftover_padding = padding_width - block_width;
                        let original_padding_idx = left_file.idx;
                        let original_block_idx = right_file.idx;
                        files.swap(left_idx, right_idx);
                        files[left_idx].idx = original_padding_idx;
                        files[right_idx].idx = original_block_idx;
                        if leftover_padding > 0 {
                            files[right_idx].length -= leftover_padding;
                            files.insert(left_idx + 1, File { id: PADDING_ID, idx: files[left_idx].idx + block_width, length: leftover_padding });
                        }
                        break;
                    } else {
                        left_idx += 1;
                    }
                } else {
                    right_idx -= 1;
                }
            } else {
                left_idx += 1;
            }
        }
        considering_id -= 1;
    }

    // println!();
    let mut checksum = 0usize;
    for file in files {
        for i in 0..file.length {
            if file.id != PADDING_ID {
                checksum += file.id * (file.idx + i);
            }
        }
    }

    println!("checksum {checksum}");
}


