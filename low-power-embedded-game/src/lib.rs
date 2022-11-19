// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    ( (dividend/divisor) , (dividend % divisor) )
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    // step_by(step) creates an iterator starting at the same point, but stepping
    // by the given amount at each iteration. 
    // This is equivalent to .next(), .nth(step-1), .nth(step-1), ..
    iter.step_by(2)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        // Manhattan is the sum of the absolute differences between points
        // Since the reference point is the origin, return the sum of the absolute
        // values of x and y
        self.0.abs() + self.1.abs()
    }
}
