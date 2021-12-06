use std::fs;
use std::time::Instant;

pub fn main() {
    let example = "../example/6.txt";
    let filename = "../input/6.txt";
    let begin = Instant::now();
    first(filename);
    println!("{:?}", Instant::now() - begin)
}

fn first(filename: &str) {
    let file = fs::read_to_string(filename).unwrap();
    let days = 256;
    let mut state = [0 as usize,0,0,0,0,0,0,0,0];
    let mut new_state = [0 as usize,0,0,0,0,0,0,0,0];
    file
        .trim()
        .split(',')
        .map(|value| usize::from_str_radix(value, 10).unwrap())
        .for_each(|value| state[value] += 1);
    for _ in 0..days {
        state = [state[1], state[2], state[3], state[4], state[5], state[6], state[0] + state[7], state[8], state[0]];
        // new_state[8] = state[0];
        // for i in 1..state.len() {
        //     new_state[i-1] = state[i]
        // }
        // new_state[6] += new_state[8];
        // state = new_state;
    }
    println!("{:?}", state.iter().sum::<usize>())
}
