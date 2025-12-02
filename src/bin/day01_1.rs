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
    let password = process_file(simulate_rotation);
    println!("Password: {}", password);
}
