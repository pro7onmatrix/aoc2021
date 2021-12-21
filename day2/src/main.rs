use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn read_input(fname: &str) -> io::Result<Vec<Command>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut commands = Vec::new();

    for line in reader.lines() {
        let l = line?;
        let s: Vec<&str> = l.split_whitespace().collect();
        let value: i32   = s[1].parse().unwrap();

        match s[0] {
            "forward" => commands.push(Command::Forward(value)),
            "down" => commands.push(Command::Down(value)),
            "up" => commands.push(Command::Up(value)),
            _ => panic!("Illegal command '{}'!", s[0]),
        }
    }

    Ok(commands)
}

fn dive1(commands: &[Command]) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;

    for command in commands.iter() {
        match command {
            Command::Forward(amount) => horizontal += amount,
            Command::Down(amount) => depth += amount,
            Command::Up(amount) => depth -= amount,
        }
    }

    (horizontal, depth)
}

fn dive2(commands: &[Command]) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands.iter() {
        match command {
            Command::Forward(amount) => {
                horizontal += amount;
                depth += aim * amount;
            },
            Command::Down(amount) => aim += amount,
            Command::Up(amount) => aim -= amount,
        }
    }

    (horizontal, depth)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let commands = read_input(&args[1]).unwrap();

    let (h1, d1) = dive1(&commands);
    println!("Horizontal position: {}, depth: {} -> Result: {}", h1, d1, h1 * d1);

    let (h2, d2) = dive2(&commands);
    println!("Horizontal position: {}, depth: {} -> Result: {}", h2, d2, h2 * d2);
}
