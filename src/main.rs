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
        let rule: Vec<i32> = puzzle_line.split('|')
            .flat_map(|r| r.parse::<i32>().ok()).collect();
        if rule.len() > 0 {
            before.entry(rule[0]).or_insert(Vec::new()).push(rule[1]);
        } else {
            let mut page_nums: Vec<i32> = puzzle_line.split(',').flat_map(|r| r.parse::<i32>().ok()).collect();
            let valid: bool = page_nums.iter()
                .all(|page_num|
                    before.get(page_num)
                        .map_or(true, |before_vec|
                            !page_nums.iter().take_while(|&x| *x != *page_num)
                                .any(|left_page| before_vec.iter().any(|&b| b == *left_page)),
                        )
                );
            if !valid {
                page_nums.sort_by(|a, b| {
                    if before.get(a).map(|a_before| a_before.contains(b)).unwrap_or_default() {
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
