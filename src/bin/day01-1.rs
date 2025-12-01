mod day01_common;
use day01_common::*;

fn simulate_rotation(rotation: &Rotation, dial: &mut i32, counter: &mut i32) {
    match rotation.direction {
        'L' => *dial -= rotation.value,
        'R' => *dial += rotation.value,
        _ => panic!("Invalid direction"),
    }
    *dial %= 100;
    if *dial == 0 {
        *counter += 1;
    }
}

fn main() {
    let rotations = include_str!("../../inputs/day01.txt")
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|&line| parse_rotation(line))
        .collect();
    let password = simulate_rotations(&rotations, simulate_rotation);
    println!("Password: {}", password);
}
