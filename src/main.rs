use std::cmp::PartialEq;
use std::fmt::Formatter;
use std::io::BufRead;
use std::{fmt, io};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    None,
    Obstacle,
    GuardUp,
    GuardDown,
    GuardLeft,
    GuardRight,
    GuardVisited,
    GuardVisitedVertical,
    GuardVisitedHorizontal,
    GuardVisitedCorner,
}

const GUARD: [Tile; 4] = [Tile::GuardUp, Tile::GuardDown, Tile::GuardLeft, Tile::GuardRight];

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match *self {
            Tile::Obstacle => '#',
            Tile::GuardUp => '^',
            Tile::GuardDown => 'v',
            Tile::GuardLeft => '<',
            Tile::GuardRight => '>',
            Tile::GuardVisited => 'X',
            Tile::GuardVisitedVertical => '|',
            Tile::GuardVisitedHorizontal => '-',
            Tile::GuardVisitedCorner => '+',
            Tile::None => '.',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Visited {
    orientation: Tile,
    x: isize,
    y: isize,
}

fn main() {
    let stdin = io::stdin();
    let mut puzzle_lines = stdin.lock().lines();
    let mut distinct_pos = 0;
    let mut starting_map: Vec<Vec<Tile>> = Vec::new();
    let (mut starting_guard_pos_x, mut starting_guard_pos_y) = (0isize, 0isize);
    let (mut bound_x, mut bound_y) = (0usize, 0usize);
    let (mut x, mut y) = (0, 0);
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let mut map_row: Vec<Tile> = Vec::new();
        for c in puzzle_line.chars() {
            let t = c.to_tile();
            if GUARD.contains(&t) {
                starting_guard_pos_x = x;
                starting_guard_pos_y = y;
            }
            x += 1;
            map_row.push(t);
        }
        bound_x = puzzle_line.len();
        bound_y += 1;
        y += 1;
        x = 0;
        starting_map.push(map_row);
    }
    'outer: while true {
        let map = starting_map.clone();
        let (mut guard_pos_x, mut guard_pos_y) = (starting_guard_pos_x, starting_guard_pos_y);
        let mut visited: Vec<Visited> = Vec::new();

        while is_in_bounds(&guard_pos_x, &guard_pos_y, &bound_x, &bound_y) {
            let px = guard_pos_x;
            let py = guard_pos_y;
            let mut po: Tile = Tile::None;

            match map[guard_pos_y as usize][guard_pos_x as usize] {
                Tile::GuardUp => {
                    po = Tile::GuardUp;
                    map[guard_pos_y as usize][guard_pos_x as usize] = Tile::GuardVisited;
                    guard_pos_y -= 1;
                    if is_in_bounds(&guard_pos_x, &guard_pos_y, &bound_x, &bound_y) {
                        if map[guard_pos_y as usize][guard_pos_x as usize] == Tile::Obstacle {
                            guard_pos_y += 1;
                            map[guard_pos_y as usize][guard_pos_x as usize] = Tile::GuardRight;
                        } else {
                            map[guard_pos_y as usize][guard_pos_x as usize] = Tile::GuardUp;
                        }
                    }
                }
                Tile::GuardDown => {
                    po = Tile::GuardDown;
                    map[guard_pos_y as usize][guard_pos_x as usize] = Tile::GuardVisited;
                    guard_pos_y += 1;
                    if is_in_bounds(&guard_pos_x, &guard_pos_y, &bound_x, &bound_y) {
                        if map[guard_pos_y as usize][guard_pos_x as usize] == Tile::Obstacle {
                            guard_pos_y -= 1;
                            map[guard_pos_y as usize][guard_pos_x as usize] = Tile::GuardLeft;
                        } else {
                            map[guard_pos_y as usize][guard_pos_x as usize] = Tile::GuardDown;
                        }
                    }
                }
                Tile::GuardLeft => {
                    po = Tile::GuardLeft;
                    map[guard_pos_y as usize][guard_pos_x as usize] = Tile::GuardVisited;
                    guard_pos_x -= 1;
                    if is_in_bounds(&guard_pos_x, &guard_pos_y, &bound_x, &bound_y) {
                        if map[guard_pos_y as usize][guard_pos_x as usize] == Tile::Obstacle {
                            guard_pos_x += 1;
                            map[guard_pos_y as usize][guard_pos_x as usize] = Tile::GuardUp;
                        } else {
                            map[guard_pos_y as usize][guard_pos_x as usize] = Tile::GuardLeft;
                        }
                    }
                }
                Tile::GuardRight => {
                    po = Tile::GuardRight;
                    map[guard_pos_y as usize][guard_pos_x as usize] = Tile::GuardVisited;
                    guard_pos_x += 1;
                    if is_in_bounds(&guard_pos_x, &guard_pos_y, &bound_x, &bound_y) {
                        if map[guard_pos_y as usize][guard_pos_x as usize] == Tile::Obstacle {
                            guard_pos_x -= 1;
                            map[guard_pos_y as usize][guard_pos_x as usize] = Tile::GuardDown;
                        } else {
                            map[guard_pos_y as usize][guard_pos_x as usize] = Tile::GuardRight;
                        }
                    }
                }
                _ => { panic!("illegal position") }
            }
            let maybe_visited = Visited {
                orientation: po,
                x: px,
                y: py,
            };
            if visited.contains(&maybe_visited) {
                println!("Loop detected!");
                    distinct_pos += 1;
                break;
            } else {
                visited.push(maybe_visited);
            }
        }

        for map_row in map {
            for tile in map_row {
                print!("{:?}", tile)
            }
            println!();
        }

    }

    println!("bounds ({},{}) last guard pos ({},{})", bound_x, bound_y, guard_pos_x, guard_pos_y);
    println!("distinct_pos: {}", distinct_pos);
    println!("visited {:?}", visited);
}

fn is_in_bounds(guard_pos_x: &isize, guard_pos_y: &isize, bound_x: &usize, bound_y: &usize) -> bool {
    *guard_pos_x >= 0isize && *guard_pos_x < *bound_x as isize &&
        *guard_pos_y >= 0isize && *guard_pos_y < *bound_y as isize
}

trait ToTile {
    fn to_tile(&self) -> Tile;
}
impl ToTile for char {
    fn to_tile(&self) -> Tile {
        match *self {
            '#' => Tile::Obstacle,
            '^' => Tile::GuardUp,
            'v' => Tile::GuardDown,
            '<' => Tile::GuardLeft,
            '>' => Tile::GuardRight,
            'X' => Tile::GuardVisited,
            _ => Tile::None,
        }
    }
}

