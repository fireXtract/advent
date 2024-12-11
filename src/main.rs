use std::io;
use std::io::BufRead;

const TRAIL_HEAD: u8 = 0;
const TRAIL_PEAK: u8 = 9;
// (x,y)
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Pos(usize, usize);

fn get_neighbor<'a>(
    trails: &'a Vec<Vec<u8>>,
    at: &Pos,
    dx: isize,
    dy: isize,
) -> Option<(&'a u8, Pos)> {
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


fn traverse(map: &Vec<Vec<u8>>, path: Vec<Pos>) -> Vec<Vec<Pos>> {
    let current_pos = path.last().unwrap();
    let current_val = map[current_pos.1][current_pos.0];

    if current_val == TRAIL_PEAK {
        return vec![path];
    }
    let neighbors = [
        get_neighbor(map, &current_pos, -1, 0), // left
        get_neighbor(map, &current_pos, 1, 0),  // right
        get_neighbor(map, &current_pos, 0, -1), // up
        get_neighbor(map, &current_pos, 0, 1),  // down
    ];
    let mut paths: Vec<Vec<Pos>> = vec![];
    for &(&value, pos) in neighbors.iter().flatten() {
        if value == current_val + 1 {
            let mut new_path = path.clone();
            new_path.push(pos);

            let next_paths = traverse(map, new_path);
            paths.extend(next_paths);
        }
    }

    paths
}

fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut map: Vec<Vec<u8>> = vec![];
    let mut trail_heads: Vec<Pos> = vec![];
    let (mut x, mut y) = (0usize, 0usize);
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let mut map_row: Vec<u8> = vec![];
        for c in puzzle_line.chars() {
            let z = c.to_digit(10).unwrap() as u8;
            if z == TRAIL_HEAD {
                trail_heads.push(Pos(x, y));
            }
            map_row.push(z);
            x += 1;
        }
        map.push(map_row);
        y += 1;
        x = 0;
    }

    let mut score_pt1 = 0;
    let mut score_pt2 = 0;
    for trail_head in trail_heads {
        let trails_to_peak = traverse(&map, vec![trail_head]);
        let mut seen_peaks: Vec<Pos> = vec![];
        for trail in trails_to_peak {
            let trail_peak = trail.last().unwrap();
            if !seen_peaks.contains(trail_peak) {
                // print!("uniq: ");
                seen_peaks.push(*trail_peak);
                score_pt1 += 1;
            }
            score_pt2 += 1;
            // println!("{trail:?}");
        }
    }

    // for row in map {
    //     for val in row {
    //         print!("{val}");
    //     }
    //     println!();
    // }
    println!("score_pt1: {score_pt1}");
    println!("score_pt2: {score_pt2}");
}
