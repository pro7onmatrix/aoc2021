use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
    Empty,
}

impl Amphipod {
    fn new(c: char) -> Self {
        match c {
            'A' => Amphipod::Amber,
            'B' => Amphipod::Bronze,
            'C' => Amphipod::Copper,
            'D' => Amphipod::Desert,
            _ => panic!(),
        }
    }
}

struct BoardState<const N: usize> {
    hallway: [Amphipod; 11],
    rooms: [[Amphipod; N]; 4],
}

impl<const N: usize> BoardState<N> {
    fn new(amphipods: &[Amphipod]) -> Self {
        let hallway = [Amphipod::Empty; 11];
        let mut rooms = [[Amphipod::Empty; N]; 4];

        for i in 0..N {
            rooms[0][i] = amphipods[4*i  ];
            rooms[1][i] = amphipods[4*i+1];
            rooms[2][i] = amphipods[4*i+2];
            rooms[3][i] = amphipods[4*i+3];
        }

        Self { hallway, rooms }
    }
}

fn main() {

}
