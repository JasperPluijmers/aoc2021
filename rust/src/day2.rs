use std::fs;

pub(crate) fn main() {
    let filename = "../input/2.txt";
    first(filename);
    second(filename)
}


fn second(filename: &str) {
    let file = fs::read_to_string(filename).unwrap();
    let final_position = file.trim().split("\n").fold(Position { depth: 0, distance: 0, aim: 0 }, |pos, line| {
        let inputs: Vec<&str> = line.split(" ").collect();
        let amount = inputs[1].parse::<isize>().unwrap();
        match inputs[0] {
            "forward" => Position { distance: pos.distance + amount, depth: pos.depth + (pos.aim * amount), ..pos },
            "down" => Position { aim: pos.aim + amount, ..pos },
            "up" => Position { aim: pos.aim - amount, ..pos },
            &_ => panic!("unexpected command {}", inputs[0])
        }
    });
    println!("{}", final_position.depth * final_position.distance)
}

fn first(filename: &str) {
    let file = fs::read_to_string(filename).unwrap();
    let final_position = file.trim().split("\n").fold(Position { depth: 0, distance: 0, aim: 0 }, |pos, line| {
        let inputs: Vec<&str> = line.split(" ").collect();
        let amount = inputs[1].parse::<isize>().unwrap();
        match inputs[0] {
            "forward" => Position { distance: pos.distance + amount, ..pos },
            "down" => Position { depth: pos.depth + amount, ..pos },
            "up" => Position { depth: pos.depth - amount, ..pos },
            &_ => panic!("unexpected command {}", inputs[0])
        }
    });

    println!("{}", final_position.depth * final_position.distance)
}

#[derive(Debug)]
struct Position {
    depth: isize,
    distance: isize,
    aim: isize,
}