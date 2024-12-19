use std::cmp::max;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn is_possible(patterns: &HashMap<u8, Vec<String>>, design: &str, cache: &mut HashMap<String, bool>, longest_pattern: usize, i: usize) -> bool {
    if design.len() == 0 {
        return true;
    }

    if let Some(&result) = cache.get(design) {
        println!("cache hit");
        return result; // Return cached result
    }

    for len in (1..=design.len()).rev() { // Iterate from longest to shortest
        let sub = &design[0..len];
        if let Some(&first_byte) = sub.as_bytes().first() {
            if let Some(valid_patterns) = patterns.get(&first_byte) {
                for pattern in valid_patterns {
                    if sub == pattern { // Exact match for substring
                        let remaining_design = &design[len..];
                        if is_possible(patterns, remaining_design, cache, longest_pattern, i) {
                            cache.insert(design.to_string(), true);
                            return true;
                        }
                    }
                }
            }
        }
    }
    cache.insert(design.to_string(), false);
    false
}

fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut patterns: HashMap<u8, Vec<String>> = HashMap::new();
    let mut possible = 0usize;
    let mut longest = 1usize;
    let mut count = 0usize;
    let mut cache: HashMap<String, bool> = HashMap::new();
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let p_split: Vec<String> = puzzle_line.split(',').map(|s| s.trim().to_string()).collect();
        if p_split.len() > 1 {
            for p in p_split {
                longest = max(longest, p.len());
                let key = p.as_bytes().first().copied().unwrap();
                patterns.entry(key).and_modify(|m| m.push(p.clone())).or_insert_with(|| vec![p]);
            }
        } else if puzzle_line.len() > 0 {
            if is_possible(&patterns, puzzle_line.as_str(), &mut cache, longest, count) {
                possible += 1;
            }
            count += 1;
            println!("current possible count: {possible} at {puzzle_line} and {patterns:?} ");
        }
    }
    println!("{patterns:?}");
    println!("{possible}");
}
