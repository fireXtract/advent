use std::cmp::max;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn ways_possible(design: &str, patterns: &HashMap<u8, Vec<String>>, cache: &mut HashMap<String, usize>) -> usize {
    if design.len() == 0 {
        return 1;
    }
    if let Some(&result) = cache.get(design) {
        return result;
    }
    let mut ways = 0;
    for len in (1..=design.len()).rev() {
        let sub = &design[0..len];
        if let Some(first_byte) = sub.as_bytes().first() {
            if let Some(valid_patterns) = patterns.get(first_byte) {
                for pattern in valid_patterns {
                    if sub == pattern {
                        let remaining_design = &design[len..];
                        ways += ways_possible(remaining_design, patterns, cache);
                    }
                }
            }
        }
    }
    cache.insert(design.to_string(), ways);
    ways
}

fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut patterns: HashMap<u8, Vec<String>> = HashMap::new();
    let mut total_ways = 0usize;
    let mut longest = 1usize;
    let mut cache: HashMap<String, usize> = HashMap::new();
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let p_split: Vec<String> = puzzle_line.split(',').map(|s| s.trim().to_string()).collect();
        if p_split.len() > 1 {
            for p in p_split {
                longest = max(longest, p.len());
                let key = p.as_bytes().first().copied().unwrap();
                patterns.entry(key).and_modify(|m| m.push(p.clone())).or_insert_with(|| vec![p]);
            }
        } else if puzzle_line.len() > 0 {
            total_ways += ways_possible(puzzle_line.as_str(), &patterns, &mut cache);
        }
    }
    println!("{total_ways}");
}
