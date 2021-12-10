use std::fmt::{Debug, format};
use std::fs;
use std::time::Instant;

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

pub fn run_timed<T, X>(f: fn(T) -> X, argument: T, part: u64)
    where
        X: Debug,
{
    let now = Instant::now();

    let answer = f(argument);

    println!(
        "part {}: {:?}, result found in {} ms",
        part,
        answer,
        now.elapsed().as_millis()
    );
}

pub fn example(number: i32) -> String {
    format!("../example/{}.txt", number)
}

pub fn input(number: i32) -> String {
    format!("../input/{}.txt", number)
}