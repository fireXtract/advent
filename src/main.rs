use std::io;
use std::io::BufRead;

const BLINKS: u8 = 75;

fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut stones: Vec<u64> = vec![];
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        stones = puzzle_line.split(' ').map(|s| s.parse::<u64>().unwrap()).collect()
    }
    for s in stones.clone() {
        print!("{s} ")
    }

    for blink in 0..BLINKS {
        let mut i = 0usize;
        println!("iteration {blink}");
        while i < stones.len() {
            let s = stones[i];
            match s {
                _ if s == 0 => stones[i] = 1,
                v if count_digits(v) % 2 == 0 => {
                    let split_digits = split_number(v);
                    stones[i] = split_digits.0;
                    stones.insert(i + 1, split_digits.1);
                    i += 1;
                }
                _ => stones[i] = s * 2024
            }
            i += 1;
        }
    }

    println!("{:?}\nlen:{}", stones, stones.len());
}

fn count_digits(i: u64) -> u64 {
    let mut count = 1;
    let mut n = i;
    while n / 10 > 0 {
        count += 1;
        n /= 10;
    }
    count
}
fn split_number(i: u64) -> (u64, u64) {
    let digit_count = count_digits(i);
    let half_digits = digit_count / 2;

    // Calculate the divisor to separate the halves
    let divisor = 10u64.pow(half_digits as u32);

    let left_half = i / divisor;
    let right_half = i % divisor;

    (left_half, right_half)
}
