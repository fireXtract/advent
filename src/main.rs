use std::convert::identity;
use std::io;
use std::io::BufRead;

fn check_direction(puzzle: &Vec<Vec<char>>, row: usize, col: usize, dx: isize, dy: isize, xmas: &[char]) -> bool {
    let w = xmas.len();
    (0..w).all(|i| {
        let new_row = (row as isize + i as isize * dy) as usize;
        let new_col = (col as isize + i as isize * dx) as usize;
        new_row < puzzle.len() && new_col < puzzle[new_row].len() && puzzle[new_row][new_col] == xmas[i]
    })
}
fn main() {
    let stdin = io::stdin();
    let mut puzzle_lines = stdin.lock().lines();
    let mut xmas_count = 0;
    let mut puzzle: Vec<Vec<char>> = Vec::new();
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let a: Vec<char> = puzzle_line.chars().map(identity).collect();
        puzzle.push(a);
    }
    let xmas = ['X', 'M', 'A', 'S'];
    for (row, row_val) in puzzle.iter().enumerate() {
        for (col, col_val) in row_val.iter().enumerate() {
            if *col_val == 'X' {
                xmas_count += (-1..=1).flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                    .filter(|(dx, dy)| *dx != 0 || *dy != 0)
                    .map(|(dx, dy)| check_direction(&puzzle, row, col, dx, dy, &xmas))
                    .filter(|x| *x)
                    .count();
            }
        }
    }
    println!("xmas_count: {}", xmas_count);
}



