use std::{io, thread};
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

/**
ADV(0) - COMBO - division numerator in ra, 2^combo denominator, trunc int, store ra
BXL(1) - LITERAL - bitwise XOR of rb and literal operand, store rb
BST(2) - COMBO - takes combo operand modulo 8 (bitmask 3 lowest), store rb
JNZ(3) - LITERAL - if ra is 0 nop, else jump by setting instruction_pointer to literal operand, do not inc instruction_pointer
BXC(4) - IGNORE - bitwise XOR or rb and rc, ignores operand, store rb
OUT(5) - COMBO - calc value of combo operand modulo 8, the outputs the value (when multiple OUT comma separate)
BDV(6) - COMBO - ADV(0) but store in rb, still read ra
CDV(7) - COMBO - ADV(0) but store in rc, still read ra
*/
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum Instruction {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Instruction::ADV,
            1 => Instruction::BXL,
            2 => Instruction::BST,
            3 => Instruction::JNZ,
            4 => Instruction::BXC,
            5 => Instruction::OUT,
            6 => Instruction::BDV,
            7 => Instruction::CDV,
            _ => { panic!("Illegal Instruction") }
        }
    }
}

#[derive(Default, Clone, Debug)]
struct Computer {
    ra: usize,
    rb: usize,
    rc: usize,
    /** +2 after every instruction except jnz*/
    instruction_ptr: usize,
    /** opcode,operand,opcode,operand */
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Computer {
    fn is_copy(&self, other: &Computer) -> bool {
        self.program == other.output
    }
    fn opcode(&self) -> Instruction {
        Instruction::from(self.program[self.instruction_ptr])
    }
    fn operand(&self) -> u8 {
        self.program[self.instruction_ptr + 1]
    }

    fn literal_operand(&self) -> usize {
        self.program[self.instruction_ptr + 1] as usize
    }

    fn combo_operand(&self) -> usize {
        match self.program[self.instruction_ptr + 1] {
            l @ 0..=3 => l as usize,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            7 => panic!("reserved"),
            _ => panic!("illegal operand")
        }
    }
}


fn main() {
    let mut output: String = String::new();
    let mut puzzle_lines = io::stdin().lock().lines();
    let mut computer: Computer = Computer::default();
    while let Some(Ok(puzzle_line)) = puzzle_lines.next() {
        match puzzle_line {
            a if a.starts_with("Register A:") => {
                let ra = a.split_at(12).1.parse::<usize>().unwrap();
                println!("a: {ra}");
                computer.ra = ra;
            }
            b if b.starts_with("Register B:") => {
                let rb = b.split_at(12).1.parse::<usize>().unwrap();
                println!("b: {rb}");
                computer.rb = rb;
            }
            c if c.starts_with("Register C:") => {
                let rc = c.split_at(12).1.parse::<usize>().unwrap();
                println!("c: {rc}");
                computer.rc = rc;
            }
            prog if prog.len() != 0 => {
                let u8prog: Vec<u8> = prog.split_at(9).1.split(',').map(|c| c.parse::<u8>().unwrap()).collect();
                println!("program {:?}", u8prog);
                computer.program = u8prog;
            }
            _ => continue
        }
    }

    let instr_len = computer.program.len();
    println!("instr_len({instr_len}) instruction_ptr({})", computer.instruction_ptr);
    let num_threads = 14;
    let start_vra = 45_185_062_144_410usize;
    let start_vra = 281_474_976_710_656usize;
    let mut handles = Vec::with_capacity(num_threads);
    let found_match = Arc::new(AtomicBool::new(false));
    let matches = Arc::new(Mutex::new(Vec::new()));;

    for thread_id in 0..num_threads {
        let computer_clone = computer.clone();
        let thread_start_vra = start_vra + thread_id; // Stagger start values
        println!("thread_start_vra {thread_start_vra}");
        let found_match = Arc::clone(&found_match);
        let mut matches = Arc::clone(&matches);
        handles.push(thread::spawn(move || {
            let ogcomputer = computer_clone.clone();
            let mut computer = computer_clone.clone();
            let mut vra = thread_start_vra;

            'outer: loop {
                let mut computer = computer.clone(); // Important: clone for each iteration
                computer.ra = vra;
                // println!("({thread_id}) start {:?}", computer);
                if found_match.load(Ordering::Relaxed) {
                    break;
                }
                // thread::sleep(Duration::from_millis(500));
                // print!("({thread_id}) vra({vra})");
                while computer.instruction_ptr + 1 < instr_len {
                    // Removed println! inside the loop for performance in parallel execution
                    match computer.opcode() {
                        Instruction::ADV => {
                            computer.ra = computer.ra / (2u32.pow(computer.combo_operand() as u32) as usize);
                        }
                        Instruction::BDV => {
                            computer.rb = computer.ra / (2u32.pow(computer.combo_operand() as u32) as usize);
                        }
                        Instruction::CDV => {
                            computer.rc = computer.ra / (2u32.pow(computer.combo_operand() as u32) as usize);
                        }
                        Instruction::BXL => {
                            computer.rb = computer.rb ^ computer.literal_operand();
                        }
                        Instruction::BST => {
                            computer.rb = computer.combo_operand() % 8;
                        }
                        Instruction::JNZ if computer.ra != 0 => {
                            computer.instruction_ptr = computer.literal_operand();
                            continue;
                        }
                        Instruction::BXC => {
                            computer.rb = computer.rb ^ computer.rc;
                        }
                        Instruction::OUT => {
                            // println!("{:?}", computer);
                            computer.output.push((computer.combo_operand() % 8) as u8);
                            let og_prog_len = ogcomputer.program.len();
                            let current_output_len = computer.output.len();
                            let diff = og_prog_len as isize - current_output_len as isize;
                            if current_output_len <= og_prog_len {
                                 if ogcomputer.program[current_output_len - 1] != computer.output[current_output_len - 1] {
                                     if current_output_len >= 10 || diff < 0 {
                                         println!("({thread_id}) was close ({diff}) with vra {vra} but was {:?}", computer);
                                     }
                                    // aborted early for different outputs
                                    break;
                                }
                            }
                            if current_output_len >= 10 || diff < 0 {
                                println!("({thread_id}) was close ({diff}) with vra {vra} but was {:?}", computer);
                            }
                        }
                        _ => {}
                    }
                    computer.instruction_ptr += 2;
                }
                if ogcomputer.is_copy(&computer) { // Check against original clone
                    println!("Thread {} found a match! vra: {}", thread_id, vra);
                    let mut matches = matches.lock().unwrap();
                    matches.push(vra);
                    found_match.store(true, Ordering::Relaxed);
                    break 'outer; // Break the inner loop, continue to the next vra
                }
                vra += num_threads; // Increment by the number of threads for even distribution
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let matches = matches.lock().unwrap();
    let lowest_match = matches.iter().min().unwrap();
    println!("smallest match was ra: {lowest_match}");

    println!("computer: {computer:#?}");
    let out2 = computer.output.iter()
        .map(|num| num.to_string()) // Convert each usize to String
        .collect::<Vec<String>>() // Collect into a Vec<String>
        .join(",");
    println!("output: {output:?} or [{out2}]");
}
