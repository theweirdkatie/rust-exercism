use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Default + Copy + PartialEq + PartialOrd + Add<Output = T>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().any(|&x| x == T::default()) || sides[0] + sides[1] < sides[2] || sides[1] + sides[2] < sides[0] {
            return None;
        }
        Some(Triangle { sides })
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
