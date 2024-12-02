use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut reports = stdin.lock().lines();
    let mut safe_count = 0;
    while let Some(Ok(raw_report)) = reports.next() {
        let parsed_report = raw_report.split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        'middle: for i in 0..parsed_report.len() {
            let mut report = parsed_report.clone();
            report.remove(i);
            let is_asc = report[0] < report[1];
            for level_window in report.windows(2) {
                let (level, next_level) = (level_window[0], level_window[1]);
                // println!("prev: {} cur: {} maybe_asc: {} gt: {}", level, next_level, maybe_asc, level >= next_level);
                if level.abs_diff(next_level) > 3 ||
                    (is_asc && (level >= next_level)) ||
                    (!is_asc && (level <= next_level)) {
                    continue 'middle;
                }
            }
            safe_count += 1;
            break;
        }
    }
    println!("safe_count: {}", safe_count);
}
