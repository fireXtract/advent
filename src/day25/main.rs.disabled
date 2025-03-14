use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Formatter;
use std::io::BufRead;
use std::{fmt, io};

const KEY_HEIGHT: usize = 7;
const KEY_WIDTH: usize = 5;

// Keys have top row empty(.), bottom row filled (#),
// Locks have top row filled (#), bottom row empty (.)
#[derive(Clone, Eq, PartialEq, Hash, Default)]
struct LockKey {
    // tumbler: [[bool; KEY_WIDTH]; KEY_HEIGHT],
    pin_heights: [u8; KEY_WIDTH],
    is_lock: bool,
}

impl fmt::Debug for LockKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_lock {
            write!(f, "(lock) ")?;
        } else {
            write!(f, "(key) ")?;
        }
        // for row in self.tumbler.iter() {
        //     for col in row {
        //         write!(f, "{}", if *col { "#" } else { "." })?;
        //     }
        //     writeln!(f)?;
        // }
        write!(f, "{}", self.pin_heights.iter().map(|&h| h.to_string()).collect::<String>())?;
        Ok(())
    }
}

impl fmt::Display for LockKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_lock {
            write!(f, "(lock [")?;
        } else {
            write!(f, "(key [")?;
        }
        write!(f, "{}", self.pin_heights.iter().map(|&h| h.to_string()).collect::<String>())?;
        write!(f, "]");
        Ok(())
    }
}

impl PartialOrd<Self> for LockKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LockKey {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_lock != other.is_lock &&
            self.pin_heights.iter()
                .zip(other.pin_heights)
                .map(|(&left, right)| left + right)
                .all(|sum| sum < KEY_HEIGHT as u8 - 1) {
            return Ordering::Equal;
        }
        Ordering::Less
    }
}

fn main() {
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut locks: Vec<LockKey> = vec![];
    let mut keys: Vec<LockKey> = vec![];
    let mut wip_lock_key: LockKey = LockKey::default();
    let mut y = 0;
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        println!("line: {puzzle_line}");
        if puzzle_line.len() == 0 || y == KEY_HEIGHT {
            y = 0;
            if wip_lock_key.is_lock {
                locks.push(wip_lock_key);
            } else {
                keys.push(wip_lock_key);
            }
            wip_lock_key = LockKey::default();
            println!("------------------------");
            continue;
        }
        if y == 0 && puzzle_line == "#####" {
            // wip_lock_key.tumbler[y] = [true; KEY_WIDTH];
            wip_lock_key.is_lock = true;
        } else if y == 0 && puzzle_line == "....." {
            // wip_lock_key.tumbler[y] = [false; KEY_WIDTH];
            wip_lock_key.is_lock = false;
        } else {
            let row: [bool; KEY_WIDTH] = puzzle_line.chars().map(|c| c == '#').collect::<Vec<_>>().try_into().expect("failed");
            // wip_lock_key.tumbler[y] = row;
            if y != 0 && y != KEY_HEIGHT - 1 {
                for (i, &e) in row.iter().enumerate() {
                    if e {
                        wip_lock_key.pin_heights[i] += 1;
                    }
                }
            }
        }
        y += 1;
        if y == KEY_HEIGHT {
            println!("{wip_lock_key:?}");
        }
    }
    if wip_lock_key.is_lock {
        locks.push(wip_lock_key);
    } else {
        keys.push(wip_lock_key);
    }
    println!("------------------------");

    let matches = count_key_lock_matches(&*keys, &*locks, |key, lock| {key.cmp(lock) == Ordering::Equal});
    println!("matches: {matches}");
}

fn count_key_lock_matches(
    keys: &[LockKey],
    locks: &[LockKey],
    validate: impl Fn(&LockKey, &LockKey) -> bool,
) -> usize {
    let mut match_count = 0;
    for key in keys {
        for lock in locks {
            if validate(key, lock) {
                match_count += 1;
            }
        }
    }
    match_count
}