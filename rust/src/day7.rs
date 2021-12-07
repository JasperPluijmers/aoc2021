use crate::util;
use std::str::FromStr;

pub fn main() {
    let example = "../input/7.txt";
    let numbers = util::read_input(example)
        .trim()
        .split(',')
        .map(|value| usize::from_str(value).unwrap())
        .collect::<Vec<usize>>();
    let minimum = *numbers.iter().min().unwrap();
    let maximum = *numbers.iter().max().unwrap();
    let minimum_fuel: usize = (minimum..maximum)
        .map(|value| {
            numbers
                .iter()
                .map(|&number| {
                    fuel_two(number, value)
                })
                .sum()
        })
        .min()
        .unwrap();
    println!("{:?}", minimum_fuel)
}

fn fuel_one(number_one: usize, number_two: usize) -> usize {
    if number_one > number_two {
        number_one - number_two
    } else {
        number_two - number_one
    }
}

fn fuel_two(number_one: usize, number_two: usize) -> usize {
    (1..=fuel_one(number_one,number_two)).sum::<usize>()
}