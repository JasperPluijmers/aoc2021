use std::fs;

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

pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}