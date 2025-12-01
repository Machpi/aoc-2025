pub struct Rotation {
    pub direction: char,
    pub value: i32,
}

pub fn parse_rotation(s: &str) -> Rotation {
    let direction = s.chars().next().unwrap();
    let value: i32 = s[1..].parse().unwrap();
    Rotation { direction, value }
}

pub fn simulate_rotations<F>(rotations: &Vec<Rotation>, mut simulate: F) -> i32
where
    F: FnMut(&Rotation, &mut i32, &mut i32),
{
    let mut dial = 50;
    let mut counter = 0;
    rotations.iter().for_each(|r| simulate(r, &mut dial, &mut counter));
    counter
}
