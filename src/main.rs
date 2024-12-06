use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
fn main() {
    let stdin = io::stdin();
    let mut puzzle_lines = stdin.lock().lines();
    let mut middle_sum = 0;
    let mut fixed_invalid_middle_sum = 0;
    let mut before: HashMap<i32, Vec<i32>> = HashMap::new();
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let rule: Vec<i32> = puzzle_line
            .split('|')
            .flat_map(|r| r.parse::<i32>().ok())
            .collect();
        if rule.len() > 0 {
            before.entry(rule[0]).or_insert(Vec::new()).push(rule[1]);
        } else {
            let mut page_nums: Vec<i32> = puzzle_line.split(',').flat_map(|r| r.parse::<i32>().ok()).collect();
            let valid: bool = page_nums.iter()
                .all(|page_num| {
                    let page_num_before = before.get(page_num);
                    let mut p = page_nums.split(|x| *x == *page_num);
                    let left = p.next().unwrap();
                    !left.iter().any(|l| page_num_before.iter().any(|&b| (*b).contains(l)))
                });
            if !valid {
                page_nums.sort_by(|&a, &b| {
                    let a_before = before.get(&a).unwrap();
                    // Check if `b` is in `before[a]` (i.e., a should come before b)
                    if a_before.contains(&b) {
                        return Ordering::Less;
                    }
                    Ordering::Equal
                });
                fixed_invalid_middle_sum += page_nums.get(page_nums.len() / 2).unwrap_or(&0)
            } else {
                middle_sum += page_nums.get(page_nums.len() / 2).unwrap_or(&0)
            }
        }
    }

    println!("middle_sum: {}", middle_sum);
    println!("fixed_invalid_middle_sum: {}", fixed_invalid_middle_sum);
}
