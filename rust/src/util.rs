
#[derive(Debug,PartialEq,Hash, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Point {
    pub(crate) fn new(x: usize, y: usize) -> Point {
        Point {x, y}
    }
}