use std::collections::HashMap;
use std::io;
use std::io::BufRead;

// (x,y)
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Pos(usize, usize);

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


// area * perimeter
// at 0,0 with no neighbors, we'd have verticals at 0,1, and horizontals at 0,1
// 0 -> 0, 1
// 1 -> 1, 2
// 2 -> 3, 4
// 3 -> 5, 6
// 4 -> 7, 8
// verticals (x, x+1) are | |
// horizontals (y, y+1) are --
// (1,1)
//horizontals 1,2
//verticals 1,2
// neighbor up
// horizontals 0, 1
// verticals 1, 2
fn traverse(map: &Vec<Vec<char>>, current_pos: Pos, verticals: &mut HashMap<usize, isize>, horizontals: &mut HashMap<usize, isize>, visited_pos: &mut Vec<Pos>) -> (usize, usize) {
    let current_val = map[current_pos.1][current_pos.0];
    if visited_pos.contains(&current_pos) {
        return (0, 0);
    } else {
        visited_pos.push(current_pos);
    }
    let left_vertical = current_pos.0;
    let right_vertical = current_pos.0 + 1;
    let top_horizontal = current_pos.1;
    let bottom_horizontal = current_pos.1 + 1;
    verticals.entry(left_vertical).and_modify(|v| *v += 1).or_insert(1);
    verticals.entry(right_vertical).and_modify(|v| *v += 1).or_insert(1);
    horizontals.entry(top_horizontal).and_modify(|v| *v += 1).or_insert(1);
    horizontals.entry(bottom_horizontal).and_modify(|v| *v += 1).or_insert(1);

    // println!("{verticals:?}");
    let left = get_neighbor(map, &current_pos, -1, 0); // left
    let right = get_neighbor(map, &current_pos, 1, 0);  // right
    let up = get_neighbor(map, &current_pos, 0, -1); // up
    let down = get_neighbor(map, &current_pos, 0, 1);  // down
    let mut area = 1;
    let mut perimeter = 0;
    let mut had_left_match = false;
    let mut had_right_match = false;
    let mut had_up_match = false;
    let mut had_down_match = false;
    if let Some((&value, pos)) = left {
        if value == current_val {
            had_left_match = true;
            let next_paths = traverse(map, pos, verticals, horizontals, visited_pos);
            verticals.entry(pos.0 + 1).and_modify(|v| *v -= 2);
            area += next_paths.0;
            perimeter += next_paths.1;
        }
    }
    if let Some((&value, pos)) = right {
        if value == current_val {
            had_right_match = true;
            let next_paths = traverse(map, pos, verticals, horizontals, visited_pos);
            verticals.entry(pos.0).and_modify(|v| *v -= 2);
            area += next_paths.0;
            perimeter += next_paths.1;
        } else {
            if let Some((&up_right,_)) = get_neighbor(map, &current_pos, 1, -1) {
                if let Some((&down_right,_)) = get_neighbor(map, &current_pos, 1, 1) {
                    if current_val == up_right && current_val == down_right {
                        perimeter += 1;
                    }
                }
            }
        }
    }
    if let Some((&value, pos)) = up {
        if value == current_val {
            had_up_match = true;
            let next_paths = traverse(map, pos, verticals, horizontals, visited_pos);
            horizontals.entry(current_pos.1).and_modify(|v| *v -= 2);
            area += next_paths.0;
            perimeter += next_paths.1;
        }
    }
    if let Some((&value, pos)) = down {
        if value == current_val {
            had_down_match = true;
            let next_paths = traverse(map, pos, verticals, horizontals, visited_pos);
            horizontals.entry(pos.1).and_modify(|v| *v -= 2);
            area += next_paths.0;
            perimeter += next_paths.1;
        }
    }


    if (had_up_match && had_down_match) || (had_left_match && had_right_match) {
        // no perimeter
    } else {
        if !had_left_match && !had_up_match {
            perimeter += 2;
        }
        if !had_right_match && !had_down_match {
            perimeter += 2;
        }
    }


    println!("({current_val})({current_pos:?}) area was {area} and perimeter was {perimeter}");
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

    let mut price = 0;
    let (mut x, mut y) = (0usize, 0usize);
    while visited_pos.len() < total_positions {
        let mut verticals: HashMap<usize, isize> = HashMap::new();
        let mut horizontals: HashMap<usize, isize> = HashMap::new();
        let (area, perimeter) = traverse(&garden_plots, Pos(x, y), &mut verticals, &mut horizontals, &mut visited_pos);
        let sides = horizontals.values().filter(|&v| *v > 0).count() + verticals.values().filter(|&v| *v > 0).count();
        let new_price = area * perimeter;
        println!("new_price: {new_price}");
        price += new_price;
        x += 1;
        if x >= width {
            y += 1;
            x = 0;
        }
    }


    println!("price: {price}");
    // price of regions fence = area * perimeter
    // total price = sum of prices of region fences
}


