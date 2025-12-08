mod day07_common;
use day07_common::*;

fn start(grid: &mut Vec<Vec<char>>) {
    for y in 0..grid[0].len() {
        if grid[0][y] == 'S' {
            grid[1][y] = '1';
        }
    }
}

fn step(line: usize, grid: &mut Vec<Vec<char>>, counter: &mut i32) {
    for y in 0..grid[line].len() {
        if grid[line][y] != '1' {
            continue;
        }
        if grid[line + 1][y] == '^' {
            *counter += 1;
            grid[line + 1][y + 1] = '1';
            grid[line + 1][y - 1] = '1';
        } else {
            grid[line + 1][y] = '1';
        }
    }
}

fn main() {
    let mut grid = get_grid();
    let mut counter = 0;
    start(&mut grid);
    for i in 1..grid.len() - 1 {
        step(i, &mut grid, &mut counter);
    }
    println!("{}", counter);
}
