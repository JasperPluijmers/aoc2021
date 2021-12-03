use std::fs;

pub(crate) fn main() {
    let filename = "../input/3.txt";
    //first(filename);
    second(filename)
}

fn first(filename: &str) {
    let file = fs::read_to_string(filename).unwrap();
    let mut data = file.trim().split('\n').fold(vec![0; 13], |mut acc, el| {
        acc[12] += 1;
        for (i, c) in el.chars().enumerate() {
            acc[i] += c.to_digit(10).unwrap();
        }
        acc
    });
    let length = data.pop().unwrap();
    let gamma = data
        .iter()
        .map(|value| if value > &(length / 2) { '1' } else { '0' })
        .collect::<String>();
    let epsilon = data
        .iter()
        .map(|value| if value > &(length / 2) { '0' } else { '1' })
        .collect::<String>();
    println!(
        "{}",
        isize::from_str_radix(&epsilon, 2).unwrap() * isize::from_str_radix(&gamma, 2).unwrap()
    )
}

fn second(filename: &str) {
    let file = fs::read_to_string(filename).unwrap();
    let mut root = Node::root();
    file.trim()
        .split('\n')
        .for_each(|number| root.add(number.chars()));
    let oxygen = isize::from_str_radix(
        &root
            .get_oxygen(vec![])
            .iter()
            .map(|value| value.to_string())
            .collect::<String>(),
        2,
    )
    .unwrap();
    let co2 = isize::from_str_radix(
        &root
            .get_co2(vec![])
            .iter()
            .map(|value| value.to_string())
            .collect::<String>(),
        2,
    )
    .unwrap();
    println!("{}", oxygen * co2)
}

#[derive(Debug)]
struct Node {
    zero_child_amount: usize,
    one_child_amount: usize,
    zero_child: Box<Option<Node>>,
    one_child: Box<Option<Node>>,
    value: Option<u8>,
}

impl Node {
    fn root() -> Node {
        Node {
            zero_child_amount: 0,
            one_child_amount: 0,
            zero_child: Box::new(None),
            one_child: Box::new(None),
            value: None,
        }
    }

    fn zero() -> Node {
        Node {
            zero_child_amount: 0,
            one_child_amount: 0,
            zero_child: Box::new(None),
            one_child: Box::new(None),
            value: Some(0),
        }
    }

    fn one() -> Node {
        Node {
            zero_child_amount: 0,
            one_child_amount: 0,
            zero_child: Box::new(None),
            one_child: Box::new(None),
            value: Some(1),
        }
    }

    fn add(&mut self, mut number: impl Iterator<Item = char>) {
        let next = number.next();
        if let Some(value) = next {
            match value.to_digit(10).unwrap() {
                0 => {
                    self.zero_child_amount += 1;
                    if self.zero_child.is_none() {
                        self.zero_child = Box::new(Some(Node::zero()));
                    }
                    if let Some(zero_child) = &mut *self.zero_child {
                        zero_child.add(number);
                    }
                }
                1 => {
                    self.one_child_amount += 1;
                    if self.one_child.is_none() {
                        self.one_child = Box::new(Some(Node::one()));
                    }
                    if let Some(one_child) = &mut *self.one_child {
                        one_child.add(number);
                    }
                }
                x => println!("unknown digit seen: {}", x),
            }
        }
    }

    fn get_oxygen(&self, mut numbers: Vec<usize>) -> Vec<usize> {
        if self.one_child_amount >= self.zero_child_amount {
            if let Some(one_child) = &*self.one_child {
                numbers.push(1);
                return one_child.get_oxygen(numbers);
            }
        } else {
            if let Some(zero_child) = &*self.zero_child {
                numbers.push(0);
                return zero_child.get_oxygen(numbers);
            }
        }
        numbers
    }

    fn get_co2(&self, mut numbers: Vec<usize>) -> Vec<usize> {
        if (self.zero_child_amount == 0)
            | ((self.one_child_amount > 0) & (self.one_child_amount < self.zero_child_amount))
        {
            if let Some(one_child) = &*self.one_child {
                numbers.push(1);
                return one_child.get_co2(numbers);
            }
        } else {
            if let Some(zero_child) = &*self.zero_child {
                numbers.push(0);
                return zero_child.get_co2(numbers);
            }
        }
        numbers
    }
}
