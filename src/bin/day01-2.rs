mod day01_common;
use day01_common::*;

fn simulate_rotation(rotation: &Rotation, dial: &mut i32, counter: &mut i32) {
    let distance = rotation.value;
    match rotation.direction {
        'R' => {
            *counter += (*dial + distance) / 100;
            *dial = (*dial + distance) % 100;
        }
        'L' => {
            let flipped = (100 - *dial) % 100;
            *counter += (flipped + distance) / 100;
            *dial = (*dial - distance) % 100;
            *dial = (*dial + 100) % 100;
        }
        _ => panic!("Invalid direction"),
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
