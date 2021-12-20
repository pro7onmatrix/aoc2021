use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

struct Point(i32, i32);

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn adjust(&mut self, minx: i32, miny: i32) {
        self.start.0 -= minx;
        self.start.1 -= miny;
        self.end.0 -= minx;
        self.end.1 -= miny;
    }

    fn add_to_grid(&self, grid: &mut Vec<Vec<i32>>, include_diagonals: bool) {
        if self.start.0 == self.end.0 {
            let x = self.start.0 as usize;

            let a = self.start.1.min(self.end.1) as usize;
            let b = self.start.1.max(self.end.1) as usize;

            for y in a..(b+1) {
                grid[y][x] += 1;
            }
        } else if self.start.1 == self.end.1 {
            let y = self.start.1 as usize;

            let a = self.start.0.min(self.end.0) as usize;
            let b = self.start.0.max(self.end.0) as usize;

            for x in a..(b+1) {
                grid[y][x] += 1;
            }
        } else if include_diagonals {
            let dx = (self.end.0 - self.start.0).abs();
            let dy = (self.end.1 - self.start.1).abs();

            if dx == dy {
                for i in 0..(dx+1) {
                    let x = if self.start.0 < self.end.0 {
                        self.start.0 + i
                    } else {
                        self.start.0 - i
                    } as usize;

                    let y = if self.start.1 < self.end.1 {
                        self.start.1 + i
                    } else {
                        self.start.1 - i
                    } as usize;
                    grid[y][x] += 1;
                }
            }
        }
    }
}

fn read_coords(line: &str) -> (Point, Point) {
    let s: Vec<&str> = line.split_whitespace().collect();
    let p1: Vec<i32> = s[0].split(',').map(|n| n.parse().unwrap()).collect();
    let p2: Vec<i32> = s[2].split(',').map(|n| n.parse().unwrap()).collect();

    (Point(p1[0], p1[1]), Point(p2[0], p2[1]))
}

fn find_min_max(minx: &mut i32, maxx: &mut i32, miny: &mut i32, maxy: &mut i32, point: &Point) {
    *minx = point.0.min(*minx);
    *maxx = point.0.max(*maxx);
    *miny = point.1.min(*miny);
    *maxy = point.1.max(*maxy);
}

fn draw_grid(grid: &Vec<Vec<i32>>) {
    for row in grid.iter() {
        for n in row.iter() {
            if *n == 0 {
                print!(".");
            } else {
                print!("{}", n);
            }
        }
        println!();
    }
}

fn analyze_lines(fname: &str, include_diagonals: bool) -> io::Result<usize> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut lines: Vec<Line> = Vec::new();
    let mut grid: Vec<Vec<i32>> = Vec::new();

    let mut minx = i32::MAX;
    let mut maxx = i32::MIN;
    let mut miny = i32::MAX;
    let mut maxy = i32::MIN;

    for line in reader.lines() {
        let (p1, p2) = read_coords(&line?);

        find_min_max(&mut minx, &mut maxx, &mut miny, &mut maxy, &p1);
        find_min_max(&mut minx, &mut maxx, &mut miny, &mut maxy, &p2);

        lines.push(Line::new(p1, p2));
    }

    let nrows = (maxy + 1 - miny) as usize;
    let ncols = (maxx + 1 - minx) as usize;

    grid.resize(nrows, vec![0; ncols]);

    for line in lines.iter_mut() {
        line.adjust(minx, miny);
        line.add_to_grid(&mut grid, include_diagonals);
    }

    if nrows <= 20 && ncols <= 20 {
        draw_grid(&grid);
    }

    let count = grid.iter()
                    .map(|row| row.iter().filter(|&&x| x >= 2).count())
                    .sum();

    Ok(count)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let n = analyze_lines(&args[1], false).unwrap();
    println!("{} spots with two lines or more", n);
    println!();

    let m = analyze_lines(&args[1], true).unwrap();
    println!("{} spots with two lines or more including diagonals", m);
}
