use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

struct Image {
    nrows: usize,
    ncols: usize,
    data: Vec<Vec<char>>,
    algorithm: Vec<char>,
    boundary_lit: bool,
}

impl Image {
    fn new(data: Vec<Vec<char>>, algorithm: Vec<char>) -> Self {
        let nrows = data.len();
        let ncols = data[0].len();

        Self { nrows, ncols, data, algorithm, boundary_lit: false }
    }

    fn enhance(&mut self) {
        let mut new_data = vec![vec!['\0'; self.ncols + 2]; self.nrows + 2];
        let mut indices  = vec![vec![if self.boundary_lit { 1 } else { 0 }; self.ncols + 2]; self.nrows + 2];

        for i in 0..self.nrows {
            for j in 0..self.ncols {
                indices[i+1][j+1] = pixel_to_int(self.data[i][j]);
            }
        }

        // Original image
        for i in 1..=self.nrows {
            for j in 1..=self.ncols {
                let index = (indices[i-1][j-1] as usize) << 8
                          | (indices[i-1][j  ] as usize) << 7
                          | (indices[i-1][j+1] as usize) << 6
                          | (indices[i  ][j-1] as usize) << 5
                          | (indices[i  ][j  ] as usize) << 4
                          | (indices[i  ][j+1] as usize) << 3
                          | (indices[i+1][j-1] as usize) << 2
                          | (indices[i+1][j  ] as usize) << 1
                          | (indices[i+1][j+1] as usize);

                new_data[i][j] = self.algorithm[index];
            }
        }

        // Top and bottom edges
        for j in 0..=self.ncols+1 {
            // Boundary lit: Top index 111111***, bottom index ***111111
            // otherwise:    Top index 000000***, bottom index ***000000
            let mut top_index    = if self.boundary_lit { 0b111111000 } else { 0 };
            let mut bottom_index = if self.boundary_lit { 0b000111111 } else { 0 };

            if j > 0 {
                top_index    |= (indices[1][j-1] as usize) << 2;
                bottom_index |= (indices[self.nrows][j-1] as usize) << 8;
            } else if self.boundary_lit {
                top_index    |= 1 << 2;
                bottom_index |= 1 << 8;
            }

            top_index    |= (indices[1][j] as usize) << 1;
            bottom_index |= (indices[self.nrows][j] as usize) << 7;

            if j < self.ncols + 1 {
                top_index    |= indices[1][j+1] as usize;
                bottom_index |= (indices[self.nrows][j+1] as usize) << 6;
            } else if self.boundary_lit {
                top_index    |= 1;
                bottom_index |= 1 << 6;
            }

            new_data[0][j]            = self.algorithm[top_index];
            new_data[self.nrows+1][j] = self.algorithm[bottom_index];
        }

        // Left and right edges
        for i in 0..=self.nrows+1 {
            // Boundary lit: Left index 11*11*11*, right index *11*11*11
            // otherwise:    Left index 00*00*00*, right index *00*00*00
            let mut left_index  = if self.boundary_lit { 0b110110110 } else { 0 };
            let mut right_index = if self.boundary_lit { 0b011011011 } else { 0 };

            if i > 0 {
                left_index  |= (indices[i-1][1] as usize) << 6;
                right_index |= (indices[i-1][self.ncols] as usize) << 8;
            } else if self.boundary_lit {
                left_index  |= 1 << 6;
                right_index |= 1 << 8;
            }

            left_index  |= (indices[i][1] as usize) << 3;
            right_index |= (indices[i][self.ncols] as usize) << 5;

            if i < self.nrows + 1 {
                left_index  |= indices[i+1][1] as usize;
                right_index |= (indices[i+1][self.ncols] as usize) << 2;
            } else if self.boundary_lit {
                left_index  |= 1;
                right_index |= 1 << 2;
            }

            new_data[i][0]            = self.algorithm[left_index];
            new_data[i][self.ncols+1] = self.algorithm[right_index];
        }

        // Remove empty edges
        if new_data.iter().all(|row| row[self.ncols+1] == '.') {
            for row in new_data.iter_mut() {
                row.pop();
            }
        }
        if new_data.iter().all(|row| row[0] == '.') {
            for row in new_data.iter_mut() {
                row.remove(0);
            }
        }
        if new_data[self.nrows+1].iter().all(|&c| c == '.') {
            new_data.pop();
        }
        if new_data[0].iter().all(|&c| c == '.') {
            new_data.remove(0);
        }

        self.nrows = new_data.len();
        self.ncols = new_data[0].len();
        self.data  = new_data;

        if self.boundary_lit {
            self.boundary_lit = self.algorithm[511] == '#';
        } else {
            self.boundary_lit = self.algorithm[0] == '#';
        }
    }

    fn count_lit_pixels(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count())
            .sum()
    }
}

fn read_input(fname: &str) -> io::Result<Image> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut algorithm: Vec<char> = Vec::with_capacity(512);
    let mut data: Vec<Vec<char>> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            algorithm.extend(line?.chars());
        } else if i > 1 {
            data.push(line?.chars().collect());
        }
    }

    Ok(Image::new(data, algorithm))
}

fn pixel_to_int(pixel: char) -> u8 {
    match pixel {
        '.' => 0,
        '#' => 1,
        _ => panic!("Invalid pixel value!"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut image = read_input(&args[1]).unwrap();

    for _ in 0..2 {
        image.enhance();
    }

    println!("After  2 iterations, {} pixels are lit", image.count_lit_pixels());

    for _ in 0..48 {
        image.enhance();
    }

    println!("After 50 iterations, {} pixels are lit", image.count_lit_pixels());
}
