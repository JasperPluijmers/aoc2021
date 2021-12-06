use std::cmp::{max, min};
use crate::util::Point;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn main() {
    let example = "../example/5.txt";
    let input = "../input/5.txt";
    first(input)
}

fn first(filename: &str) {
    println!("{:?}", parse(filename).into_iter().filter(|(point, amount)| amount > &&1).collect::<Vec<(Point, usize)>>().len());
}

fn parse(filename: &str) -> HashMap<Point, usize> {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let file = fs::read_to_string(filename).unwrap();
    let instructions = file.trim().split('\n').map(|line| {
        let caps = re.captures(line).unwrap();
        LineInstruction {
            from: Point::new(
                usize::from_str_radix(caps.get(1).unwrap().as_str(), 10).unwrap(),
                usize::from_str_radix(caps.get(2).unwrap().as_str(), 10).unwrap())
            , to: Point::new(
                usize::from_str_radix(caps.get(3).unwrap().as_str(), 10).unwrap(),
                usize::from_str_radix(caps.get(4).unwrap().as_str(), 10).unwrap())
        }
    });
    let mut amounts = HashMap::new();
    instructions.for_each(|instruction| {
        if instruction.is_straight() {
            draw_straight_line(&mut amounts, instruction)
        } else {
            draw_sloped_line(&mut amounts, instruction)
        }
    });
    return amounts
}

fn draw_straight_line(amounts: &mut HashMap<Point, usize>, instruction: LineInstruction) {
    let from_x = min(instruction.from.x, instruction.to.x);
    let to_x = max(instruction.from.x, instruction.to.x);
    let from_y = min(instruction.from.y, instruction.to.y);
    let to_y = max(instruction.from.y, instruction.to.y);
    for x in from_x..=to_x {
        for y in from_y..=to_y {
            *amounts.entry(Point::new(x,y)).or_insert(0) += 1
        }
    }
}

fn draw_sloped_line(amounts: &mut HashMap<Point, usize>, instruction: LineInstruction) {
    let left_point = if instruction.from.x < instruction.to.x {&instruction.from} else {&instruction.to};
    let right_point = if instruction.from.x > instruction.to.x {&instruction.from} else {&instruction.to};
    let up = left_point.y < right_point.y;
    let mut y = left_point.y;
    for x in left_point.x..=right_point.x {
        *amounts.entry(Point::new(x,y)).or_insert(0) += 1;
        if up {
            y+=1;
        } else {
            if y > 0 {
                y -= 1;
            }
        }
    }
}

#[derive(Debug)]
struct LineInstruction {
    from: Point,
    to: Point,
}

impl LineInstruction {
    fn is_straight(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }
}
