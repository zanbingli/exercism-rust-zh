pub struct Triangle {
    sides: Vec<u64>,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut sd = sides.clone();
        sd.sort();
        if sd[0] + sd[1] <= sd[2] {
            return None;
        }
        Some(Triangle {
            sides: Vec::from(sd),
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] ||
            self.sides[1] == self.sides[2] ||
            self.sides[0] == self.sides[2]
    }
}
