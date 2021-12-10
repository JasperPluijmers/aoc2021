use crate::util;
use std::str::FromStr;

pub fn main() {
    let example = "../input/7.txt";

    util::run_timed(|filename| part_two(filename), example, 7);
}

fn part_two(filename: &str) -> usize {
    let numbers = util::read_input(filename)
        .trim()
        .split(',')
        .map(|value| usize::from_str(value).unwrap())
        .collect::<Vec<usize>>();
    (*numbers.iter().min().unwrap()..*numbers.iter().max().unwrap())
        .map(|value| numbers.iter().map(|&number| fuel_two(number, value)).sum())
        .min()
        .unwrap()
}

fn fuel_one(number_one: usize, number_two: usize) -> usize {
    if number_one > number_two {
        number_one - number_two
    } else {
        number_two - number_one
    }
}

fn fuel_two(number_one: usize, number_two: usize) -> usize {
    (1..=fuel_one(number_one, number_two)).sum::<usize>()
}

fn fuel_two_repl(number_one: usize, number_two: usize) -> usize {
    let n = fuel_one(number_one, number_two);
    n * (n+1) / 2
}
