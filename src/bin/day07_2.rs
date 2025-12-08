mod day07_common;
use day07_common::*;

fn start(grid: &mut Vec<Vec<String>>) {
    for y in 0..grid[0].len() {
        if grid[0][y] == "S" {
            grid[1][y] = "1".to_string();
        }
    }
}

fn increase(x: usize, y: usize, timelines: usize, grid: &mut Vec<Vec<String>>) {
    if grid[x][y] == "^" {
    } else if grid[x][y] == "." {
        grid[x][y] = timelines.to_string();
        return;
    } else {
        let val: usize = grid[x][y].parse().unwrap();
        grid[x][y] = (val + timelines).to_string();
    }
}

fn step(line: usize, grid: &mut Vec<Vec<String>>) {
    for y in 0..grid[line].len() {
        if grid[line][y] == "^" || grid[line][y] == "." {
            continue;
        }
        let timelines = grid[line][y].parse::<usize>().unwrap();
        if grid[line + 1][y] == "^" {
            increase(line + 1, y + 1, timelines, grid);
            increase(line + 1, y - 1, timelines, grid);
        } else {
            increase(line + 1, y, timelines, grid);
        }
    }
}

fn sum_grid(line: &Vec<String>) -> usize {
    let mut total = 0;
    for cell in line {
        if *cell != "^" && *cell != "." {
            let val: usize = cell.parse().unwrap();
            total += val;
        }
    }
    total
}

fn main() {
    let mut grid = get_grid()
        .iter()
        .map(|row| row.iter().map(|&c| c.to_string()).collect())
        .collect();
    start(&mut grid);
    for i in 1..grid.len() - 1 {
        step(i, &mut grid);
    }
    let counter = sum_grid(&grid[grid.len() - 1]);
    println!("{}", counter);
}
