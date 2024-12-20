use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Formatter;
use std::io::BufRead;
use std::{fmt, io, thread};
use std::char::MAX;
use std::time::Duration;

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

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct State {
    cost: usize,
    row: usize,
    col: usize,
    skips_remaining: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Correct ordering for a min-heap:
        other.cost.cmp(&self.cost)
            .then_with(|| self.row.cmp(&other.row))
            .then_with(|| self.col.cmp(&other.col))
            .then_with(|| other.skips_remaining.cmp(&self.skips_remaining))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
const TURN_COST: usize = 1000;
const MAX_SKIPS: usize = 2;
fn shortest_path(
    grid: &[Vec<bool>],
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,

) -> Option<(Vec<(usize, usize)>, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut dist = vec![vec![vec![usize::MAX; MAX_SKIPS + 1]; cols]; rows];
    let mut prev = vec![vec![vec![(usize::MAX, usize::MAX, usize::MAX); MAX_SKIPS + 1]; cols]; rows];
    let mut pq = BinaryHeap::new();

    dist[start_row][start_col][MAX_SKIPS as usize] = 0;
    pq.push(State { cost: 0, row: start_row, col: start_col, skips_remaining: MAX_SKIPS });

    let mut end_state: Option<State> = None; // Store the end state

    while let Some(state @ State { cost, row, col, skips_remaining }) = pq.pop() {
        println!("{state:?}");
        if cost > dist[row][col][skips_remaining] {
            continue;
        }

        if row == end_row && col == end_col {
            end_state = Some(state); // Store the final state
            break; // Exit the loop
        }

        let deltas: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for &(dr, dc) in &deltas {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;

            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                let is_wall = !grid[nr][nc];
                let next_cost = cost + 1;

                if is_wall && skips_remaining > 0 {
                    if next_cost < dist[nr][nc][(skips_remaining - 1)] {
                        dist[nr][nc][skips_remaining - 1] = next_cost;
                        prev[nr][nc][skips_remaining - 1] = (row, col, skips_remaining);
                        pq.push(State { cost: next_cost, row: nr, col: nc, skips_remaining: skips_remaining - 1 });
                    }
                } else if !is_wall && next_cost < dist[nr][nc][skips_remaining] {
                    dist[nr][nc][skips_remaining] = next_cost;
                    prev[nr][nc][skips_remaining] = (row, col, skips_remaining);
                    pq.push(State { cost: next_cost, row: nr, col: nc, skips_remaining });
                }
            }
        }
    }
    println!("exited main loop");

    if let Some(State { cost, row: _, col: _, skips_remaining }) = end_state {
        let mut path = Vec::new();
        let mut current = (end_row, end_col, skips_remaining);

        while current != (start_row, start_col, MAX_SKIPS) {
            path.push((current.0, current.1));
            current = prev[current.0][current.1][current.2]; // Correct lookup
        }

        path.push((start_row, start_col));
        path.reverse();
        return Some((path, cost));
    } else {
        return None;
    }
}

/**
for r in &dist {
    for &c in r {
        if c == usize::MAX {
            print!("####");
        } else {
            print!("[{:02x}]", c);
        }
    }
    println!();
}
*/


fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut map_tiles: Vec<Vec<Tile>> = vec![];
    let mut map: Vec<Vec<bool>> = vec![];
    let (mut start, mut end) = ((0usize, 0usize), (0usize, 0usize));
    let (mut x, mut y) = (0usize, 0usize);
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let mut map_tile_row: Vec<Tile> = vec![];
        let mut map_row: Vec<bool> = vec![];
        for c in puzzle_line.chars() {
            if let tile = c.to_tile() {
                map_tile_row.push(tile);
                map_row.push(tile != Tile::Wall);
                if tile == Tile::Start {
                    start = (x, y);
                } else if tile == Tile::End {
                    end = (x, y);
                }
            }
            x += 1;
        }
        map_tiles.push(map_tile_row);
        map.push(map_row);
        y += 1;
        x = 0;
    }


    let printing_map = map.clone();
    for (y, row) in printing_map.clone().iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if start.0 == x && start.1 == y {
                print!("S");
            } else if end.0 == x && end.1 == y {
                print!("E");
            } else if cell {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }

    if let Some((shortest, score_p1)) = shortest_path(&map, start.0, start.1, end.0, end.1) {
        if score_p1 == usize::MAX {
            println!("unreachable score: {score_p1}");
        } else {
            println!("reachable score: {score_p1}");
        }
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
    }
    println!("EOL");
}
