use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut reports = stdin.lock().lines();

    let mut safe_count = 0;
    'outer: while let Some(Ok(report)) = reports.next() {
        let report_nums = report.split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        // println!("line");
        let mut maybe_asc = false;
        let mut first_run = true;
        for window in report_nums.windows(2) {
            let level = window[0];
            let next_level = window[1];
            if first_run {
                first_run = false;
                maybe_asc = level < next_level;
            }
            // println!("prev: {} cur: {} maybe_asc: {} gt: {}", level, next_level, maybe_asc, level >= next_level);
            if level.abs_diff(next_level) > 3 ||
                (maybe_asc && level >= next_level) ||
                (!maybe_asc && level <= next_level) {
                println!("unsafe");
                continue 'outer;
            }
        }
        println!("safe");
        safe_count += 1;
    }

    println!("safe_count: {}", safe_count);
}
