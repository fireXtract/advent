use std::cmp::Ordering;
use std::io;
use std::io::BufRead;
use rand::prelude::*;

const SECRET_COUNT: usize = 2000;

fn mix(value: usize, secret_number: usize) -> usize {
    value ^ secret_number
}

fn prune(secret_number: usize) -> usize {
    secret_number % 16777216
}

fn price(secret_number: usize) -> isize {
    (secret_number % 10) as isize
}

fn solve(initial: usize) -> Vec<usize> {
    let mut v: Vec<usize> = vec![initial];
    let mut secret_number = initial;
    for _ in 0..SECRET_COUNT {
        let one = prune(mix(secret_number * 64, secret_number));
        let two = prune(mix(one / 32, one));
        let three = prune(mix(two * 2048, two));
        secret_number = three;
        v.push(three)
    }
    v
}

fn solve2(secret_numbers: Vec<isize>, sequence: &[isize; 4]) -> isize {
    let mut v2: [isize; 4] = [0isize; 4];
    let mut count = 1;
    let mut filled = false;
    let mut last_price = 0isize;
    // print!("[");
    for i in secret_numbers.windows(2) {
        let next = i[1] - i[0];
        v2[0] = v2[1];
        v2[1] = v2[2];
        v2[2] = v2[3];
        v2[3] = next;
        // print!(", {next}");
        // println!("{v2:?}");
        if filled || count >= 4 {
            filled = true;
            // println!("filled {filled} compare {} {}", i[0], i[1]);
            if v2.cmp(sequence) == Ordering::Equal {
                last_price = i[1];
                // println!("found match {last_price}");
                break;
            }
            // compare
        } else {
            // println!("count: {count}");
            count += 1;
        }
    }
    // print!("]");
    last_price
}


fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();

    let mut each_last: Vec<(usize, usize)> = vec![];
    let mut num_secrets: Vec<(usize, Vec<usize>)> = vec![];
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let num = puzzle_line.parse::<usize>().unwrap();
        let s = solve(num);
        println!("{num}: {s:?}");
        each_last.push((num, *s.last().unwrap()));
        num_secrets.push((num, s));
    }
    println!("{each_last:?}");
    let sum: usize = each_last.iter().map(|(_, last_secret)| last_secret).sum();
    println!("sum p1: {sum}");
    println!("price of 123 {}", price(123));
    println!("price of 15887950 {}", price(15887950));
    println!("price of 16495136 {}", price(16495136));
    println!("price of 527345 {}", price(527345));

    let mut vprices : Vec<Vec<isize>> = vec![];
    for (_, secrets) in num_secrets {
        println!("{secrets:?}");
        let prices: Vec<isize> = secrets.iter().map(|&s| price(s)).collect();
        println!("{prices:?}");
        vprices.push(prices);
    }
    let sqnc = &[-2isize, 1isize, -1isize, 3isize];
    let mut biggest_bananas: ([isize;4], isize) = (*sqnc, -1);
    let mut rng =  rand::rng();
    let mut i0r: Vec<isize> = (-9isize..=9).collect();
    i0r.shuffle(&mut rng);
    let mut i1r: Vec<isize> = (-9isize..=9).collect();
    i1r.shuffle(&mut rng);
    let mut i2r: Vec<isize> = (-9isize..=9).collect();
    i2r.shuffle(&mut rng);
    let mut i3r: Vec<isize> = (-9isize..=9).collect();
    i3r.shuffle(&mut rng);
    for &i0 in &i0r {
        for &i1 in &i1r {
            for &i2 in &i2r {
                for &i3 in &i3r {
                    let mut bananas = 0isize;
                    let sqnc = [i0, i1, i2, i3];
                    for prices in vprices.clone() {
                        bananas += solve2(prices, &sqnc);
                    }
                    if bananas > biggest_bananas.1 {
                        biggest_bananas = (sqnc, bananas);
                        println!("found new biggest_bananas: {biggest_bananas:?}");
                    } else {
                        println!("dropped not biggest_bananas: {biggest_bananas:?}");
                    }
                }
            }
        }
    }

    println!("bananas: {biggest_bananas:?}");

    println!("EOL");
}
