pub fn process_file<F>(mut find_joltage: F) -> i64
where
    F: FnMut(&Vec<i64>) -> i64,
{
    include_str!("../../inputs/day03.txt")
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<i64>>()
        })
        .map(|numbers| find_joltage(&numbers))
        .sum()
}
