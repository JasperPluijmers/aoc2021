use std::collections::HashSet;
use std::str::FromStr;
use crate::util;

pub fn main() {
    first(&util::input(9));
    second(&util::input(9))
}

fn second(filename: &str) {
    let grid = Grid::new(&util::read_input(filename));
    let mut basins = vec![];
    for x in 0..grid.size.0 {
        for y in 0..grid.size.1 {
            if grid.get_value(x,y) != 9 && !basins.iter().flatten().any(|value| value == &(x,y)) {
                basins.push(grid.find_basin(x,y, HashSet::new()))
            }
        }
    }
    let mut basin_lengths = basins.iter().map(|basin| basin.len()).collect::<Vec<usize>>();
    basin_lengths.sort();
    println!("{:?}", basin_lengths.iter().rev().take(3).product::<usize>())
}

fn first(filename: &str) {
    let grid = Grid::new(&util::read_input(filename));
    let mut low_points = vec![];
    for x in 0..grid.size.0 {
        for y in 0..grid.size.1 {
            let value = grid.get_value(x,y);
            let neighbours = grid.get_neighbours(x,y);
            if neighbours.iter().all(|neighbour| neighbour > &value) {
                low_points.push(1 + value);
            }
        }
    }
    println!("{:?}", low_points);
    println!("{}", low_points.iter().sum::<u32>())
}

struct Grid {
    grid: Vec<Vec<u32>>,
    size: (usize, usize)
}

impl Grid {
    fn new(desc: &str) -> Grid {
        let grid = desc
            .trim()
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|letter| letter.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        let size_y = grid.len();
        let size_x = grid[0].len();
        Grid {grid, size: (size_x, size_y)}
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<u32>{
        let mut neighbours = vec![];
        if y > 0 {
            neighbours.push(self.grid[y-1][x])
        }
        if y + 1 < self.size.1 {
            neighbours.push(self.grid[y+1][x])
        }
        if x % self.size.0 != 0 {
            neighbours.push(self.grid[y][x - 1])
        }
        if x % self.size.0 != self.size.0 - 1 {
            neighbours.push(self.grid[y][x + 1])
        }
        neighbours
    }

    fn get_neighbour_locations(&self, x: usize, y: usize) -> Vec<(usize, usize)>{
        let mut neighbours = vec![];
        if y > 0 {
            neighbours.push((x, y-1))
        }
        if y + 1 < self.size.1 {
            neighbours.push((x, y+1))
        }
        if x % self.size.0 != 0 {
            neighbours.push((x-1, y))
        }
        if x % self.size.0 != self.size.0 - 1 {
            neighbours.push((x+1,y))
        }
        neighbours
    }

    fn get_value(&self, x: usize, y: usize) -> u32 {
        self.grid[y][x]
    }

    fn find_basin(&self, x: usize, y: usize, mut basin: HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
        let neighbours = self.get_neighbour_locations(x, y);
        for neighbour in neighbours {
            let neighbour_value = self.get_value(neighbour.0, neighbour.1);
            if neighbour_value != 9 && !basin.contains(&neighbour){
                basin.insert(neighbour);
                basin.extend(&self.find_basin(neighbour.0, neighbour.1, basin.clone()))
            }
        }
        basin
    }
}
