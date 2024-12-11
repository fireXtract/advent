use std::convert::identity;
use std::io;
use std::io::BufRead;

fn check_direction(puzzle: &Vec<Vec<char>>, row: isize, col: isize, dx: isize, dy: isize, xmas: &[char]) -> bool {
    let w = xmas.len() as isize;
    let half_w = w / 2;
    ((-half_w)..=(half_w)).all(|i| {
        let new_row = row + i * dy;
        let new_col = col + i * dx;
        new_row >= 0 && new_col >= 0 &&
            new_row < puzzle.len() as isize && new_col < puzzle[new_row as usize].len() as isize &&
            puzzle[new_row as usize][new_col as usize] == xmas[(i+half_w) as usize]
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
    let xmas = ['M', 'A', 'S'];
    for (row, row_val) in puzzle.iter().enumerate() {
        for (col, col_val) in row_val.iter().enumerate() {
            if *col_val == 'A' {
                xmas_count += (-1..=1).flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                    .filter(|(dx, dy)| *dx != 0 && *dy != 0)
                    .map(|(dx, dy)|
                        (check_direction(&puzzle, row as isize, col as isize, dx, dy, &xmas) &&
                            check_direction(&puzzle, row as isize, col as isize, dx*-1, dy, &xmas)) ||
                            (check_direction(&puzzle, row as isize, col as isize, dx, dy, &xmas) &&
                                check_direction(&puzzle, row as isize, col as isize, dx, dy*-1, &xmas))
                    )
                    .filter(|x| *x)
                    .count();
            }
        }
    }
    println!("xmas_count: {}", xmas_count/2);
}



