use crate::util;

pub(crate) fn main() {
    second()
}

fn second() {
    let mut outputs = util::read_input(&util::input(10))
        .trim()
        .split('\n')
        .map(|line| LineChecker::new().check_line(line))
        .flatten()
        .map(|output| score(output))
        .collect::<Vec<usize>>();
    outputs.sort();
    println!("{:?}", outputs[outputs.len()/2])
}

fn score(input: Vec<char>) -> usize {
    let mut score = 0;
    for letter in input.into_iter().rev() {
        score *= 5;
        score += match letter {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            x => panic!("unknown letter encountered: {}", x)
        }
    }
    score
}

fn first() {
    let outputs = util::read_input(&util::input(10))
        .trim()
        .split('\n')
        .map(|line| LineChecker::new().check_line(line))
        .filter(|result| result.is_err())
        .map(|err| {
            if let Err(inner) = err {
                value(inner)
            } else {
                panic!("")
            }
        })
        .sum::<u32>();
    println!("{:?}", outputs)
}

fn value(letter: char) -> u32 {
    match letter {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        x => panic!("unknown letter encountered: {}", x),
    }
}

#[derive(Debug)]
struct LineChecker {
    stack: Vec<char>,
}

impl LineChecker {
    fn new() -> LineChecker {
        LineChecker { stack: vec![] }
    }

    fn check_line(mut self, line: &str) -> Result<Vec<char>, char> {
        for letter in line.chars() {
            match letter {
                n @ ('[' | '<' | '(' | '{') => self.stack.push(n),
                ']' => {
                    let last_opened = self.stack.pop().unwrap();
                    if last_opened != '[' {
                        return Err(']');
                    }
                }
                '>' => {
                    let last_opened = self.stack.pop().unwrap();
                    if last_opened != '<' {
                        return Err('>');
                    }
                }
                ')' => {
                    let last_opened = self.stack.pop().unwrap();
                    if last_opened != '(' {
                        return Err(')');
                    }
                }
                '}' => {
                    let last_opened = self.stack.pop().unwrap();
                    if last_opened != '{' {
                        return Err('}');
                    }
                }
                x => panic!("unknown letter encountered: {}", x),
            }
        }
        Ok(self.stack)
    }
}
