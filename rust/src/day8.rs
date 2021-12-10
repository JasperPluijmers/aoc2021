use crate::util;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;

pub(crate) fn main() {
    println!("{}", first(&util::input(8)));
    println!("{}", second(&util::input(8)))
}

fn first(filename: &str) -> usize {
    let allowed = [2 as usize, 3, 4, 7];
    util::read_input(filename)
        .trim()
        .split('\n')
        .map(|line| {
            line.split('|')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|value| if allowed.contains(&value.len()) { 1 } else { 0 })
        })
        .flatten()
        .sum::<usize>()
}

fn second(filename: &str) -> usize{
    util::read_input(filename).trim().split('\n').map(|line| {
        let mut parts = line.split('|');
        let mapping = solve_line(
            parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|value| value.to_string())
                .collect::<Vec<String>>(),
        );
        calculate_display(mapping, parts.next().unwrap())
    }).sum()
}

fn calculate_display(mapping: HashMap<char, char>, line: &str) -> usize {
    let numbers = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];
    let number = usize::from_str(&line.trim().split_whitespace().map(|word| {
        let mut chars = word
            .chars()
            .map(|letter| *mapping.get(&letter).unwrap())
            .collect::<Vec<char>>();
        chars.sort();
        let word = String::from_iter(chars.iter());
        numbers
            .iter()
            .enumerate()
            .filter(|(_, number_word)| number_word.to_string() == word)
            .next()
            .unwrap()
            .0
            .to_string()
    })
        .collect::<String>()).unwrap();
    number
}

fn solve_line(inputs: Vec<String>) -> HashMap<char, char> {
    let mut mapping = HashMap::<char, char>::new();
    let mut occurences = HashMap::new();
    inputs
        .iter()
        .map(|value| value.chars())
        .flatten()
        .for_each(|value| *occurences.entry(value).or_insert(0) += 1);
    for (value, occurence) in occurences.iter() {
        match occurence {
            4 => {
                mapping.insert(*value, 'e');
            }
            6 => {
                mapping.insert(*value, 'b');
            }
            9 => {
                mapping.insert(*value, 'f');
            }
            8 => {
                if inputs
                    .iter()
                    .filter(|word| word.len() == 2)
                    .next()
                    .unwrap()
                    .chars()
                    .any(|letter| &letter == value)
                {
                    mapping.insert(*value, 'c');
                } else {
                    mapping.insert(*value, 'a');
                }
            }
            7 => {
                if inputs
                    .iter()
                    .filter(|word| word.len() == 4)
                    .next()
                    .unwrap()
                    .chars()
                    .any(|letter| &letter == value)
                {
                    mapping.insert(*value, 'd');
                } else {
                    mapping.insert(*value, 'g');
                }
            }
            &_ => {
                println!("Wrong occurences discovered: {}", value)
            }
        }
    }
    mapping
}
