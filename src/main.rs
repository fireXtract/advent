use std::cmp::PartialEq;
use regex::Regex;
use std::io;
use std::io::BufRead;

/**
    0: x
    1: y
 */
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Pos(isize, isize);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Robot {
    pos: Pos,
    vel: Pos,
}

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
    let mut puzzle_lines = io::stdin().lock().lines();
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut quad_top_left = 0;
    let mut quad_top_right = 0;
    let mut quad_bottom_left = 0;
    let mut quad_bottom_right = 0;
    let mut robots: Vec<Robot> = vec![];
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        let caps = re.captures(&*puzzle_line).unwrap();
        let robot = Robot {
            pos: Pos(caps.get(1).unwrap().as_str().parse::<isize>().unwrap(), caps.get(2).unwrap().as_str().parse::<isize>().unwrap()),
            vel: Pos(caps.get(3).unwrap().as_str().parse::<isize>().unwrap(), caps.get(4).unwrap().as_str().parse::<isize>().unwrap()),
        };
        robots.push(robot);
        println!("{robot:?}");
        println!("sanity {}, {}, {}", 15 % 10, -15 % 10, euclidean_mod(-15 , 10));

    }
    let mut p2bots = robots.clone();
    for r in robots.iter_mut() {
        advance(r, 100);
        match r.pos {
            Pos(x, y) if x < WIDE / 2 && y < TALL / 2 => quad_top_left += 1,
            Pos(x, y) if x > WIDE / 2 && y < TALL / 2 => quad_top_right += 1,
            Pos(x, y) if x < WIDE / 2 && y > TALL / 2 => quad_bottom_left += 1,
            Pos(x, y) if x > WIDE / 2 && y > TALL / 2 => quad_bottom_right += 1,
            _ => println!("robot {r:?} on median"),
        }
    }
    score_p1 += quad_top_left * quad_top_right * quad_bottom_left * quad_bottom_right;

    for t in 0..10000 {
        println!("After {t} seconds ({},{})", p2bots[0].pos.0, p2bots[0].pos.1);
        for py in 0..TALL {
            for px in 0..WIDE {
                let p_pos = Pos(px,py);
                let robot_poss = p2bots.iter().filter(|&r|r.pos == p_pos).count();
                if robot_poss > 0 {
                    print!("1");
                } else {
                    print!(".");
                }
            }
            print!(" {t}");
            println!();
        }
        for r in p2bots.iter_mut() {
            advance(r, 1);
        }
    }

    // part 2 lazy solution, I guessed that the Christmas tree would be solid, find lines w/ grep
    // ./target/release/advent.exe <./src/day14/input > treestack.txt
    // grep '111111' treestack.txt

    println!("score_p1: {score_p1}");
}
