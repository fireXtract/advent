use std::collections::HashMap;
use std::io;
use std::io::BufRead;

const BLINKS: u8 = 75;

fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut stones: Vec<u64> = vec![];
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        stones = puzzle_line.split(' ').map(|s| s.parse::<u64>().unwrap()).collect()
    }
    let mut length = 0;
    let mut cache: HashMap<(u64, u8), u64> = HashMap::new();
    for stone in stones {
        length += recurse_blink(stone, 0, &mut cache);
    }
    println!("len:{}", length);
}

fn recurse_blink(stone: u64, depth: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    let mut has_key = false;
    let result = match (stone, depth) {
        _ if depth == BLINKS => 1,
        _ if cache.contains_key(&(stone, depth)) => {
            has_key = true;
            cache[&(stone, depth)]
        }
        _ if stone == 0 => recurse_blink(1, depth + 1, cache),
        (v, _) if count_digits(v) % 2 == 0 => {
            let split_digits = split_number(v);
            recurse_blink(split_digits.0, depth + 1, cache) + recurse_blink(split_digits.1, depth + 1, cache)
        }
        _ => recurse_blink(stone * 2024, depth + 1, cache)
    };
    if !has_key { cache.insert((stone, depth), result); };
    result
}

#[inline(always)]
fn count_digits(i: u64) -> u64 {
    let mut count = 1;
    let mut n = i;
    while n / 10 > 0 {
        count += 1;
        n /= 10;
    }
    count
}
#[inline(always)]
fn split_number(i: u64) -> (u64, u64) {
    let digit_count = count_digits(i);
    let half_digits = digit_count / 2;

    let divisor = 10u64.pow(half_digits as u32);

    let left_half = i / divisor;
    let right_half = i % divisor;

    (left_half, right_half)
}
