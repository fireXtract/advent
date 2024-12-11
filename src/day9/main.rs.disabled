use std::io;
use std::io::BufRead;

fn main() {
    const RADIX: u32 = 10;
    let stdin = io::stdin();
    let mut puzzle_lines = stdin.lock().lines();
    let mut id: isize = 0isize;
    let mut filesystem: Vec<isize> = Vec::new();
    while let Some(Ok(disk_map)) = puzzle_lines.next() {
        let mut is_file = true;
        for x in disk_map.chars() {
            for _ in 0..x.to_digit(RADIX).unwrap() {
                if is_file {
                    filesystem.push(id);
                } else {
                    filesystem.push(-1);
                }
            }
            if is_file {
                id += 1;
            }
            is_file = !is_file;
        }
    }

    let og_filesystem = filesystem.clone();
    for c in og_filesystem {
        if c == -1 {
            print!(".");
        } else {
            if c > 9 {
                print!("({c})");
            } else {
                print!("{c}");
            }
        }
    }
    println!();

    let mut left_idx = 0usize;
    let mut right_idx = filesystem.len() - 1;
    let mut seen_ids: Vec<isize> = Vec::new();
    while seen_ids.len() <= id as usize + 1  {
        print!("({left_idx}<{right_idx})");
        let mut left_val = filesystem[left_idx];
        if left_val == -1 {
            let right_val = filesystem[right_idx];
            if right_val != -1 {
                print!("(c{right_val})");
                if !seen_ids.contains(&right_val) {
                    // traverse all padding for this current block
                    let block_width = match_length(&filesystem, right_idx, right_val, -1);
                    println!("block width of {right_val} was {block_width}");
                    while left_idx < right_idx {
                        left_val = filesystem[left_idx];
                        if left_val == -1 {
                            let padding_width = match_length(&filesystem, left_idx, -1, 1);
                            if padding_width >= block_width {
                                println!("c{right_val} fits at {left_idx} bc {padding_width}>{block_width}");
                                for j in 0..block_width {
                                    println!("{j} swapping");
                                    filesystem.swap(left_idx + j, right_idx - j);
                                }
                                left_idx = 0;
                                right_idx -= block_width;
                                seen_ids.push(right_val);
                            } else {
                                println!("c{right_val} padding at {left_idx} didn't fit");
                                left_idx += 1
                            }
                        } else {
                            left_idx += 1;
                        }
                    }
                    if !seen_ids.contains(&right_val) {seen_ids.push(right_val);}
                    left_idx = 0;
                    right_idx = filesystem.len()-1;
                } else {
                    right_idx -= 1;
                }
            } else {
                right_idx -= 1;
            }
        } else {
            left_idx += 1;
        }
    }
    println!("swapping done");
    let swapped = filesystem.clone();
    for c in swapped {
        if c == -1 {
            print!(".");
        } else {
            if c > 9 {
                print!("({c})");
            } else {
                print!("{c}");
            }
        }
    }
    println!();
    let distinct_pos = filesystem.iter().enumerate()
        .filter(|(_, &i)| i >= 0)
        .fold(0, |acc, (idx, &val)|
            acc + (idx * val as usize),
        );

    // println!("{filesystem:?}");


    println!("distinct_pos: {}", distinct_pos);
}

fn match_length(vec: &Vec<isize>, start_pos: usize, match_value: isize, direction: isize) -> usize {
    let mut current_pos = start_pos as isize;
    let mut match_len = 0;

    while 0 <= current_pos && current_pos < vec.len() as isize && vec[current_pos as usize] == match_value {
        match_len += 1;
        current_pos += direction;
    }

    match_len
}

