mod day06_common;
use day06_common::*;

fn get_operations(mat: &Vec<Vec<&str>>) -> Vec<Operation> {
    let rows = mat.len();
    let cols = mat[0].len();
    let mut operations = Vec::new();
    for i in 0..cols {
        let mut numbers = Vec::new();
        for j in 0..rows - 1 {
            numbers.push(mat[j][i].parse::<i64>().unwrap());
        }
        operations.push(Operation {
            numbers,
            operation: mat[rows - 1][i].chars().next().unwrap(),
        });
    }
    operations
}

pub fn get_mat() -> Vec<Vec<&'static str>> {
    include_str!("../../inputs/day06.txt")
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&'static str>>())
        .filter(|v| !v.is_empty())
        .collect::<Vec<Vec<&'static str>>>()
}

fn main() {
    let mat = get_mat();
    let operations = get_operations(&mat);
    let result = operations
        .iter()
        .map(|op| process_operation(op))
        .sum::<i64>();
    println!("{}", result);
}
