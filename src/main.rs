use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
static EMPTY_VEC: &Vec<i32> = &Vec::new();
fn main() {
    let stdin = io::stdin();
    let mut puzzle_lines = stdin.lock().lines();
    let mut middle_sum = 0;
    let mut fixed_invalid_middle_sum = 0;
    // let mut puzzle: Vec<i32> = Vec::new();
    let mut before: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut after: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut jk = 0;
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let rule: Vec<i32> = puzzle_line.split('|').flat_map(|r| {
            // print!("{r} ");
            r.parse::<i32>().ok()
        }).collect();
        if rule.len() > 0 {
            before.entry(rule[0]).or_insert(Vec::new()).push(rule[1]);
            after.entry(rule[1]).or_insert(Vec::new()).push(rule[0]);
        } else {
            println!("{before:?} {after:?}");
            println!("on {jk}");
            let mut page_nums: Vec<i32> = puzzle_line.split(',').flat_map(|r| r.parse::<i32>().ok()).collect();
            let mut working_page: Vec<i32> = page_nums.clone();
            let valid: bool = page_nums.iter().enumerate()
                .all(|(index, pnum)| {
                    let mut p = page_nums.split(|x| *x == *pnum);
                    let left = p.next().unwrap_or_default().to_vec();
                    let right = p.next().unwrap_or_default().to_vec();
                    // given v, any element in before[v] must be after v, so not in left
                    // given v, any element in after[v] must before v, so not in right
                    let is_left_invalid = left.iter().any(|l| before.get(pnum).iter().any(|&b| (*b).contains(l)));
                    let is_right_invalid = right.iter().any(|r| after.get(pnum).iter().any(|&a| (*a).contains(r)));
                    if is_left_invalid { print!("!! invalid left - "); }
                    if is_right_invalid { print!("!! invalid right - "); }
                    let pair = (left, right);
                    println!("{pnum:?} against {pair:?} {is_left_invalid} {is_right_invalid}");

                    !(is_left_invalid || is_right_invalid)
                });
            if !valid {
                println!("not valid {working_page:?}");
                page_nums.sort_by(|&a, &b| {
                    let a_before = before.get(&a).unwrap_or(EMPTY_VEC);

                    // Check if `b` is in `before[a]` (i.e., a should come before b)
                    if a_before.contains(&b) {
                        return Ordering::Less;
                    }

                    Ordering::Equal
                });
                println!("fixed it {working_page:?}");
                fixed_invalid_middle_sum += page_nums.get(page_nums.len() / 2).unwrap_or(&0)
            } else {
                middle_sum += page_nums.get(page_nums.len() / 2).unwrap_or(&0)
            }
            jk += 1;

            println!("{valid:?}");
        }
    }

    println!("middle_sum: {}", middle_sum);
    println!("fixed_invalid_middle_sum: {}", fixed_invalid_middle_sum);
}



