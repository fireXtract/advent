use std::collections::{BTreeSet, HashMap, HashSet};
use std::io;
use std::io::BufRead;

type Graph = HashMap<String, HashSet<String>>;

fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut graph: Graph = HashMap::new();
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        println!("{puzzle_line}");
        let mut split = puzzle_line.split('-');
        let left = split.next().unwrap().to_string();
        let right = split.next().unwrap().to_string();

        add_edge(&mut graph, left.clone(), right.clone());
        add_edge(&mut graph, right, left);
        println!("{graph:?}");
    }

    let mut threes: HashSet<BTreeSet<String>> = HashSet::new();
    for (c0, n0) in &graph {
        for c1 in n0 {
            for c2 in &graph[c1] {
                //if graph[c2].contains(c1) && graph[c2].contains(c0) && graph[c0].contains(c2) && graph[c0].contains(c1) && graph[c1].contains(c0) && graph[c1].contains(c2) {
                if graph[c2].contains(c1) && graph[c2].contains(c0) && graph[c0].contains(c2) {
                    if c0.starts_with('t') || c1.starts_with('t') || c2.starts_with('t') {
                        println!("{c0},{c1},{c2}");
                        threes.insert(BTreeSet::from([c0.clone(), c1.clone(), c2.clone()]));
                    }
                }
            }
        }
    }
    println!();
    println!("unique: {}", threes.len());
    println!("{threes:?}");

    println!("EOL");
}

fn add_edge(graph: &mut Graph, node1: String, node2: String) {
    graph.entry(node1.clone()).or_insert_with(HashSet::new).insert(node2.clone());
    graph.entry(node2.clone()).or_insert_with(HashSet::new).insert(node1.clone());
}
