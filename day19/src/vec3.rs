#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Vec3(i32, i32, i32);

impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(x, y, z)
    }

    #[inline(always)]
    pub fn x(&self) -> &i32 { &self.0 }
    #[inline(always)]
    pub fn y(&self) -> &i32 { &self.1 }
    #[inline(always)]
    pub fn z(&self) -> &i32 { &self.2 }

    #[inline(always)]
    pub fn flip_xz(&mut self) {
        // Rotate 180° around y axis
        self.0 = -self.0;
        self.2 = -self.2;
    }

    #[inline(always)]
    pub fn flip_yz(&mut self) {
        // Rotate 180° around x axis
        self.1 = -self.1;
        self.2 = -self.2;
    }

    #[inline(always)]
    pub fn flip_xy(&mut self) {
        // Rotate 180° around z axis
        self.0 = -self.0;
        self.1 = -self.1;
    }

    #[inline(always)]
    pub fn rotate_x_clockwise(&mut self) {
        // Clockwise rotation around x axis => y -> z, z -> -y
        let temp = self.1;
        self.1 = -self.2;
        self.2 = temp;
    }

    #[inline(always)]
    pub fn rotate_x_counterclockwise(&mut self) {
        // Counterclockwise rotation around x axis => y -> -z, z -> y
        let temp = self.1;
        self.1 = self.2;
        self.2 = -temp;
    }

    #[inline(always)]
    pub fn rotate_y_clockwise(&mut self) {
        // Clockwise rotation around y axis => x -> -z, z -> x
        let temp = self.0;
        self.0 = self.2;
        self.2 = -temp;
    }

    #[inline(always)]
    pub fn rotate_y_counterclockwise(&mut self) {
        // Counterclockwise rotation around y axis => x -> z, z -> -x
        let temp = self.0;
        self.0 = -self.2;
        self.2 = temp;
    }

    #[inline(always)]
    pub fn rotate_z_clockwise(&mut self) {
        // Clockwise rotation around z axis => x -> y, y -> -x
        let temp = self.0;
        self.0 = -self.1;
        self.1 = temp;
    }

    #[inline(always)]
    pub fn rotate_z_counterclockwise(&mut self) {
        // Counterclockwise rotation around x axis => x -> -y, y -> x
        let temp = self.0;
        self.0 = self.1;
        self.1 = -temp;
    }

    pub fn mag_sq(&self) -> i32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn manhattan(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl std::convert::From<&str> for Vec3 {
    fn from(line: &str) -> Self  {
        let numbers: Vec<i32> = line.split(',').map(|n| n.parse().unwrap()).collect();
        Self(numbers[0], numbers[1], numbers[2])
    }
}

impl std::convert::From<String> for Vec3 {
    fn from(line: String) -> Self  {
        let numbers: Vec<i32> = line.split(',').map(|n| n.parse().unwrap()).collect();
        Self(numbers[0], numbers[1], numbers[2])
    }
}

impl std::convert::From<&String> for Vec3 {
    fn from(line: &String) -> Self  {
        let numbers: Vec<i32> = line.split(',').map(|n| n.parse().unwrap()).collect();
        Self(numbers[0], numbers[1], numbers[2])
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl std::ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl std::ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}
