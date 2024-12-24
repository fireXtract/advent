use std::collections::{HashMap, LinkedList};
use std::io;
use std::io::BufRead;


fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut indices: HashMap<String, Vec<usize>> = HashMap::new();
    let mut links: Vec<LinkedList<String>> = vec![];
    let mut i = 0;
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        println!("{puzzle_line}");
        let mut split = puzzle_line.split('-');
        let left = split.next().unwrap().to_string();
        let right = split.next().unwrap().to_string();
        let seen_left = indices.contains_key(&left);
        let seen_right = indices.contains_key(&right);
        match (seen_left, seen_right) {
            (false, false) => {
                indices.insert(left.clone(), vec![i]);
                indices.insert(right.clone(), vec![i]);
                links.push(LinkedList::from([left, right]));
                i += 1;
            }
            (true, false) => {
                let mut right_indices = vec![];
                for &index in &indices[&left] {
                    links[index]..push_back(right.clone());
                    links[index].push_back(right.clone());
                    right_indices.push(index);
                }
                indices.insert(right.clone(), right_indices);
            }
            (false, true) => {
                let mut left_indices = vec![];
                for &index in &indices[&right] {
                    links[index].push_back(left.clone());
                    left_indices.push(index);
                }
                indices.insert(left.clone(), left_indices);
            }
            (true, true) => {
                for &index in &indices[&left] {
                    links[index].push_back(right.clone());
                }
                for &index in &indices[&right] {
                    links[index].push_back(left.clone());
                }
            }
        }
        println!("{indices:?}");
        println!("{links:?}");
    }

    println!("EOL");
}
