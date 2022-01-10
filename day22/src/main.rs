use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use regex::Regex;

mod quadrilateral;
use quadrilateral::Quadrilateral;

#[derive(Debug)]
struct RebootInstruction {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    zmin: i32,
    zmax: i32,
    on: bool,
}

impl RebootInstruction {
    fn new(xmin: i32, xmax: i32, ymin: i32, ymax: i32, zmin: i32, zmax: i32, on: bool) -> Self {
        Self { xmin, xmax, ymin, ymax, zmin, zmax, on }
    }
}

fn read_input(fname: &str) -> io::Result<Vec<RebootInstruction>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let re = Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();

    let mut instructions = Vec::new();

    for line in reader.lines() {
        let l = line?;
        let caps = re.captures(&l).unwrap();

        let xmin = caps[2].parse::<i32>().unwrap();
        let xmax = caps[3].parse::<i32>().unwrap();
        let ymin = caps[4].parse::<i32>().unwrap();
        let ymax = caps[5].parse::<i32>().unwrap();
        let zmin = caps[6].parse::<i32>().unwrap();
        let zmax = caps[7].parse::<i32>().unwrap();

        let instruction = match &caps[1] {
            "on" => RebootInstruction::new(xmin, xmax, ymin, ymax, zmin, zmax, true),
            "off" => RebootInstruction::new(xmin, xmax, ymin, ymax, zmin, zmax, false),
            _ => panic!(),
        };

        instructions.push(instruction);
    }

    Ok(instructions)
}

fn reboot(instructions: &[RebootInstruction], restrict: bool) -> usize {
    let mut on_regions: Vec<Quadrilateral> = Vec::new();

    for instruction in instructions.iter() {
        for i in (0..on_regions.len()).rev() {
            if on_regions[i].is_empty() {
                on_regions.swap_remove(i);
            }
        }

        let new_region = if restrict {
            if let Some(region) = Quadrilateral::from(instruction).restrict(-50, 50, -50, 50, -50, 50) {
                region
            } else {
                continue;
            }
        } else {
            Quadrilateral::from(instruction)
        };

        for region in on_regions.iter_mut() {
            region.intersect(&new_region);
        }

        if instruction.on {
            on_regions.push(new_region);
        }
    }

    on_regions.iter()
              .fold(0, |acc, region| acc + region.volume() as usize)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let instructions = read_input(&args[1]).unwrap();
    println!("After local reboot, {} cubes are on", reboot(&instructions, true));
    println!("After global reboot, {} cubes are on", reboot(&instructions, false));
}
