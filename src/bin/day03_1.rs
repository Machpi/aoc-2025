mod day03_common;
use day03_common::*;

fn find_joltage(numbers: &Vec<i64>) -> i64 {
    let n = numbers.len();
    let (greatest_index, &greatest) = numbers[..n - 1]
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, &num)| num)
        .unwrap();
    let (_, &second) = numbers[greatest_index + 1..]
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, &num)| num)
        .unwrap();
    10 * greatest + second
}

fn main() {
    let result = process_file(find_joltage);
    println!("{}", result);
}
