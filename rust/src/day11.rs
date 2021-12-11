use std::cmp::max;
use crate::util;
use crate::util::Grid;
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;
use termion::{color, style};
use rand::Rng;

pub fn main() {
    let mut grid = Grid::new(util::read_input(&util::example(11)).trim());
    let length = grid.size.0 * grid.size.1;
    // let mut mask = create_standard_mask((grid.size.0, grid.size.1/2), 9);
    // let mask_2 = create_standard_mask((grid.size.0, grid.size.1/2), 3);
    // mask.extend(mask_2);

    let mask = create_random_mask(grid.size, 6,9);
    for i in 0.. {
        show_grid(&grid.grid);
        grid.step(&mask);
        // if length == grid.step(&mask) {
        //     return;
        // }
        sleep(Duration::from_millis(200))
    }
}

fn show_grid(grid: &Vec<Vec<u32>>) {
    let displ = grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|value| display_value(value))
                .collect::<String>()
                + "\n"
        })
        .collect::<String>();
    print!("{}[2J", 27 as char);
    println!("{}", displ)
}

fn display_value(value: &u32) -> String {
    if value == &0 {
        format!("{}{}{}", color::Fg(color::Red), value, style::Reset)
    } else {
        value.to_string()
    }
}

fn create_standard_mask(size: (usize, usize), value: u32) -> Vec<Vec<u32>> {
    let mut mask = vec![];
    for _ in 0..size.1 {
        mask.push(vec![value; size.0])
    }
    mask
}

fn create_random_mask(size: (usize, usize), minimum: u32, maximum: u32) -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();
    let mut mask = vec![];
    for _ in 0..size.1 {
        let mut line = vec![];
        for _ in 0..size.0 {
            line.push(rng.gen_range(minimum..maximum))
        }
        mask.push(line)
    }
    mask

}

trait Octo {
    fn increment(&mut self);

    fn flash(
        &mut self,
        to_flash: HashSet<(usize, usize)>,
        flashed: HashSet<(usize, usize)>,
        mask: &Vec<Vec<u32>>,
    ) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>);

    fn cleanup(&mut self, mask: &Vec<Vec<u32>>);

    fn step(&mut self, mask: &Vec<Vec<u32>>) -> usize;
}

impl Octo for Grid {
    fn increment(&mut self) {
        self.grid
            .iter_mut()
            .for_each(|line| line.iter_mut().for_each(|mut value| *value += 1));
    }

    fn flash(
        &mut self,
        mut to_flash: HashSet<(usize, usize)>,
        mut flashed: HashSet<(usize, usize)>,
        mask: &Vec<Vec<u32>>,
    ) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>) {
        if to_flash.is_empty() {
            return (to_flash, flashed);
        }
        let mut new_to_flash = HashSet::new();
        for flash in &to_flash {
            flashed.insert(flash.clone());
            let neighbours = self.get_neighbour_locations(flash.0, flash.1);
            for neighbour in neighbours {
                self.set_value(
                    neighbour.0,
                    neighbour.1,
                    self.get_value(neighbour.0, neighbour.1) + 1,
                );
                if should_flash(self.get_value(neighbour.0, neighbour.1), mask, &neighbour)
                    && !flashed.contains(&neighbour)
                    && !to_flash.contains(&neighbour)
                {
                    new_to_flash.insert(neighbour);
                }
            }
        }

        self.flash(new_to_flash, flashed, mask)
    }

    fn cleanup(&mut self, mask: &Vec<Vec<u32>>) {
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                if should_flash(self.get_value(x, y), mask, &(x, y)) {
                    self.set_value(x, y, 0)
                }
            }
        }
    }

    fn step(&mut self, mask: &Vec<Vec<u32>>) -> usize {
        self.increment();
        let to_flash = self
            .grid
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(x, value)| **value > 9)
                    .map(move |(x, _)| (x, y))
            })
            .flatten()
            .collect::<HashSet<(usize, usize)>>();
        let amount = self.flash(to_flash, HashSet::new(), mask).1.len();
        self.cleanup(mask);
        amount
    }
}

fn should_flash(value: u32, mask: &Vec<Vec<u32>>, coordinate: &(usize, usize)) -> bool {
    value > mask[coordinate.1][coordinate.0]
}
