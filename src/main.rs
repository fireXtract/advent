use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct WireName(String);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Signal {
    ONE,
    ZERO,
    UNKNOWN,
}

#[derive(Debug)]
enum Gate {
    XOR(WireName, WireName, WireName), // Output wire, Input 1, Input 2
    AND(WireName, WireName, WireName),
    OR(WireName, WireName, WireName),
    INPUT(WireName), // Represents an external input
}

#[derive(Debug)]
struct Circuit {
    gates: Vec<Gate>,
    wire_values: HashMap<WireName, Signal>,
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            gates: Vec::new(),
            wire_values: HashMap::new(),
        }
    }

    fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    fn set_input(&mut self, wire_name: &WireName, signal: Signal) {
        self.wire_values.insert(wire_name.clone(), signal);
    }

    fn evaluate(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for gate in self.gates.iter() {
                match gate {
                    Gate::INPUT(output) => {
                        // Inputs are already set, nothing to evaluate
                    }
                    Gate::XOR(output, input1, input2) => {
                        let val = match (self.wire_values.get(input1), self.wire_values.get(input2)) {
                            (Some(Signal::ONE), Some(Signal::ZERO)) | (Some(Signal::ZERO), Some(Signal::ONE)) => Signal::ONE,
                            (Some(Signal::ONE), Some(Signal::ONE)) | (Some(Signal::ZERO), Some(Signal::ZERO)) => Signal::ZERO,
                            _ => Signal::UNKNOWN,
                        };
                        if self.wire_values.get(output) != Some(&val) {
                            self.wire_values.insert(output.clone(), val);
                            changed = true;
                        }
                    }
                    Gate::AND(output, input1, input2) => {
                        let val = match (self.wire_values.get(input1), self.wire_values.get(input2)) {
                            (Some(Signal::ONE), Some(Signal::ONE)) => Signal::ONE,
                            (Some(Signal::ZERO), _) | (_, Some(Signal::ZERO)) => Signal::ZERO,
                            _ => Signal::UNKNOWN,
                        };
                        if self.wire_values.get(output) != Some(&val) {
                            self.wire_values.insert(output.clone(), val);
                            changed = true;
                        }
                    }
                    Gate::OR(output, input1, input2) => {
                        let val = match (self.wire_values.get(input1), self.wire_values.get(input2)) {
                            (Some(Signal::ONE), _) | (_, Some(Signal::ONE)) => Signal::ONE,
                            (Some(Signal::ZERO), Some(Signal::ZERO)) => Signal::ZERO,
                            _ => Signal::UNKNOWN,
                        };
                        if self.wire_values.get(output) != Some(&val) {
                            self.wire_values.insert(output.clone(), val);
                            changed = true;
                        }
                    }
                }
            }
        }
    }
    fn to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph circuit {\n");
        dot.push_str("  rankdir=LR;\n"); // Left to right layout

        // Define nodes (wires) with colors
        for (wire, signal) in &self.wire_values {
            let color = if wire.0.starts_with("x") {
                "yellow"
            } else if wire.0.starts_with("y") {
                "orange"
            } else if wire.0.starts_with("z") {
                "green"
            } else if wire.0 == "INPUT" {
                "grey"
            } else {
                "lightblue"
            };
            // let shape = if wire.0 == "INPUT"{
            //     "box"
            // } else {
            //     "ellipse"
            // };
            // let style = if *signal == Signal::ONE{
            //     "filled"
            // } else {
            //     "solid"
            // };
            dot.push_str(&format!(
                "  \"{}\" [label=\"{}\", fillcolor=\"{}\", style=\"{}\", shape=\"{}\"];\n",
                wire.0, wire.0, color, "filled", "ellipse"
            ));
        }

        // Define edges (connections)
        for gate in &self.gates {
            match gate {
                Gate::XOR(output, input1, input2) => {
                    dot.push_str(&format!("  \"{}\" -> \"{}\" [label=\"XOR\"];\n", input1.0, output.0));
                    dot.push_str(&format!("  \"{}\" -> \"{}\" [label=\"XOR\"];\n", input2.0, output.0));
                }
                Gate::AND(output, input1, input2) => {
                    dot.push_str(&format!("  \"{}\" -> \"{}\" [label=\"AND\"];\n", input1.0, output.0));
                    dot.push_str(&format!("  \"{}\" -> \"{}\" [label=\"AND\"];\n", input2.0, output.0));
                }
                Gate::OR(output, input1, input2) => {
                    dot.push_str(&format!("  \"{}\" -> \"{}\" [label=\"OR\"];\n", input1.0, output.0));
                    dot.push_str(&format!("  \"{}\" -> \"{}\" [label=\"OR\"];\n", input2.0, output.0));
                }
                Gate::INPUT(output) => {
                    dot.push_str(&format!("  \"INPUT\" -> \"{}\" [label=\"INPUT\"];\n", output.0));
                }
            }
        }

        dot.push_str("}\n");
        dot
    }
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split: Vec<&str> = s.splitn(2, " -> ").collect();
        if split.len() != 2 {
            return Err(format!("Invalid format: {}", s));
        }
        let output = WireName(split[1].to_string());
        let expression = split[0];
        println!("expr: {expression:?}");
        println!("output: {output:?}");

        let parts: Vec<&str> = expression.splitn(3, " ").collect();
        println!("{parts:?}");
        let gate_str = parts[1];
        let inputs: [WireName; 2] = [parts[0], parts[2]]
            .map(|name| WireName(name.to_string()));

        match gate_str {
            "XOR" => Ok(Gate::XOR(output, inputs[0].clone(), inputs[1].clone())),
            "AND" => Ok(Gate::AND(output, inputs[0].clone(), inputs[1].clone())),
            "OR" => Ok(Gate::OR(output, inputs[0].clone(), inputs[1].clone())),
            "INPUT" => Ok(Gate::INPUT(output)),
            _ => Err(format!("Invalid gate type: {}", gate_str)),
        }
    }
}

fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();

    let mut circuit = Circuit::new();
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        println!("{puzzle_line}");
        if puzzle_line.contains(':') {
            let mut split = puzzle_line.split(": ");
            let name = split.next().unwrap().to_string();
            let value = split.next();
            if value.is_some() {
                circuit.set_input(
                    &WireName(name),
                    if value.unwrap() == "1"
                    { Signal::ONE } else { Signal::ZERO },
                );
            } else {
                panic!("Unknown")
            }
        } else if puzzle_line.contains("->") {
            if let Ok(gate) = Gate::from_str(&*puzzle_line) {
                circuit.add_gate(gate);
            } else {
                println!("illegal gate");
            }
        }
    }

    println!("{circuit:#?}");
    circuit.evaluate();
    println!("{circuit:#?}");
    let mut zs: Vec<(&WireName, &Signal)> = circuit.wire_values.iter().filter(|(wire, _)| wire.0.starts_with("z")).collect();
    zs.sort_by_key(|(wire, _)| wire.0.clone());
    zs.reverse();
    println!("{zs:#?}");
    let binary_string: String = zs
        .iter()
        .map(|(_, signal)| match signal {
            Signal::ONE => '1',
            Signal::ZERO => '0',
            Signal::UNKNOWN => panic!("Unknown signal encountered in z registers"), // Handle unknowns appropriately
        })
        .collect();

    println!("Binary string: {}", binary_string);
    if !binary_string.is_empty() {
        if let Ok(decimal_value) = u64::from_str_radix(&binary_string, 2) {
            println!("Decimal value: {}", decimal_value);
        } else {
            println!("Error converting binary string to decimal");
        }
    } else {
        println!("No z registers found.");
    }

    println!();

    println!("{}", circuit.to_dot());
    println!("EOL");
}
