use std::env;
use std::io::{BufReader, prelude::*};
use std::fs::File;

enum CaveType {
    Start,
    End,
    Small,
    Large,
}

struct Cave {
    name: String,
    typ: CaveType,
    visited: bool,
    visits: usize,
}

impl Cave {
    fn new(name: &str) -> Self {
        let typ = match name {
            "start" => CaveType::Start,
            "end" => CaveType::End,
            _ => if name == name.to_uppercase() {
                CaveType::Large
            } else {
                CaveType::Small
            }
        };

        Self {
            name: String::from(name),
            typ,
            visited: false,
            visits: 0,
        }
    }
}

struct CaveSystem {
    caves: Vec<Cave>,
    connections: Vec<(usize, usize)>,
}

impl CaveSystem {
    fn new(fname: &str) -> Self {
        let f = File::open(fname).unwrap();
        let reader = BufReader::new(f);

        let mut caves: Vec<Cave> = Vec::new();
        let mut connections = Vec::new();

        for line in reader.lines().map(|l| l.unwrap()) {
            let split: Vec<&str> = line.split("-").collect();

            let first: usize = match (0..caves.len()).into_iter().find(|&i| caves[i].name == split[0]) {
                Some(i) => i,
                None => {
                    caves.push(Cave::new(split[0]));
                    caves.len() - 1
                }
            };

            let second: usize = match (0..caves.len()).into_iter().find(|&i| caves[i].name == split[1]) {
                Some(i) => i,
                None => {
                    caves.push(Cave::new(split[1]));
                    caves.len() - 1
                }
            };

            connections.push((first, second));
        }

        Self { caves, connections }
    }

    fn solve(&mut self) -> Vec<Vec<String>> {
        let start = (0..self.caves.len()).into_iter()
                                         .find(|&i| matches!(self.caves[i].typ, CaveType::Start))
                                         .unwrap();

        let mut path = Vec::new();
        let mut solutions = Vec::new();

        self.step(start, &mut path, &mut solutions);

        solutions
    }

    fn step(&mut self, current_cave: usize, path: &mut Vec<usize>, solutions: &mut Vec<Vec<String>>) {
        if matches!(self.caves[current_cave].typ, CaveType::End) {
            path.push(current_cave);
            solutions.push(path.iter().map(|&i| self.caves[i].name.clone()).collect());
            path.pop();
            return;
        }

        let possible: Vec<usize> = self.connections.iter()
                                                   .filter(|&&(i, j)| i == current_cave || j == current_cave)
                                                   .map(|&(i, j)| if i == current_cave { j } else { i })
                                                   .filter(|&i| match self.caves[i].typ {
                                                       CaveType::Start => false,
                                                       CaveType::Small => !self.caves[i].visited,
                                                       _               => true,
                                                   })
                                                   .collect();

        path.push(current_cave);
        self.caves[current_cave].visited = true;

        for i in possible.into_iter() {
            self.step(i, path, solutions);
        }

        path.pop();
        self.caves[current_cave].visited = false;
    }

    fn solve_part2(&mut self) -> Vec<Vec<String>> {
        let start = (0..self.caves.len()).into_iter()
                                         .find(|&i| matches!(self.caves[i].typ, CaveType::Start))
                                         .unwrap();

        let mut path = Vec::new();
        let mut solutions = Vec::new();

        self.step_part2(start, true, &mut path, &mut solutions);

        solutions
    }

    fn step_part2(&mut self,
                  current_cave: usize,
                  may_visit_small_twice: bool,
                  path: &mut Vec<usize>,
                  solutions: &mut Vec<Vec<String>>)
    {
        if matches!(self.caves[current_cave].typ, CaveType::End) {
            path.push(current_cave);
            solutions.push(path.iter().map(|&i| self.caves[i].name.clone()).collect());
            path.pop();
            return;
        }

        path.push(current_cave);
        self.caves[current_cave].visits += 1;

        let may_visit_small_twice =
            if matches!(self.caves[current_cave].typ, CaveType::Small) && self.caves[current_cave].visits == 2 {
                false
            } else {
                may_visit_small_twice
            };

        let possible: Vec<usize> = self.connections.iter()
                                                   .filter(|&&(i, j)| i == current_cave || j == current_cave)
                                                   .map(|&(i, j)| if i == current_cave { j } else { i })
                                                   .filter(|&i| match self.caves[i].typ {
                                                       CaveType::Start => false,
                                                       CaveType::Small => self.caves[i].visits == 0 || may_visit_small_twice,
                                                       _               => true,
                                                   })
                                                   .collect();

        for i in possible.into_iter() {
            self.step_part2(i, may_visit_small_twice, path, solutions);
        }

        path.pop();
        self.caves[current_cave].visits -= 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cave_system = CaveSystem::new(&args[1]);

    let solutions = cave_system.solve();
    // for s in solutions.iter() {
    //     println!("{}", s.join(","));
    // }
    println!("Part 1: {} total solutions", solutions.len());
    println!();

    let solutions_part2 = cave_system.solve_part2();
    // for s in solutions_part2.iter() {
    //     println!("{}", s.join(","));
    // }
    println!("Part 2: {} total solutions", solutions_part2.len());
}
