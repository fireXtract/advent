use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut puzzle_lines = stdin.lock().lines();
    let mut middle_sum = 0;
    // let mut puzzle: Vec<i32> = Vec::new();
    let mut before: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut after: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut jk = 0;
    let mut valids: Vec<Vec<&i32>> = Vec::new();
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let rule: Vec<i32> = puzzle_line.split('|').flat_map(|r| {
            // print!("{r} ");
            r.parse::<i32>().ok()
        }).collect();
        if rule.len() > 0 {
            before.entry(rule[0]).or_insert(Vec::new()).push(rule[1]);
            after.entry(rule[1]).or_insert(Vec::new()).push(rule[0]);
        } else {
            println!("on {jk}");
            let pages: Vec<i32> = puzzle_line.split(',').flat_map(|r| r.parse::<i32>().ok()).collect();
            let valid: bool = pages.iter()
                .all(|v| {
                    let mut p = pages.split(|x| *x == *v);
                    let left = p.next().unwrap_or_default().to_vec();
                    let right = p.next().unwrap_or_default().to_vec();
                    // given v, all elements in before[v] must be after v, so not in left
                    // given v, all elements in after[v] must before v, so not in right
                    let is_left_invalid = left.iter().any(|l| before.get(v).iter().any(|&b| (*b).contains(l)));
                    let is_right_invalid = right.iter().any(|r| after.get(v).iter().any(|&a| (*a).contains(r)));
                    if is_left_invalid { print!("!! invalid left"); }
                    if is_right_invalid { print!("!! invalid right"); }
                    let pair = (left, right);
                    println!("{v:?} against {pair:?} {is_left_invalid} {is_right_invalid}");

                    !(is_left_invalid || is_right_invalid)
                });
            jk += 1;
            middle_sum += if valid { pages.get(pages.len() / 2).unwrap_or(&0) } else { &0 };
            println!("{valid:?}");
        }
    }

    println!("middle_sum: {}", middle_sum);
}



