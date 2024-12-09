use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::io::BufRead;
use std::{fmt, io};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    None,
    Obstacle,
    ObstacleManual,
    GuardUp,
    GuardDown,
    GuardLeft,
    GuardRight,
    GuardVisited,
}

const GUARD: [Tile; 4] = [Tile::GuardUp, Tile::GuardDown, Tile::GuardLeft, Tile::GuardRight];
const OBSTACLE: [Tile; 2] = [Tile::ObstacleManual, Tile::Obstacle];

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match *self {
            Tile::Obstacle => '#',
            Tile::ObstacleManual => '0',
            Tile::GuardUp => '^',
            Tile::GuardDown => 'v',
            Tile::GuardLeft => '<',
            Tile::GuardRight => '>',
            Tile::GuardVisited => 'X',
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
    let mut drop_obstacle_after: isize = isize::MAX;
    let mut first_drop_obstacle_after: isize = isize::MIN;
    let mut known_obstacles: Vec<(usize,usize)> = Vec::new();

    'outer: loop {
        let mut map = starting_map.clone();
        let (mut guard_pos_x, mut guard_pos_y) = (starting_guard_pos_x, starting_guard_pos_y);
        let mut visited: Vec<Visited> = Vec::new();
        let mut visitedmap: HashMap<isize, Vec<Visited>> = HashMap::new();

        let mut drop_after: isize = drop_obstacle_after;

        while is_in_bounds(&guard_pos_x, &guard_pos_y, &bound_x, &bound_y) {
            let px = guard_pos_x;
            let py = guard_pos_y;
            let mut po: Tile = map[guard_pos_y as usize][guard_pos_x as usize];


            let maybe_obstacle = match po {
                Tile::GuardUp => move_guard(&mut map, &mut guard_pos_x, &mut guard_pos_y, 0, -1, &bound_x, &bound_y, &mut po, &mut drop_after),
                Tile::GuardDown => move_guard(&mut map, &mut guard_pos_x, &mut guard_pos_y, 0, 1, &bound_x, &bound_y, &mut po, &mut drop_after),
                Tile::GuardLeft => move_guard(&mut map, &mut guard_pos_x, &mut guard_pos_y, -1, 0, &bound_x, &bound_y, &mut po, &mut drop_after),
                Tile::GuardRight => move_guard(&mut map, &mut guard_pos_x, &mut guard_pos_y, 1, 0, &bound_x, &bound_y, &mut po, &mut drop_after),
                _ => { panic!("illegal position") }
            };
            if maybe_obstacle.is_some() {
                let obstacle = maybe_obstacle.unwrap();
                if known_obstacles.contains(&obstacle) {
                    println!("Already tried this obstacle, exiting early {obstacle:?}");
                    break;
                } else {
                    println!("Obstacle placed at {obstacle:?}");
                    known_obstacles.push(obstacle);
                }
            }
            let maybe_visited = Visited {
                orientation: po,
                x: px,
                y: py,
            };

            if visitedmap.get(&px).map_or(false, |v| v.contains(&maybe_visited)) {
                println!("Loop detected! ({},{})", px, py);
                distinct_pos += 1;
                println!("distinct_pos: {}", distinct_pos);
                // for map_row in map {
                //     for tile in map_row {
                //         print!("{:?}", tile)
                //     }
                //     println!();
                // }
                break;
            } else {
                visited.push(maybe_visited);
                visitedmap.entry(px).or_insert_with(Vec::new).push(maybe_visited);
            }
        }



        if first_drop_obstacle_after == isize::MIN {
            let next_drop_after = visited.len() as isize - 1;
            first_drop_obstacle_after = next_drop_after;
            drop_obstacle_after = next_drop_after;
            for map_row in map {
                for tile in map_row {
                    print!("{:?}", tile)
                }
                println!();
            }
        }
        drop_obstacle_after -= 1;
        println!("bounds ({},{}) last guard pos ({},{}), drop at ({}/{})", bound_x, bound_y, guard_pos_x, guard_pos_y, drop_obstacle_after, first_drop_obstacle_after);
        // println!("visited {:?}", visited);

        if drop_obstacle_after <= 0 {
            break 'outer;
        }
    }

    println!("distinct_pos: {}", distinct_pos);
}

fn is_in_bounds(guard_pos_x: &isize, guard_pos_y: &isize, bound_x: &usize, bound_y: &usize) -> bool {
    *guard_pos_x >= 0isize && *guard_pos_x < *bound_x as isize &&
        *guard_pos_y >= 0isize && *guard_pos_y < *bound_y as isize
}

fn move_guard(map: &mut Vec<Vec<Tile>>,
              guard_pos_x: &mut isize, guard_pos_y: &mut isize,
              dx: isize, dy: isize,
              bound_x: &usize, bound_y: &usize,
              po: &mut Tile, drop_obstacle_after: &mut isize) -> Option<(usize, usize)> {
    *po = map[*guard_pos_y as usize][*guard_pos_x as usize];
    map[*guard_pos_y as usize][*guard_pos_x as usize] = Tile::GuardVisited;
    *guard_pos_x += dx;
    *guard_pos_y += dy;
    let mut obstacle_at: Option<(usize, usize)> = None;
    if is_in_bounds(guard_pos_x, guard_pos_y, bound_x, bound_y) {
        if *drop_obstacle_after == 0 {
            map[*guard_pos_y as usize][*guard_pos_x as usize] = Tile::ObstacleManual;
            obstacle_at = Some((*guard_pos_y as usize, *guard_pos_x as usize));
        }
        *drop_obstacle_after -= 1;
        if OBSTACLE.contains(&map[*guard_pos_y as usize][*guard_pos_x as usize]) {
            *guard_pos_x -= dx;
            *guard_pos_y -= dy;
            map[*guard_pos_y as usize][*guard_pos_x as usize] = match *po {
                Tile::GuardUp => Tile::GuardRight,
                Tile::GuardDown => Tile::GuardLeft,
                Tile::GuardLeft => Tile::GuardUp,
                Tile::GuardRight => Tile::GuardDown,
                _ => unreachable!(),
            };
        } else {
            map[*guard_pos_y as usize][*guard_pos_x as usize] = *po;
        }
    }
    obstacle_at
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

