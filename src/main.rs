use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut reports = stdin.lock().lines();

    let mut safe_count = 0;
    'outer: while let Some(Ok(report)) = reports.next() {
        // let report_nums = report.split_whitespace().into_iter().map(|num| num.parse::<i32>().unwrap());
        println!("line");
        let mut maybe_asc = false;
        for (i, level) in report.split_whitespace().into_iter().enumerate().skip(1) {
            if let Some(prev_level) = report.split_whitespace().nth(i - 1) {
                if i <= 1 {
                    maybe_asc = prev_level < level;
                }
                println!("prev: {} cur: {} maybe_asc: {} gt: {}", prev_level, level, maybe_asc, prev_level >= level);
                if maybe_asc && prev_level > level {
                    println!("first ");
                    continue 'outer;
                } else if !maybe_asc && prev_level < level {
                    println!("second ");
                    continue 'outer;
                }
            };
        }
        safe_count += 1;
    }

    println!("safe_count: {}", safe_count);
}
