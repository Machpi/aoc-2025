mod day03_common;
use day03_common::*;

fn find_joltage(numbers: &Vec<i64>) -> i64 {
    let mut nums = numbers.clone();
    while nums.len() > 12 {
        for i in 0..nums.len() {
            if i == nums.len() - 1 {
                nums.remove(i);
                break;
            } else if nums[i] < nums[i + 1] {
                nums.remove(i);
                break;
            }
        }
    }
    nums.iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}

fn main() {
    let result = process_file(find_joltage);
    println!("{}", result);
}
