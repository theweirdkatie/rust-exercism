pub struct Triangle {
    sides: [u64; 3], 
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().all(|&x| x>0 ) && sides[0]+sides[1] >= sides[2] && sides[1]+sides[2] >= sides[0]{
            return Some(Triangle{sides});
        } else {
            return None;
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.as_slice().windows(2).all(|x| x[0] == x[1])
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[0] != self.sides[2] && self.sides[1] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        !self.is_equilateral() && !self.is_scalene()
    }
}
