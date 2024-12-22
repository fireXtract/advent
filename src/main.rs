use std::io;
use std::io::BufRead;


fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let (mut start, mut end) = ((0usize, 0usize), (0usize, 0usize));
    let (mut x, mut y) = (0usize, 0usize);
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        for c in puzzle_line.chars() {
            x += 1;
        }

        y += 1;
        x = 0;
    }


    println!("EOL");
}
