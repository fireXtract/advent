use std::io;
use std::io::BufRead;

const SECRET_COUNT: usize = 2000;

fn mix(value: usize, secret_number: usize) -> usize {
    value ^ secret_number
}

fn prune(secret_number: usize) -> usize {
    secret_number % 16777216
}

fn solve(initial: usize) -> Vec<usize> {
    let mut v: Vec<usize> = vec![];
    let mut secret_number = initial;
    for i in 0..SECRET_COUNT {
        let one = prune(mix(secret_number * 64, secret_number));
        let two = prune(mix(one / 32, one));
        let three = prune(mix(two * 2048, two));
        secret_number = three;
        v.push(three)
    }
    v
}


fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();

    let mut each_last: Vec<(usize, usize)> = vec![];
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let num = puzzle_line.parse::<usize>().unwrap();
        let s = solve(num);
        each_last.push((num, *s.last().unwrap()));
        println!("{num}: {s:?}");
    }
    println!("{each_last:?}");
    let sum: usize = each_last.iter().map(|(_, last_secret)| last_secret).sum();
    println!("sum: {sum}");

    println!("EOL");
}
