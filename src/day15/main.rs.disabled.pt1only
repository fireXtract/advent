use std::cmp::PartialEq;
use std::fmt::Formatter;
use std::io::BufRead;
use std::{fmt, io};

/**
0: x
1: y
 */
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Pos(usize, usize);

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    None,
    Wall,
    Box,
    Robot,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match *self {
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::Robot => '@',
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
            'O' => Tile::Box,
            '@' => Tile::Robot,
            '.' => Tile::None,
            _ => Tile::None,
        }
    }
}

fn get_neighbor<'a>(
    map: &'a Vec<Vec<Tile>>,
    at: &Pos,
    dx: isize,
    dy: isize,
) -> Option<(&'a Tile, Pos)> {
    let new_x = at.0 as isize + dx;
    let new_y = at.1 as isize + dy;
    if new_x < 0 || new_y < 0 {
        return None;
    }

    let (new_y, new_x) = (new_y as usize, new_x as usize);
    map
        .get(new_y)
        .and_then(|row| row.get(new_x))
        .map(|neighbor_val| (neighbor_val, Pos(new_x, new_y)))
}

fn traverse(
    map: &mut Vec<Vec<Tile>>,
    p: &Pos,
    dx: isize,
    dy: isize,
) -> Pos {
    println!("p {p:?} dx {dx} dy {dy}");
    let current_tile = map[p.1][p.0];
    let neighbor =
        get_neighbor(map, &p, dx, dy);

    if let Some((&neighbor_tile, neighbor_pos)) = neighbor {
        let mut next = neighbor_pos;
        println!("{neighbor:?}");
        if Tile::Box == neighbor_tile {
            // attempt to move box
            println!("attempting to move box");
            next = traverse(map, &neighbor_pos, dx, dy);
            if next != neighbor_pos {
                // we moved
                let ny = p.1 as isize + dy;
                let nx = p.0 as isize + dx;
                let (nx, ny) = (nx as usize, ny as usize);
                let next_tile = map[ny][nx];
                map[p.1][p.0] = next_tile;
                map[ny][nx] = current_tile;
                return Pos(nx, ny);
            }
        } else if Tile::None == neighbor_tile {
            let ny = p.1 as isize + dy;
            let nx = p.0 as isize + dx;
            let (nx, ny) = (nx as usize, ny as usize);
            let next_tile = map[ny][nx];
            map[p.1][p.0] = next_tile;
            map[ny][nx] = current_tile;
            return Pos(nx, ny);
        }
    } else {
        println!("Probably tried to go out of bounds");
        return *p;
    }


    *p
}


fn main() {
    let mut score_p1 = 0;
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut map: Vec<Vec<Tile>> = vec![];
    let mut instructions: Vec<char> = vec![];
    let mut robot_pos: Pos = Pos(0,0);
    let (mut x, mut y) = (0usize,0usize);
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        if puzzle_line.starts_with('#') {
            let mut map_row: Vec<Tile> = vec![];
            for c in puzzle_line.chars() {
                if let tile = c.to_tile() {
                    map_row.push(tile);
                    if tile == Tile::Robot {
                        robot_pos.0 = x;
                        robot_pos.1 = y;
                    }

                }
                x+=1;
            }
            map.push(map_row);
            y+=1;
            x=0;
        } else {
            for c in puzzle_line.chars() {
                instructions.push(c);
            }
        }
    }
    println!("{instructions:?}");

    for instruction in instructions {
        robot_pos = match instruction {
            '^' => traverse(&mut map, &robot_pos, 0, -1),
            'v' => traverse(&mut map, &robot_pos, 0, 1),
            '<' => traverse(&mut map, &robot_pos, -1, 0),
            '>' => traverse(&mut map, &robot_pos, 1, 0),
            _ => panic!("illegal")
        };
        let mut gps = 0;
        let (mut x, mut y) = (0usize,0usize);
        for r in map.clone() {
            for tile in r {
                if tile == Tile::Box {
                    gps += 100 * y + x;
                }
                print!("{tile:?}");
                x += 1;
            }
            print!(" {gps}");
            x = 0;
            y += 1;
            println!();
        }
        println!("gps: {gps}");
    }

    println!("score_p1: {score_p1}");
}
