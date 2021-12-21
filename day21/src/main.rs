use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

#[derive(Copy, Clone)]
struct Player {
    score: u32,
    position: u32,
}

impl Player {
    fn new(position: u32) -> Self {
        // Positions are read in as 1-based
        Self { position: position - 1, score: 0 }
    }

    fn step(&mut self, steps: u32) {
        self.position = (self.position + steps) % 10;
        self.score += self.position + 1;
    }

    fn reset(&mut self, position: u32, score: u32) {
        self.position = position;
        self.score = score;
    }

    fn has_won(&self, target: u32) -> bool {
        self.score >= target
    }
}

fn read_input(fname: &str) -> io::Result<Vec<Player>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut players = Vec::new();

    for line in reader.lines() {
        let position = line?.split_whitespace()
                            .last()
                            .unwrap()
                            .parse()
                            .unwrap();

        players.push(Player::new(position));
    }

    Ok(players)
}

fn play_deterministric(players: &[Player]) -> u32 {
    let mut players = [players[0].clone(), players[1].clone()];

    let mut die = 0;
    let mut die_rolls = 0;

    loop {
        for i in 0..2 {
            die_rolls += 3;

            let roll1 = die % 100 + 1;
            let roll2 = (die + 1) % 100 + 1;
            let roll3 = (die + 2) % 100 + 1;

            die = (die + 3) % 100;

            players[i].step(roll1 + roll2 + roll3);

            if players[i].has_won(1000) {
                return players[i^1].score * die_rolls;
            }
        }
    }
}

fn play_dirac(players: &[Player]) -> usize {
    let mut players = [players[0].clone(), players[1].clone()];
    let [wins_player1, wins_player2] = dirac_helper(&mut players, 1, 0);
    wins_player1.max(wins_player2)
}

// Multipliers for step sizes => how many ways three three-sided
// dice can add up to a given number
const MULTIPLIERS: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

fn dirac_helper(players: &mut [Player; 2], multiplier: usize, turn: usize) -> [usize; 2] {
    let mut wins = [0, 0];

    for step_size in 3..=9 {
        let next_multiplier = multiplier * MULTIPLIERS[step_size as usize];

        let prev_position = players[turn].position;
        let prev_score = players[turn].score;

        players[turn].step(step_size);

        if players[turn].has_won(21) {
            wins[turn] += next_multiplier;
        } else {
            let [wins_p1, wins_p2] = dirac_helper(players, next_multiplier, turn^1);
            wins[0] += wins_p1;
            wins[1] += wins_p2;
        }

        players[turn].reset(prev_position, prev_score);
    }

    wins
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let players = read_input(&args[1]).unwrap();

    let score_deterministic = play_deterministric(&players);
    println!("Score for deteministic game: {}", score_deterministic);

    let score_dirac = play_dirac(&players);
    println!("Score for Dirac game: {}", score_dirac);
}
