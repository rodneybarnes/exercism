// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let result = dividend / divisor;
    let remainder = dividend % divisor;
    (result, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate().filter(|i| i.0 % 2 == 0).map(|i| i.1)
}

pub struct Position(pub i16, pub i16);
impl Position {
    /// x1 - x0 + y1 - y0
    pub fn manhattan(&self) -> i16 {
        (&self.0 - 0).abs() + (&self.1 - 0).abs()
    }
}
