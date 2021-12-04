use crate::day4::BingoField::{Marked, Unmarked};
use std::collections::HashMap;
use std::fs;
use std::iter::Map;
use std::str::Split;

pub(crate) fn main() {
    let filename = "../input/4.txt";
    let example = "../example/4.txt";
    first(filename);
    second(filename)
}

fn second(filename: &str) {
    let (called_numbers, mut boards) = parse_input(filename);
    for number in called_numbers {
        let mut to_delete = vec![];
        for i in 0..boards.len() {
            if let Some(score) = boards[i].mark(number) {
                if boards.len() == 1 {
                    println!("{}", score);
                    return
                } else {
                    to_delete.push(i);
                }
            }
        }
        for number in to_delete.into_iter().rev() {
            boards.remove(number);
        }
    }
}


fn first(filename: &str) {
    let (called_numbers, mut boards) = parse_input(filename);
    for number in called_numbers {
        for board in boards.iter_mut() {
            if let Some(score) = board.mark(number) {
                println!("{}", score);
                return;
            }
        }
    }
}

fn parse_input(filename: &str) -> (Vec<usize>, Vec<BingoBoard>){
    let file = fs::read_to_string(filename).unwrap();
    let mut lines = file.trim().split("\n\n");
    let called_numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|value| value.parse::<usize>().unwrap())
        .collect();
    let mut boards = lines
        .filter(|value| !value.is_empty())
        .map(|repr| BingoBoard::new(repr))
        .collect::<Vec<BingoBoard>>();
    (called_numbers, boards)
}

#[derive(Debug)]
enum BingoField {
    Marked(usize),
    Unmarked(usize),
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct BingoBoard {
    board: Vec<Vec<BingoField>>,
    numbers: HashMap<usize, Position>,
}

impl BingoBoard {
    fn new(board_string: &str) -> BingoBoard {
        let board: Vec<Vec<BingoField>> = board_string
            .trim()
            .split('\n')
            .map(|row| {
                row.split_whitespace()
                    .map(|number| BingoField::Unmarked(number.trim().parse::<usize>().unwrap()))
                    .collect::<Vec<BingoField>>()
            })
            .collect();
        let mut numbers = HashMap::new();
        board.iter().enumerate().for_each(|(index, row)| {
            row.iter().enumerate().for_each(|(second_index, field)| {
                if let Unmarked(number) = field {
                    numbers.insert(
                        *number,
                        Position {
                            x: index,
                            y: second_index,
                        },
                    );
                }
            })
        });
        BingoBoard { board, numbers }
    }

    fn mark(&mut self, number: usize) -> Option<usize> {
        if let Some(position) = self.numbers.get(&number) {
            self.board[position.x][position.y] = Marked(number);
            if self.check_row(position.x) | self.check_column(position.y) {
                return Some(self.score() * number);
            }
        }
        None
    }

    fn check_row(&self, row: usize) -> bool {
        for i in 0..5 {
            if let Unmarked(_) = self.board[row][i] {
                return false;
            }
        }
        true
    }

    fn check_column(&self, column: usize) -> bool {
        for i in 0..5 {
            if let Unmarked(_) = self.board[i][column] {
                return false;
            }
        }
        true
    }

    fn score(&self) -> usize {
        self.board
            .iter()
            .flatten()
            .map(|value| match value {
                Marked(_) => 0,
                Unmarked(number) => *number,
            })
            .sum()
    }
}
