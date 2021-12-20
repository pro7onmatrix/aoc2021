use std::str::Chars;

#[derive(Debug, Clone)]
pub enum SnailfishNumber {
    Empty,
    Value(i32),
    Pair {
        left: Box<SnailfishNumber>,
        right: Box<SnailfishNumber>,
    },
}

pub enum Propagate {
    KeepGoing,
    Left(i32),
    Right(i32),
    Done,
}

impl SnailfishNumber {
    pub fn new(input: &str) -> Self {
        if input.len() == 1 {
            // Only one number
            SnailfishNumber::Value(input.parse().unwrap())
        } else {
            // Skip enclosing brackets
            SnailfishNumber::read_pair(&mut input[1..input.len()-1].chars())
        }
    }

    fn read_pair(chars: &mut Chars) -> SnailfishNumber {
        let mut got_left = false;

        let mut left = Box::default();
        let mut right = Box::default();

        while let Some(c) = chars.next() {
            match c {
                '0'..='9' => {
                    if !got_left {
                        left = Box::new(SnailfishNumber::Value(c as i32 - '0' as i32));
                    } else {
                        right = Box::new(SnailfishNumber::Value(c as i32 - '0' as i32));
                    }
                },
                ',' => got_left = true,
                '[' => {
                    if !got_left {
                        left = Box::new(SnailfishNumber::read_pair(chars));
                    } else {
                        right = Box::new(SnailfishNumber::read_pair(chars));
                    }
                },
                ']' => break,
                _ => {}
            }
        }

        SnailfishNumber::Pair { left, right }
    }

    pub fn magnitude(&self) -> i32 {
        match self {
            SnailfishNumber::Value(n) => *n,
            SnailfishNumber::Pair { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
            SnailfishNumber::Empty => 0,
        }
    }

    pub fn reduce(&mut self) {
        loop {
            if self.max_depth() > 4 {
                self.handle_explosion();
            } else if self.needs_splits() {
                self.handle_split();
            } else {
                break;
            }
        }
    }

    pub fn handle_explosion(&mut self) {
        self.explosion_helper(0);
    }

    pub fn handle_split(&mut self) -> bool {
        if let SnailfishNumber::Pair { left, right } = self {
            if let SnailfishNumber::Value(n) = left.as_mut() {
                if *n >= 10 {
                    let l = *n / 2;
                    let r = *n - l;

                    **left = SnailfishNumber::Pair {
                        left: Box::new(SnailfishNumber::Value(l)),
                        right: Box::new(SnailfishNumber::Value(r)),
                    };

                    return true;
                }
            } else if left.handle_split() {
                return true;
            }

            if let SnailfishNumber::Value(n) = right.as_mut() {
                if *n >= 10 {
                    let l = *n / 2;
                    let r = *n - l;

                    **right = SnailfishNumber::Pair {
                        left: Box::new(SnailfishNumber::Value(l)),
                        right: Box::new(SnailfishNumber::Value(r)),
                    };

                    return true;
                }
            } else if right.handle_split() {
                return true;
            }
        }

        false
    }

    fn explosion_helper(&mut self, depth: u32) -> Propagate {
        if let SnailfishNumber::Pair { left, right } = self {
            if depth == 3 {
                let mut propagate = Propagate::KeepGoing;

                if let SnailfishNumber::Pair { left: l, right: r } = left.as_mut() {
                    if let SnailfishNumber::Value(m) = l.as_ref() {
                        propagate = Propagate::Left(*m);
                    } else {
                        panic!("Got a nesting deeper than 4 at pair {}!", left);
                    }

                    if let SnailfishNumber::Value(m) = r.as_ref() {
                        if let SnailfishNumber::Value(n) = right.as_mut() {
                            *n += m;
                        } else {
                            right.add_leftmost(*m);
                        }
                    } else {
                        panic!("Got a nesting deeper than 4 at pair {}!", left);
                    }

                    *left = Box::new(SnailfishNumber::Value(0));
                } else if let SnailfishNumber::Pair { left: l, right: r } = right.as_mut() {
                    if let SnailfishNumber::Value(m) = r.as_ref() {
                        propagate = Propagate::Right(*m);
                    } else {
                        panic!("Got a nesting deeper than 4 at pair {}!", right);
                    }

                    if let SnailfishNumber::Value(m) = l.as_ref() {
                        if let SnailfishNumber::Value(n) = left.as_mut() {
                            *n += m;
                        } else {
                            left.add_rightmost(*m);
                        }
                    } else {
                        panic!("Got a nesting deeper than 4 at pair {}!", right);
                    }

                    *right = Box::new(SnailfishNumber::Value(0));
                }

                return propagate;
            }

            match left.explosion_helper(depth + 1) {
                // If an explosion propagates from the left and wants to add to
                // the right, update the right branch, otherwise propagate further
                Propagate::Right(value) => {
                    if let SnailfishNumber::Value(n) = right.as_mut() {
                        *n += value;
                    } else {
                        right.add_leftmost(value);
                    }

                    return Propagate::Done;
                },
                Propagate::KeepGoing => {},
                other => return other,
            }

            match right.explosion_helper(depth + 1) {
                Propagate::Left(value) => {
                    // If an explosion propagates from the right and wants to add to
                    // the left, update the left branch, otherwise propagate further
                    if let SnailfishNumber::Value(n) = left.as_mut() {
                        *n += value;
                    } else {
                        left.add_rightmost(value);
                    }

                    return Propagate::Done;
                },
                Propagate::KeepGoing => {}
                other => return other,
            }
        }

        Propagate::KeepGoing
    }

    fn add_leftmost(&mut self, value: i32) {
        if let SnailfishNumber::Pair { left, right: _ } = self {
            if let SnailfishNumber::Value(n) = left.as_mut() {
                *n += value;
            } else {
                left.add_leftmost(value);
            }
        }
    }

    fn add_rightmost(&mut self, value: i32) {
        if let SnailfishNumber::Pair { left: _, right } = self {
            if let SnailfishNumber::Value(n) = right.as_mut() {
                *n += value;
            } else {
                right.add_rightmost(value);
            }
        }
    }

    pub fn max_depth(&self) -> usize {
        if let SnailfishNumber::Pair { left, right } = self {
            let depth_left = left.max_depth();
            let depth_right = right.max_depth();

            depth_left.max(depth_right) + 1
        } else {
            0
        }
    }

    fn needs_splits(&self) -> bool {
        match self {
            SnailfishNumber::Value(n) => *n >= 10,
            SnailfishNumber::Pair { left, right } => left.needs_splits() || right.needs_splits(),
            SnailfishNumber::Empty => false,
        }
    }
}

impl std::default::Default for SnailfishNumber {
    fn default() -> Self {
        SnailfishNumber::Value(0)
    }
}

impl std::ops::Add for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: SnailfishNumber) -> SnailfishNumber {
        if let SnailfishNumber::Empty = self {
            return rhs;
        }

        if let SnailfishNumber::Empty = rhs {
            return self;
        }

        let mut result = SnailfishNumber::Pair {
            left: Box::new(self),
            right: Box::new(rhs)
        };

        result.reduce();
        result
    }
}

impl std::ops::Add<&SnailfishNumber> for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: &SnailfishNumber) -> SnailfishNumber {
        if let SnailfishNumber::Empty = self {
            return rhs.clone();
        }

        if let SnailfishNumber::Empty = rhs {
            return self;
        }

        let mut result = SnailfishNumber::Pair {
            left: Box::new(self),
            right: Box::new(rhs.clone())
        };

        result.reduce();
        result
    }
}

impl std::ops::Add<&SnailfishNumber> for &SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: &SnailfishNumber) -> SnailfishNumber {
        if let SnailfishNumber::Empty = self {
            return rhs.clone();
        }

        if let SnailfishNumber::Empty = rhs {
            return self.clone();
        }

        let mut result = SnailfishNumber::Pair {
            left: Box::new(self.clone()),
            right: Box::new(rhs.clone())
        };

        result.reduce();
        result
    }
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SnailfishNumber::Value(n) => write!(f, "{}", n),
            SnailfishNumber::Pair { left, right }  => write!(f, "[{},{}]", left, right),
            SnailfishNumber::Empty => write!(f, "[]"),
        }
    }
}

impl std::cmp::PartialEq for SnailfishNumber {
    fn eq(&self, other: &SnailfishNumber) -> bool {
        match self {
            SnailfishNumber::Value(m) => {
                if let SnailfishNumber::Value(n) = other {
                    return m == n;
                } else {
                    return false;
                }
            },
            SnailfishNumber::Pair { left, right } => {
                if let SnailfishNumber::Pair { left: other_left, right: other_right } = other {
                    return *left == *other_left && *right == *other_right;
                } else {
                    return false;
                }
            },
            SnailfishNumber::Empty => {
                if let SnailfishNumber::Empty = other {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
}

impl std::iter::Sum for SnailfishNumber {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(SnailfishNumber::Empty, |a, b| a + b)
    }
}

impl<'a> std::iter::Sum<&'a SnailfishNumber> for SnailfishNumber {
    fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
        iter.fold(SnailfishNumber::Empty, |a, b| a + b)
    }
}
