mod day06_common;
use day06_common::*;

pub fn get_operations() -> Vec<Operation> {
    let lines: Vec<&str> = include_str!("../../inputs/day06.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();
    let mut result = Vec::new();
    let ops = lines[lines.len() - 1]
        .split_whitespace()
        .collect::<Vec<&str>>();
    let mut cepha = vec![String::new(); lines[0].len()];
    for line in lines[..lines.len() - 1].iter() {
        let chars = line.chars().collect::<Vec<char>>();
        for (i, c) in chars.iter().enumerate() {
            if *c == ' ' {
                continue;
            }
            cepha[i].push(*c);
        }
    }
    let mut curr_set = 0;
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    numbers.push(Vec::new());
    for c in cepha {
        if c == "" {
            curr_set += 1;
            numbers.push(Vec::new());
            continue;
        }
        let num = c.parse::<i64>().unwrap();
        numbers[curr_set].push(num);
    }
    for (i, nums) in numbers.into_iter().enumerate() {
        result.push(Operation {
            numbers: nums,
            operation: ops[i].chars().next().unwrap(),
        });
    }
    result
}

fn main() {
    let operations = get_operations();
    let result = operations
        .iter()
        .map(|op| process_operation(op))
        .sum::<i64>();
    println!("{}", result);
}
