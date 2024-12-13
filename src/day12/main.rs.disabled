use std::collections::HashMap;
use std::io;
use std::io::BufRead;

// (x,y)
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
struct Pos(usize, usize);

impl Pos {
    fn top_left(&self) -> Pos {
        Pos(self.0, self.1)
    }
    fn top_right(&self) -> Pos {
        Pos(self.0 + 1, self.1)
    }
    fn bottom_left(&self) -> Pos {
        Pos(self.0, self.1 + 1)
    }
    fn bottom_right(&self) -> Pos {
        Pos(self.0 + 1, self.1 + 1)
    }
    fn corners(&self) -> [Pos; 4] {
        [
            self.top_left(),
            self.top_right(),
            self.bottom_left(),
            self.bottom_right(),
        ]
    }
}

fn get_neighbor<'a>(
    trails: &'a Vec<Vec<char>>,
    at: &Pos,
    dx: isize,
    dy: isize,
) -> Option<(&'a char, Pos)> {
    let new_x = at.0 as isize + dx;
    let new_y = at.1 as isize + dy;
    if new_x < 0 || new_y < 0 {
        return None;
    }

    let (new_y, new_x) = (new_y as usize, new_x as usize);
    trails
        .get(new_y)
        .and_then(|row| row.get(new_x))
        .map(|neighbor_val| (neighbor_val, Pos(new_x, new_y)))
}

fn traverse(
    map: &Vec<Vec<char>>,
    c_pos: Pos,
    corners: &mut HashMap<Pos, Vec<Pos>>,
    visited_pos: &mut Vec<Pos>,
) -> (usize, usize) {
    let current_val = map[c_pos.1][c_pos.0];
    if visited_pos.contains(&c_pos) {
        return (0, 0);
    } else {
        visited_pos.push(c_pos);
    }

    c_pos.corners().iter().for_each(|corner| {
        corners
            .entry(*corner)
            .and_modify(|v| v.push(c_pos))
            .or_insert(vec![c_pos]);
    });

    let neighbors = [
        get_neighbor(map, &c_pos, -1, 0),// left
        get_neighbor(map, &c_pos, 1, 0), // right
        get_neighbor(map, &c_pos, 0, -1),// up
        get_neighbor(map, &c_pos, 0, 1), // down
    ];
    let mut area = 1;
    let mut perimeter = 4;

    for &(&value, pos) in neighbors.iter().flatten() {
        if value == current_val {
            perimeter -= 1;
            let next_paths = traverse(map, pos, corners, visited_pos);
            area += next_paths.0;
            perimeter += next_paths.1;
        }
    }

    (area, perimeter)
}

fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut garden_plots: Vec<Vec<char>> = vec![];
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let garden_row: Vec<char> = puzzle_line.chars().collect();
        garden_plots.push(garden_row);
    }
    let width = garden_plots[0].len();
    let height = garden_plots.len();
    let total_positions = width * height;
    let mut visited_pos: Vec<Pos> = vec![];

    let mut price_p1 = 0;
    let mut price_p2 = 0;
    let (mut x, mut y) = (0usize, 0usize);
    while visited_pos.len() < total_positions {
        let mut corners: HashMap<Pos, Vec<Pos>> = HashMap::new();
        let (area, perimeter) = traverse(&garden_plots, Pos(x, y), &mut corners, &mut visited_pos);
        price_p1 += area * perimeter;
        let mut sides = 0;
        for v in corners.values() {
            let mut sum = 0;
            let mut at = 0;
            let l = v.len();
            let mut corner_case = false;
            if l > 1 {
                while at < l {
                    for i in 0..l {
                        if i != at {
                            if  l == 2 && (v[i].0 != v[at].0 && v[i].1 != v[at].1) {
                                corner_case = true;
                            }
                            if v[i].0 == v[at].0 || v[i].1 == v[at].1{
                                sum += 1;
                                break;
                            }
                        }
                    }
                    at +=1;
                }
            } else {
                sum = 1;
            }
            if corner_case {
                sides += 2;
                continue;
            }
            if sum % 2 == 1 {
                sides += 1;
            }

        }
        price_p2 += area * sides;
        x += 1;
        if x >= width {
            y += 1;
            x = 0;
        }
    }

    println!("price_p1: {price_p1}");
    println!("price_p2: {price_p2}");
}
