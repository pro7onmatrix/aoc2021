use crate::vec3::Vec3;

enum FaceCompare {
    Same,
    Flipped,
    Different,
}

pub struct Scanner {
    position: Vec3,
    nbeacons: usize,
    beacons: Vec<Vec3>,
    distance_matrix: Vec<Vec<i32>>,
}

impl Scanner {
    pub fn get_position(&self) -> &Vec3 {
        &self.position
    }

    pub fn get_beacons(&self) -> &[Vec3] {
        &self.beacons
    }

    pub fn find_overlap(&self, other: &Scanner) -> Option<(Vec<Vec3>, Vec<Vec3>)> {
        let mut overlap       = Vec::new();
        let mut overlap_other = Vec::new();

        for i in 0..self.nbeacons {
            for j in 0..other.nbeacons {
                if at_least_12_matches_in_rows(&self.distance_matrix[i], &other.distance_matrix[j]) {
                    overlap.push(self.beacons[i]);
                    overlap_other.push(other.beacons[j]);
                    break;
                }
            }
        }

        if overlap.is_empty() {
            None
        } else {
            Some((overlap, overlap_other))
        }
    }

    pub fn adjust_orientation(&mut self, beacons1: &[Vec3], beacons2: &[Vec3]) {
        let mut first = beacons2[0].clone();

        let mut changes1 = Vec::with_capacity(beacons1.len() - 1);
        let mut changes2 = Vec::with_capacity(beacons1.len() - 1);

        for i in 1..beacons1.len() {
            changes1.push(beacons1[i] - beacons1[i - 1]);
            changes2.push(beacons2[i] - beacons2[i - 1]);
        }

        // println!("========= BEFORE ==========");
        // for (a, b) in changes1.iter().zip(changes2.iter()) {
        //     println!("{}\t\t{}", a, b);
        // }
        // println!();

        match compare_directions(&changes1, 'x', &changes2, 'x') {
            FaceCompare::Same => {
                // Both face in the positive x direction => only find "up"
            },
            FaceCompare::Flipped => {
                // Facing in negative x direction => rotate 180° around y
                first.flip_xz();

                for beacon in self.beacons.iter_mut() {
                    beacon.flip_xz();
                }

                for c in changes2.iter_mut() {
                    c.flip_xz();
                }
            },
            FaceCompare::Different => {
                match compare_directions(&changes1, 'x', &changes2, 'y') {
                    FaceCompare::Same => {
                        // Facing towards positive y instead of positive x
                        // => rotate counterclockwise around z axis
                        first.rotate_z_counterclockwise();

                        for beacon in self.beacons.iter_mut() {
                            beacon.rotate_z_counterclockwise();
                        }

                        for c in changes2.iter_mut() {
                            c.rotate_z_counterclockwise();
                        }
                    },
                    FaceCompare::Flipped => {
                        // Facing towards negative y instead of positive x
                        // => rotate clockwise around z axis
                        first.rotate_z_clockwise();

                        for beacon in self.beacons.iter_mut() {
                            beacon.rotate_z_clockwise();
                        }

                        for c in changes2.iter_mut() {
                            c.rotate_z_clockwise();
                        }
                    },
                    FaceCompare::Different => {
                        match compare_directions(&changes1, 'x', &changes2, 'z') {
                            FaceCompare::Same => {
                                // Facing towards positive z instead of positive x
                                // => rotate clockwise around y axis
                                first.rotate_y_clockwise();

                                for beacon in self.beacons.iter_mut() {
                                    beacon.rotate_y_clockwise();
                                }

                                for c in changes2.iter_mut() {
                                    c.rotate_y_clockwise();
                                }
                            },
                            FaceCompare::Flipped => {
                                // Facing towards negative z instead of positive x
                                // => rotate counterclockwise around y axis
                                first.rotate_y_counterclockwise();

                                for beacon in self.beacons.iter_mut() {
                                    beacon.rotate_y_counterclockwise();
                                }

                                for c in changes2.iter_mut() {
                                    c.rotate_y_counterclockwise();
                                }
                            },
                            FaceCompare::Different => panic!("No shared facing direction?!"),
                        }
                    }
                }
            },
        }

        // Now both are facing the same x direction, make `self` define
        // positive z as "up"
        match compare_directions(&changes1, 'z', &changes2, 'z') {
            FaceCompare::Same => {
                // Already done
            },
            FaceCompare::Flipped => {
                // Defined up as negative z => rotate 180° around x axis
                first.flip_yz();

                for beacon in self.beacons.iter_mut() {
                    beacon.flip_yz();
                }

                for c in changes2.iter_mut() {
                    c.flip_yz();
                }
            },
            FaceCompare::Different => {
                match compare_directions(&changes1, 'z', &changes2, 'y') {
                    FaceCompare::Same => {
                        // Defined "up" as positive y instead of positive z
                        // => rotate clockwise around x axis
                        first.rotate_x_clockwise();

                        for beacon in self.beacons.iter_mut() {
                            beacon.rotate_x_clockwise();
                        }

                        for c in changes2.iter_mut() {
                            c.rotate_x_clockwise();
                        }
                    },
                    FaceCompare::Flipped => {
                        // Defined "up" as negative y instead of positive z
                        // => rotate counterclockwise around x axis
                        first.rotate_x_counterclockwise();

                        for beacon in self.beacons.iter_mut() {
                            beacon.rotate_x_counterclockwise();
                        }

                        for c in changes2.iter_mut() {
                            c.rotate_x_counterclockwise();
                        }
                    },
                    FaceCompare::Different => panic!("No shared 'up' direction?!"),
                }
            },
        }

        self.position = beacons1[0] - first;

        for beacon in self.beacons.iter_mut() {
            *beacon += self.position;
        }
    }
}

impl std::fmt::Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "--- scanner ---")?;
        for beacon in self.beacons.iter() {
            writeln!(f, "{}", beacon)?;
        }
        write!(f, "")
    }
}

impl<I> std::convert::From<&mut I> for Scanner
    where I: Iterator<Item=String>
{
    fn from(iter: &mut I) -> Self {
        let mut beacons = Vec::new();

        while let Some(line) = iter.next() {
            if line.is_empty() {
                break;
            }

            beacons.push(Vec3::from(&line));
        }

        let nbeacons = beacons.len();
        let mut distance_matrix = vec![vec![0; nbeacons]; nbeacons];

        for i in 0..nbeacons-1 {
            for j in i+1..nbeacons {
                distance_matrix[i][j] = (beacons[i] - beacons[j]).mag_sq();
                distance_matrix[j][i] = distance_matrix[i][j];
            }
        }

        Self {
            position: Vec3::new(0, 0, 0),
            nbeacons,
            beacons,
            distance_matrix,
        }
    }
}

fn at_least_12_matches_in_rows(row1: &[i32], row2: &[i32]) -> bool {
    let mut matches = 0;

    for i in row1.iter() {
        for j in row2.iter() {
            if i == j {
                matches += 1;
                if matches == 12 {
                    return true;
                }
            }
        }
    }

    return false;
}

fn compare_directions(a: &[Vec3], axis1: char, b: &[Vec3], axis2: char) -> FaceCompare {
    let a: Vec<&i32> = match axis1 {
        'x' => a.iter().map(|v| v.x()).collect(),
        'y' => a.iter().map(|v| v.y()).collect(),
        'z' => a.iter().map(|v| v.z()).collect(),
        _ => panic!("Invalid axis!"),
    };

    let b: Vec<&i32> = match axis2 {
        'x' => b.iter().map(|v| v.x()).collect(),
        'y' => b.iter().map(|v| v.y()).collect(),
        'z' => b.iter().map(|v| v.z()).collect(),
        _ => panic!("Invalid axis!"),
    };

    if a.iter().zip(b.iter()).all(|(&&i, &&j)| i == j) {
        FaceCompare::Same
    } else if a.iter().zip(b.iter()).all(|(&&i, &&j)| i == -j) {
        FaceCompare::Flipped
    } else {
        FaceCompare::Different
    }
}
