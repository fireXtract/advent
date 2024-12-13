use std::io;
use std::io::BufRead;

// (x,y)
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Default)]
struct Pos(usize, usize);

#[derive(Debug, Clone, Copy, Default)]
struct Machine {
    prize: Pos,
    a: Pos,
    b: Pos,
}

const MAX_PRESSES: usize = 100;
const A_COST: usize = 3;
const B_COST: usize = 1;

// elimination method
fn solve(machine: Machine) -> Option<(usize, usize)> {
    let a1 = machine.a.0 as f64;
    let a2 = machine.a.1 as f64;
    let b1 = machine.b.0 as f64;
    let b2 = machine.b.1 as f64;
    let c1 = machine.prize.0 as f64;
    let c2 = machine.prize.1 as f64;

    let multiplier = a2 / a1;
    let new_b2 = b2 - multiplier * b1;
    let new_c2 = c2 - multiplier * c1;
    let b_presses = new_c2 / new_b2;
    let a_presses = (c1 - b1 * b_presses) / a1;

    let (a_cast, b_cast) = (a_presses.round() as usize, b_presses.round() as usize);
    if a_cast * machine.a.0 + b_cast * machine.b.0 != machine.prize.0 ||
        a_cast * machine.a.1 + b_cast * machine.b.1 != machine.prize.1 {
        return None;
    }

    Some((a_cast, b_cast))
}

fn main() {
    let mut score_p1 = 0;
    let mut score_p2 = 0;
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut machine: Machine = Machine::default();
    let re = regex::Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        if puzzle_line.starts_with("Button A:") {
            let x = puzzle_line.get(12..14).unwrap().parse::<usize>().unwrap();
            let y = puzzle_line.get(18..20).unwrap().parse::<usize>().unwrap();
            machine = Machine { prize: Pos(0, 0), a: Pos(x, y), b: Pos(0, 0) };
        }
        if puzzle_line.starts_with("Button B:") {
            let x = puzzle_line.get(12..14).unwrap().parse::<usize>().unwrap();
            let y = puzzle_line.get(18..20).unwrap().parse::<usize>().unwrap();
            machine.b = Pos(x, y);
        }
        if puzzle_line.starts_with("Prize:") {
            let caps = re.captures(&*puzzle_line).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            machine.prize = Pos(x, y);

            if let Some((a, b)) = solve(machine) {
                let cost = (a * A_COST) + (b * B_COST);
                score_p1 += cost;
            }
            machine.prize.0 += 10000000000000;
            machine.prize.1 += 10000000000000;
            if let Some((a, b)) = solve(machine) {
                let cost = (a * A_COST) + (b * B_COST);
                score_p2 += cost;
            }
        }
    }

    println!("score_p1: {score_p1}");
    println!("score_p2: {score_p2}");
}
