use regex::Regex;
use std::io;
use std::io::BufRead;

// (x,y)
#[derive(Debug, Clone, Copy, Default)]
struct Pos(isize, isize);

#[derive(Debug, Clone, Copy, Default)]
struct Robot {
    pos: Pos,
    vel: Pos,
}

const A_COST: usize = 3;
const B_COST: usize = 1;
const WIDE: isize = 101;
const TALL: isize = 103;
// const WIDE: isize = 11;
// const TALL: isize = 7;

fn euclidean_mod(a: isize, b: isize) -> isize {
    let rem = a % b;
    if rem < 0 {
        rem + b
    } else {
        rem
    }
}

#[inline(always)]
fn advance(machine: &mut Robot, t: isize) {
    let x = machine.pos.0;
    let y = machine.pos.1;
    let dx = machine.vel.0 * t;
    let dy = machine.vel.1 * t;

    machine.pos.0 = euclidean_mod(x + dx, WIDE);
    machine.pos.1 = euclidean_mod(y + dy, TALL);
}

fn main() {
    let mut score_p1 = 0;
    let mut score_p2 = 0;
    let mut puzzle_lines = io::stdin().lock().lines();
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robot: Robot = Robot::default();
    let mut quad_top_left = 0;
    let mut quad_top_right = 0;
    let mut quad_bottom_left = 0;
    let mut quad_bottom_right = 0;
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let caps = re.captures(&*puzzle_line).unwrap();
        let mut robot = Robot {
            pos: Pos(caps.get(1).unwrap().as_str().parse::<isize>().unwrap(), caps.get(2).unwrap().as_str().parse::<isize>().unwrap()),
            vel: Pos(caps.get(3).unwrap().as_str().parse::<isize>().unwrap(), caps.get(4).unwrap().as_str().parse::<isize>().unwrap()),
        };
        println!("{robot:?}");
        println!("sanity {}, {}, {}", 15 % 10, -15 % 10, euclidean_mod(-15 , 10));
        for t in 0..5 {
            println!("After {t} seconds ({},{})", robot.pos.0, robot.pos.1);
            for py in 0..TALL {
                for px in 0..WIDE {
                    if robot.pos.0 == px && robot.pos.1 == py {
                        print!("1");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            advance(&mut robot, 1);
        }
        // advance(&mut robot, 100);
        match robot.pos {
            Pos(x, y) if x < WIDE / 2 && y < TALL / 2 => quad_top_left += 1,
            Pos(x, y) if x > WIDE / 2 && y < TALL / 2 => quad_top_right += 1,
            Pos(x, y) if x < WIDE / 2 && y > TALL / 2 => quad_bottom_left += 1,
            Pos(x, y) if x > WIDE / 2 && y > TALL / 2 => quad_bottom_right += 1,
            _ => println!("robot {robot:?} on median"),
        }
        // let cost = (a * A_COST) + (b * B_COST);
    }
    score_p1 += quad_top_left * quad_top_right * quad_bottom_left * quad_bottom_right;

    println!("score_p1: {score_p1}");
    println!("score_p2: {score_p2}");
}
