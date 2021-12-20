use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

struct Transmission {
    version: u8,
    type_id: u8,
    value: u64,
    subpackets: Vec<Transmission>,
}

impl Transmission {
    fn new(input: &str) -> Self {
        let (transmission, _) = parse_bits(input);
        transmission
    }

    fn version_sum(&self) -> u32 {
        let mut sum = self.version as u32;

        for subpacket in self.subpackets.iter() {
            sum += subpacket.version_sum();
        }

        sum
    }

    fn evaluate(&self) -> u64 {
        let values: Vec<u64> = self.subpackets.iter().map(|sub| sub.evaluate()).collect();

        match self.type_id {
            0 => values.iter().sum(),
            1 => values.iter().product(),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            4 => self.value,
            5 => (values[0] > values[1]) as u64,
            6 => (values[0] < values[1]) as u64,
            7 => (values[0] == values[1]) as u64,
            _ => panic!("Invalid type ID!"),
        }
    }

    fn print_transmission(&self) {
        self.print_helper(0);
    }

    fn print_helper(&self, depth: u32) {
        if self.type_id == 4 {
            println!("Literal, version: {}, value: {}", self.version, self.value);
        } else {
            println!("Operator, version: {}, subpackets: {}", self.version, self.subpackets.len());
            for subpacket in self.subpackets.iter() {
                for _ in 0..depth+1 {
                    print!("  ");
                }
                subpacket.print_helper(depth + 1);
            }
        }
    }
}

fn read_input(fname: &str) -> io::Result<String> {
    let f = File::open(fname)?;
    let mut reader = BufReader::new(f);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;

    let output = buf.chars().map(|c| match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }).collect::<String>();

    Ok(output)
}

fn parse_bits(input: &str) -> (Transmission, String) {
    let version = u8::from_str_radix(&input[0..3], 2).unwrap();
    let type_id = u8::from_str_radix(&input[3..6], 2).unwrap();

    if type_id == 4 {
        // Literal value
        let mut value = 0;
        let mut offset = 6;

        loop {
            value = (value << 4) | u64::from_str_radix(&input[offset+1..offset+5], 2).unwrap();

            if input[offset..].starts_with('0') {
                break;
            }

            offset += 5;
        }

        let transmission = Transmission {
            version,
            type_id,
            value,
            subpackets: Vec::new()
        };

        return (transmission, String::from(&input[offset+5..]));
    } else {
        // Operator
        let length_type_id = u8::from_str_radix(&input[6..7], 2).unwrap();
        let mut subpackets = Vec::new();

        let remaining_input;

        if length_type_id == 0 {
            // 15 bit number with length of subpackets
            let subpack_length = usize::from_str_radix(&input[7..22], 2).unwrap();

            let mut subpack_input = String::from(&input[22..22+subpack_length]);
            while !subpack_input.is_empty() {
                let subpack = parse_bits(&subpack_input);
                subpackets.push(subpack.0);
                subpack_input = subpack.1;
            }

            remaining_input = String::from(&input[22+subpack_length..]);
        } else {
            // 11 bit number with number of subpackets
            let subpack_num = usize::from_str_radix(&input[7..18], 2).unwrap();

            let mut subpack_input = String::from(&input[18..]);
            while subpackets.len() < subpack_num {
                let subpack = parse_bits(&subpack_input);
                subpackets.push(subpack.0);
                subpack_input = subpack.1;
            }

            remaining_input = subpack_input;
        }

        let transmission = Transmission {
            version,
            type_id,
            value: 0,
            subpackets
        };

        return (transmission, remaining_input);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = read_input(&args[1]).unwrap();

    let t = Transmission::new(&code);
    // t.print_transmission();
    println!("Sum of versions: {}", t.version_sum());
    println!("Result: {}", t.evaluate());
}
