use crate::RebootInstruction;

#[derive(Debug)]
pub struct Quadrilateral {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    zmin: i32,
    zmax: i32,
    holes: Vec<Quadrilateral>,
}

impl Quadrilateral {
    pub fn new(xmin: i32, xmax: i32, ymin: i32, ymax: i32, zmin: i32, zmax: i32) -> Self {
        Self { xmin, xmax, ymin, ymax, zmin, zmax, holes: Vec::new() }
    }

    pub fn restrict(mut self, xmin: i32, xmax: i32, ymin: i32, ymax: i32, zmin: i32, zmax: i32) -> Option<Self> {
        if self.xmax < xmin || self.xmin > xmax
            || self.ymax < ymin || self.ymin > ymax
            || self.zmax < zmin || self.zmin > zmax
        {
            return None;
        }

        self.xmin = self.xmin.max(xmin);
        self.xmax = self.xmax.min(xmax);
        self.ymin = self.ymin.max(ymin);
        self.ymax = self.ymax.min(ymax);
        self.zmin = self.zmin.max(zmin);
        self.zmax = self.zmax.min(zmax);

        Some(self)
    }

    pub fn volume(&self) -> usize {
        let v = (self.xmax - self.xmin + 1) as usize
              * (self.ymax - self.ymin + 1) as usize
              * (self.zmax - self.zmin + 1) as usize;
        let h = self.holes.iter().fold(0, |acc, hole| acc + hole.volume());

        v - h
    }

    pub fn is_empty(&self) -> bool {
        self.volume() == 0
    }

    pub fn intersect(&mut self, other: &Quadrilateral) {
        if other.xmax < self.xmin || other.xmin > self.xmax
            || other.ymax < self.ymin || other.ymin > self.ymax
            || other.zmax < self.zmin || other.zmin > self.zmax
        {
            return;
        }

        let intersect_xmin = self.xmin.max(other.xmin);
        let intersect_xmax = self.xmax.min(other.xmax);
        let intersect_ymin = self.ymin.max(other.ymin);
        let intersect_ymax = self.ymax.min(other.ymax);
        let intersect_zmin = self.zmin.max(other.zmin);
        let intersect_zmax = self.zmax.min(other.zmax);

        let new_hole = Self::new(intersect_xmin,
                                 intersect_xmax,
                                 intersect_ymin,
                                 intersect_ymax,
                                 intersect_zmin,
                                 intersect_zmax);

        for hole in self.holes.iter_mut() {
            hole.intersect(other);
        }

        self.holes.push(new_hole);
    }
}

impl std::convert::From<&RebootInstruction> for Quadrilateral {
    fn from(instruction: &RebootInstruction) -> Self {
        Self::new(instruction.xmin,
                  instruction.xmax,
                  instruction.ymin,
                  instruction.ymax,
                  instruction.zmin,
                  instruction.zmax)
    }
}
