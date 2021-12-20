use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use std::collections::HashMap;
use itertools::Itertools;

fn read_input(fname: &str) -> io::Result<(String, HashMap<(char, char), char>)> {
    let f = File::open(fname)?;
    let mut reader = BufReader::new(f);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;

    let template = buf.replace("\n", "");
    reader.read_line(&mut buf)?;

    let mut rules = HashMap::new();

    for line in reader.lines() {
        // Format: "AB -> C" => Need indices 0, 1, and 6
        let l: Vec<char> = line?.chars().collect();
        rules.insert((l[0], l[1]), l[6]);
    }

    Ok((template, rules))
}

fn step(input: String, rules: &HashMap<(char, char), char>) -> String {
    let mut output = String::new();

    for (a, b) in input.chars().zip(input.chars().skip(1)) {
        output.push(a);
        output.push(rules[&(a, b)]);
    }
    output.push(input.chars().last().unwrap());

    output
}

fn simulate(template: &String, rules: &HashMap<(char, char), char>, nsteps: usize) -> String {
    let mut s = template.clone();

    for _ in 0..nsteps {
        s = step(s, &rules);
    }

    s
}

fn calculate_result(s: &str) -> usize {
    let counts = s.chars().counts();

    let (min, max) = counts.values()
                           .fold((usize::MAX, usize::MIN), |(min, max), &n| (min.min(n), max.max(n)));

    max - min
}

fn polymerize(template: &str, rules: &HashMap<(char, char), char>, nsteps: usize) -> usize {
    let mut bucket = make_bucket(&template);

    for _ in 0..nsteps {
        bucket = next_bucket(bucket, rules);
    }

    let counts = count_elements(template, &bucket);
    let (min, max) = counts.values()
                           .fold((usize::MAX, usize::MIN), |(min, max), &n| (min.min(n), max.max(n)));

    max - min
}

fn make_bucket(template: &str) -> HashMap<(char, char), usize> {
    let mut result = HashMap::new();

    for pair in template.chars().zip(template.chars().skip(1)) {
        if let Some(n) = result.get_mut(&pair) {
            *n += 1;
        } else {
            result.insert(pair, 1);
        }
    }

    result
}

fn next_bucket(bucket: HashMap<(char, char), usize>,
               rules: &HashMap<(char, char), char>) -> HashMap<(char, char), usize>
{
    let mut new_bucket = HashMap::new();

    for (&pair, &value) in bucket.iter() {
        let (a, b) = pair;
        let c = rules[&pair];

        let first_pair  = (a, c);
        let second_pair = (c, b);

        if let Some(n) = new_bucket.get_mut(&first_pair) {
            *n += value;
        } else {
            new_bucket.insert(first_pair, value);
        }

        if let Some(n) = new_bucket.get_mut(&second_pair) {
            *n += value;
        } else {
            new_bucket.insert(second_pair, value);
        }
    }

    new_bucket
}

fn count_elements(template: &str, bucket: &HashMap<(char, char), usize>) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    for (&(a, b), &n) in bucket.iter() {
        if let Some(m) = result.get_mut(&a) {
            *m += n;
        } else {
            result.insert(a, n);
        }

        if let Some(m) = result.get_mut(&b) {
            *m += n;
        } else {
            result.insert(b, n);
        }
    }

    // All elements are counted double...
    for val in result.values_mut() {
        *val /= 2;
    }

    // ... except the first and last one
    let first = template.chars().nth(0).unwrap();
    let last  = template.chars().last().unwrap();
    *result.get_mut(&first).unwrap() += 1;
    *result.get_mut(&last).unwrap()  += 1;

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (template, rules) = read_input(&args[1]).unwrap();

    let part1 = polymerize(&template, &rules, 10);
    println!("Result for part 1: {}", part1);

    let part2 = polymerize(&template, &rules, 40);
    println!("Result for part 2: {}", part2);
}
