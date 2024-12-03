use regex::Regex;
use std::io;
use std::io::Read;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).expect("Didn't get input");
    let program = input.replace('\n', "");
    let r_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let r_do = Regex::new(r"do\(\)").unwrap();
    let r_dont = Regex::new(r"don't\(\)").unwrap();
    let mut sum = 0;

    let mut indicies_do: Vec<usize> = r_do.find_iter(&*program).map(|m| m.start()).collect();
    let mut indicies_dont: Vec<usize> = r_dont.find_iter(&*program).map(|m| m.start()).collect();
    let mut mask: Vec<bool> = Vec::new();
    let mut do_iter = indicies_do.iter().peekable();
    let mut dont_iter = indicies_dont.iter().peekable();
    let mut last_do_index = *do_iter.next().unwrap_or(&0);
    let mut last_dont_index = *dont_iter.next().unwrap_or(&program.len());
    let mut set_to = true;
    for i in 0..program.len() {
        mask.push(set_to);
        if i == last_do_index {
            set_to = true;
            last_do_index = *do_iter.next().unwrap_or(&program.len());
        }
        if i == last_dont_index {
            set_to = false;
            last_dont_index = *dont_iter.next().unwrap_or(&program.len());
        }
    }

    for i in 0..program.len() {
        if mask[i] == false {
            print!("0");
        } else {
            print!("1");
        }
    }
    println!();
    println!("{}", &*program);
    let mul_cm = r_mul.captures_iter(&*program);
    for mul_caps in mul_cm {
        let a: Vec<_> = mul_caps.iter().map(|m| m.unwrap().as_str()).collect();
        println!("{}", a.join("|"));
        for pair in mul_caps.iter().skip(1).zip(mul_caps.iter().skip(2)) {
            if let (Some(left), Some(right)) = pair {
                if mask[right.start()] {
                    let left_num = left.as_str().parse::<i32>().unwrap();
                    let right_num = right.as_str().parse::<i32>().unwrap();
                    sum += left_num * right_num;
                }
            }
        }
    }

    println!("sum: {}", sum);
}


