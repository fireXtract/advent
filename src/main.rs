use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use crate::Signal::UNKNOWN;

#[derive(Debug, Eq, PartialEq)]
struct Wire {
    name: String,
    signal: Signal
}

impl Wire {
    fn is_ready(&self) -> bool {
        [Signal::ZERO,Signal::ONE].contains(&self.signal)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Signal {
    XOR(Box<(Wire, Wire)>),
    AND(Box<(Wire,Wire)>),
    OR(Box<(Wire,Wire)>),
    ONE,
    ZERO,
    UNKNOWN,
}

fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut wires: Vec<Wire> = vec![];
    let mut links: HashMap<String, &Wire> = HashMap::new();
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        println!("{puzzle_line}");
        if puzzle_line.contains(':') {
            let mut split = puzzle_line.split(": ");
            let name = split.next();
            let value = split.next();
            if value.is_some() {
                wires.push(Wire{
                    name: name.unwrap().to_string(),
                    signal: if value.unwrap() == "1" { Signal::ONE } else {Signal::ZERO }
                })
            } else {
                panic!("Unknown")
            }
        } else if puzzle_line.contains("->") {
            let mut split = puzzle_line.split(" -> ");
            let left = split.next().unwrap();
            let right = split.next().unwrap().to_string();
            match left {
                xor if xor.contains("XOR") => {
                    let mut split = puzzle_line.split(" XOR ");
                    let left_left = split.next().unwrap().to_string();
                    let left_right = split.next().unwrap().to_string();
                    let left_wire: Wire = if links.contains_key(&left_left) {

                    } else {
                        Wire {
                            name: left_left,
                            signal: UNKNOWN,
                        }
                    };
                    let right_wire: Wire = if links.contains_key(&left_right) {
                        // links[left_right]
                        Wire {
                            name: left_right,
                            signal: UNKNOWN,
                        }
                    } else {
                        Wire {
                            name: left_right,
                            signal: UNKNOWN,
                        }
                    };
                    let xor_wire = Wire {
                        name: right,
                        signal: Signal::XOR(Box::new((left_wire,right_wire))),
                    };
                    wires.push(xor_wire);
                }
                _ => {}
            }
        }
    }

    println!("{wires:?}");
    println!("{links:?}");

    println!("EOL");
}
