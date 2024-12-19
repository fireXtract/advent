use std::collections::HashMap;
use std::io::BufRead;
use std::io;

fn is_possible(patterns: &Vec<String>, design: &str) -> bool {
    if design.len() == 0 {
        return true;
    }
    for pattern in patterns {
        if design.starts_with(pattern) {
            // println!("{design} started with {pattern}");
            let l = pattern.len();
            if is_possible(patterns, &design[l..]) {
                return true;
            }
        }
    }

    false
}

fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut patterns: Vec<String> = vec![];
    let mut patterns_map: HashMap<u8, String> = HashMap::new();
    let mut possible = 0usize;
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let p_split: Vec<String> = puzzle_line.split(',').map(|s| s.trim().to_string()).collect();
        if p_split.len() > 1 {
            for p in p_split {
                patterns.push(p);
            }
        } else if puzzle_line.len() > 0{
            if is_possible(&patterns, puzzle_line.as_str()) {
                possible += 1;
            }
        }
    }
    println!("{patterns:?}");
    println!("{possible}");
}
