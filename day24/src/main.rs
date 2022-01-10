use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use std::str::Chars;

enum Instruction {
    Inp(char),
    Add(char, SecondOperand),
    Mul(char, SecondOperand),
    Div(char, SecondOperand),
    Mod(char, SecondOperand),
    Eql(char, SecondOperand),
}

impl std::convert::From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let fields: Vec<&str> = s.split_whitespace().collect();

        let reg1 = fields[1].chars().nth(0).unwrap();

        match fields[0] {
            "inp" => Instruction::Inp(reg1),
            "add" => if let Ok(value) = fields[2].parse() {
                Instruction::Add(reg1, SecondOperand::Immediate(value))
            } else {
                Instruction::Add(reg1, SecondOperand::Register(fields[2].chars().nth(0).unwrap()))
            },
            "mul" => if let Ok(value) = fields[2].parse() {
                Instruction::Mul(reg1, SecondOperand::Immediate(value))
            } else {
                Instruction::Mul(reg1, SecondOperand::Register(fields[2].chars().nth(0).unwrap()))
            },
            "div" => if let Ok(value) = fields[2].parse() {
                Instruction::Div(reg1, SecondOperand::Immediate(value))
            } else {
                Instruction::Div(reg1, SecondOperand::Register(fields[2].chars().nth(0).unwrap()))
            },
            "mod" => if let Ok(value) = fields[2].parse() {
                Instruction::Mod(reg1, SecondOperand::Immediate(value))
            } else {
                Instruction::Mod(reg1, SecondOperand::Register(fields[2].chars().nth(0).unwrap()))
            },
            "eql" => if let Ok(value) = fields[2].parse() {
                Instruction::Eql(reg1, SecondOperand::Immediate(value))
            } else {
                Instruction::Eql(reg1, SecondOperand::Register(fields[2].chars().nth(0).unwrap()))
            },
            _ => panic!("Invalid instruction!"),
        }
    }
}

impl std::convert::From<String> for Instruction {
    fn from(s: String) -> Self {
        Self::from(&s[..])
    }
}

enum SecondOperand {
    Register(char),
    Immediate(i64),
}

struct ALU<'a> {
    registers: [i64; 4],
    input: Chars<'a>,
}

impl<'a> ALU<'a> {
    fn new(input: &'a str) -> Self {
        Self { registers: [0; 4], input: input.chars() }
    }

    fn register_index(c: char) -> usize {
        c as usize - 'w' as usize
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Inp(target) => {
                // let mut buffer = String::new();
                // let stdin = io::stdin();
                // stdin.read_line(&mut buffer).unwrap();
                // let value = buffer.replace("\n", "").parse().unwrap();

                let value = if let Some(c) = self.input.next() {
                    c as i64 - '0' as i64
                } else {
                    panic!("No more input to read!");
                };

                self.registers[ALU::register_index(*target)] = value;
            },
            Instruction::Add(lhs, rhs) => {
                let rhs = match rhs {
                    SecondOperand::Register(reg) => self.registers[ALU::register_index(*reg)],
                    SecondOperand::Immediate(val) => *val,
                };
                self.registers[ALU::register_index(*lhs)] += rhs;
            },
            Instruction::Mul(lhs, rhs) => {
                let rhs = match rhs {
                    SecondOperand::Register(reg) => self.registers[ALU::register_index(*reg)],
                    SecondOperand::Immediate(val) => *val,
                };
                self.registers[ALU::register_index(*lhs)] *= rhs;
            },
            Instruction::Div(lhs, rhs) => {
                let rhs = match rhs {
                    SecondOperand::Register(reg) => self.registers[ALU::register_index(*reg)],
                    SecondOperand::Immediate(val) => *val,
                };
                if rhs == 0 {
                    panic!("Division by zero!");
                }
                self.registers[ALU::register_index(*lhs)] /= rhs;
            },
            Instruction::Mod(lhs, rhs) => {
                let rhs = match rhs {
                    SecondOperand::Register(reg) => self.registers[ALU::register_index(*reg)],
                    SecondOperand::Immediate(val) => *val,
                };
                let lhs = &mut self.registers[ALU::register_index(*lhs)];
                if *lhs < 0 || rhs < 0 {
                    panic!("Negative numbers in modulo operation!");
                }
                if rhs == 0 {
                    panic!("Division by zero!");
                }
                *lhs %= rhs;
            },
            Instruction::Eql(lhs, rhs) => {
                let rhs = match rhs {
                    SecondOperand::Register(reg) => self.registers[ALU::register_index(*reg)],
                    SecondOperand::Immediate(val) => *val,
                };
                let lhs = &mut self.registers[ALU::register_index(*lhs)];
                *lhs = (*lhs == rhs) as i64;
            },
        }
    }
}

fn read_input(fname: &str) -> io::Result<Vec<Instruction>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut instructions = Vec::new();

    for line in reader.lines() {
        instructions.push(Instruction::from(line?));
    }

    Ok(instructions)
}

fn is_model_number_valid(model_number: i64, monad: &[Instruction]) -> bool {
    let input = format!("{}", model_number);
    let mut alu = ALU::new(&input);

    for instruction in monad.iter() {
    // for (i, instruction) in monad.iter().enumerate() {
        alu.execute_instruction(instruction);
        // println!("Instruction {:3}: w={}, x={}, y={}, z={}", i+1, alu.registers[0], alu.registers[1], alu.registers[2], alu.registers[3]);
    }

    alu.registers[3] == 0
}

fn contains_zeros(number: i64) -> bool {
    // format!("{}", number).chars().find(|&c| c == '0').is_some()

    let mut fac = 10;
    let mut rem = 0;

    for _ in 0..14 {
        let m = number % fac;

        if m - rem == 0 {
            return true;
        }

        rem = m;
        fac *= 10;
    }

    false
}

fn find_highest_valid_model_number_brute_force(monad: &[Instruction]) -> i64 {
    let mut current = 100000000000000;
    let mut found_one = false;

    loop {
        current -= 1;

        if current % 1000000 == 0 {
            println!("{}", current);
        }

        if contains_zeros(current) {
            continue;
        }

        let is_valid = is_model_number_valid(current, monad);

        if is_valid {
            if !found_one {
                println!("Found the first valid number: {}", current);
            }
            // Keep going if the number is valid
            found_one = true;
        } else if found_one {
            // If it's invalid and we have seen a valid number before,
            // return that
            return current + 1;
        }
    }
}

fn monad(number: i64) -> i64 {
    let x_adds = [11, 11, 15, -11, 15, 15, 14, -7, 12, -6, -10, -15, -9, 0];
    let y_adds = [ 6, 12,  8,   7,  7, 12,  2, 15,  4,  5,  12,  11, 13, 7];

    let mut w;
    let mut x;
    let mut z = 0;

    let mut modulo = 10000000000000;

    for i in 0..14 {
        w = number % modulo / (10 * modulo);

        x = z % 26 + x_adds[i];

        if i > 8 || (i + 1) % 4 == 0 {
            z /= 26;
        }

        if x != w {
            z = 26 * z + w + y_adds[i];
        }

        modulo /= 10;
    }

    z
}

fn find_highest_valid_model_number() -> i64 {
    let mut current = 100000000000000;
    let mut found_one = false;

    let mut stdout = io::stdout();

    loop {
        current -= 1;

        if current % 1000000 == 0 {
            print!("\r{}", current);
            stdout.flush().unwrap();
        }

        if contains_zeros(current) {
            continue;
        }

        let is_valid = monad(current) == 0;

        if is_valid {
            if !found_one {
                println!();
                println!("Found the first valid number: {}", current);
            }
            // Keep going if the number is valid
            found_one = true;
        } else if found_one {
            // If it's invalid and we have seen a valid number before,
            // return that
            return current + 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let instructions = read_input(&args[1]).unwrap();

    let highest_valid = find_highest_valid_model_number();
    println!("Highest valid model number: {}", highest_valid);
}
