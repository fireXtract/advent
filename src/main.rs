use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Formatter;
use std::io::BufRead;
use std::{fmt, io};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    None,
    Wall,
    Start,
    End,
    Deer,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match *self {
            Tile::Wall => '#',
            Tile::Start => 'S',
            Tile::End => 'E',
            Tile::Deer => '@',
            Tile::None => '.',
        };
        write!(f, "{}", c)
    }
}

trait ToTile {
    fn to_tile(&self) -> Tile;
}
impl ToTile for char {
    fn to_tile(&self) -> Tile {
        match *self {
            '#' => Tile::Wall,
            'S' => Tile::Start,
            'E' => Tile::End,
            '@' => Tile::Deer,
            '.' => Tile::None,
            _ => Tile::None,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct State {
    cost: usize,
    row: usize,
    col: usize,
    prev_dx: i64,
    prev_dy: i64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse for min-heap behavior
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
const TURN_COST: usize = 1000;
fn shortest_path(
    grid: &[Vec<bool>],
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
) -> (Vec<(usize, usize)>, usize) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut dist = vec![vec![usize::MAX; cols]; rows];
    let mut prev = vec![vec![(usize::MAX, usize::MAX); cols]; rows];
    let mut pq = BinaryHeap::new();

    dist[start_row][start_col] = 0;
    pq.push(State {
        cost: 0,
        row: start_row,
        col: start_col,
        prev_dx: 0,
        prev_dy: 0,
    });

    while let Some(State {
                       cost,
                       row,
                       col,
                       prev_dx,
                       prev_dy,
                   }) = pq.pop()
    {
        // If this state is already worse than known best, skip it
        if cost > dist[row][col] {
            continue;
        }

        // Stop if we've reached the target
        if row == end_row && col == end_col && prev_dx == 0 && prev_dy == 1 {
            break;
        }

        // All possible movement directions
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        for &(dr, dc) in &directions {
            let new_row = row as i64 + dr;
            let new_col = col as i64 + dc;

            if new_row >= 0 && new_row < rows as i64 && new_col >= 0 && new_col < cols as i64 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if grid[new_row][new_col] {
                    let dx = dr;
                    let dy = dc;

                    let tentative_distance = cost + 1;

                    if tentative_distance < dist[new_row][new_col] {
                        dist[new_row][new_col] = tentative_distance;
                        prev[new_row][new_col] = (row, col);
                        pq.push(State {
                            cost: tentative_distance,
                            row: new_row,
                            col: new_col,
                            prev_dx: dx,
                            prev_dy: dy,
                        });
                    }
                } else {
                    println!("Blocked by wall at ({}, {})", new_row, new_col);
                }
            }
        }
    }

    // Reconstruct the path
    let mut path = Vec::new();
    let mut current = (end_row, end_col);

    if dist[end_row][end_col] == usize::MAX {
        return (path, usize::MAX); // No path found
    }

    while current != (start_row, start_col) {
        path.push(current);
        current = prev[current.0][current.1];
    }

    for r in &dist {
        for &c in r {
            if c == usize::MAX {
                print!("#######");
            } else {
                print!("[{:05x}]", c);
            }
        }
        println!();
    }

    path.push((start_row, start_col));
    path.reverse();

    (path, dist[end_row][end_col])
}


fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut map: Vec<Vec<bool>> = vec![vec![true; 7]; 7];
    let mut start = (0usize, 0usize);
    let mut end = (6usize, 6usize);
    let (mut row, mut col) = (0usize, 0usize);
    let mut bytes = 12isize;
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        if bytes > 0 {
            let byte_pos: Vec<usize> = puzzle_line.split(',').map(|v| v.parse::<usize>().unwrap()).collect();
            println!("{},{}", byte_pos[0], byte_pos[1]);
            map[byte_pos[1]][byte_pos[0]] = false;
        }
        bytes -= 1;
    }

    let printing_map = map.clone();
    for (y, row) in printing_map.clone().iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }

    let (shortest, score_p1) = shortest_path(&map, start.0, start.1, end.0, end.1);
    for (row, cells) in printing_map.iter().enumerate() {
        for (col, &cell) in cells.iter().enumerate() {
            if shortest.contains(&(row, col)) {
                print!("@");
            } else {
                if cell {
                    print!(".");
                } else {
                    print!("#");
                }
            }
        }
        println!();
    }

    println!("score: {score_p1}");
}
