use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use std::collections::{HashSet, HashMap};
use lazy_static::lazy_static;

lazy_static! {
    static ref CODES: HashMap<&'static str, i32> = vec![
        ("abcefg",  0),
        ("cf",      1),
        ("acdeg",   2),
        ("acdfg",   3),
        ("bcdf",    4),
        ("abdfg",   5),
        ("abdefg",  6),
        ("acf",     7),
        ("abcdefg", 8),
        ("abcdfg",  9)
    ].into_iter().collect();
}

fn count_1478(fname: &str) -> io::Result<usize> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let unique_lengths: HashSet<usize> = HashSet::from([2, 3, 4, 7]);
    let mut n = 0;

    for line in reader.lines() {
        let l = line?;
        let split: Vec<&str> = l.split(" | ").collect();
        let output = split[1];

        n += output.split_whitespace()
                   .filter(|s| unique_lengths.contains(&s.len()))
                   .count();
    }

    Ok(n)
}

fn code_to_digit(code: &str, map: &HashMap<char, char>) -> i32 {
    let mut possible: Vec<&str> = CODES.keys()
                                       .filter(|s| s.len() == code.len())
                                       .cloned()
                                       .collect();
    let mut chars = code.chars();

    while possible.len() > 1 {
        let c = chars.next().unwrap();
        possible.retain(|n| n.contains(map[&c]));
    }

    CODES[possible[0]]
}

fn decode(samples: &Vec<&str>, codes: &Vec<&str>) -> i32 {
    let mut one_str  = String::new();
    let mut four_str = String::new();

    for s in samples.iter() {
        match s.len() {
            2 => one_str = String::from(*s),
            4 => four_str = String::from(*s),
            _ => {},
        }
    }

    let mut map: HashMap<char, char> = HashMap::new();

    //  aaaa
    // b    c
    // b    c
    //  dddd
    // e    f
    // e    f
    //  gggg

    for sample in samples.iter() {
        for c in sample.chars() {
            if map.contains_key(&c) {
                continue;
            }

            let missing = samples.iter().filter(|s| !s.contains(c)).count();

            match missing {
                1 => { map.insert(c, 'f'); },
                2 => if one_str.contains(c) {
                    map.insert(c, 'c');
                } else {
                    map.insert(c, 'a');
                },
                3 => if four_str.contains(c) {
                    map.insert(c, 'd');
                } else {
                    map.insert(c, 'g');
                },
                4 => { map.insert(c, 'b'); },
                6 => { map.insert(c, 'e'); },
                _ => {},
            }
        }
    }

    let mut value = 0;
    for code in codes.iter() {
        value = value * 10 + code_to_digit(&code, &map);
    }
    value
}

fn solve(fname: &str) -> io::Result<i32> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut result = 0;

    for line in reader.lines() {
        let l = line?;
        let split: Vec<&str> = l.split(" | ").collect();
        let samples: Vec<&str> = split[0].split_whitespace().collect();
        let codes: Vec<&str> = split[1].split_whitespace().collect();

        let n = decode(&samples, &codes);
        println!("{} {} {} {}: {}", codes[0], codes[1], codes[2], codes[3], n);

        result += n;
    }

    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let count = count_1478(&args[1]).unwrap();
    println!("Found {} occurences of 1, 4, 7, or 8", count);

    let result = solve(&args[1]).unwrap();
    println!("Sum of output values: {}", result);
}
