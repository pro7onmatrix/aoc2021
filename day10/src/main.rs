use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref CORRUPTION_SCORES: HashMap<char, i32> = vec![
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ].into_iter().collect();

    static ref COMPLETION_SCORES: HashMap<char, i64> = vec![
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4)
    ].into_iter().collect();
}

fn score_navigation(fname: &str) -> io::Result<(i32, i64)> {
    // Completion score needs 64 bit to prevent overflow!

    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut stack = Vec::new();
    let mut corruption_score  = 0;
    let mut completion_scores = Vec::new();

    for line in reader.lines() {
        stack.clear();

        let mut corrupted = false;

        for c in line?.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                _ => {
                    let target = stack.pop().unwrap();

                    if c != target {
                        corrupted = true;
                        corruption_score += CORRUPTION_SCORES[&c];
                        break;
                    }
                }
            }
        }

        if !corrupted {
            let mut completion_score = 0;
            while !stack.is_empty() {
                let c = stack.pop().unwrap();
                completion_score = completion_score * 5 + COMPLETION_SCORES[&c];
            }
            completion_scores.push(completion_score);
        }
    }

    completion_scores.sort();
    let completion_score = completion_scores[completion_scores.len() / 2];

    Ok((corruption_score, completion_score))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (corruption_score, completion_score) = score_navigation(&args[1]).unwrap();
    println!("Corruption score: {}", corruption_score);
    println!("Completion score: {}", completion_score);
}
