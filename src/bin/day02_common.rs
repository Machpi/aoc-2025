pub fn process_file<F>(mut is_invalid: F) -> i64
where
    F: FnMut(i64) -> bool,
{
    include_str!("../../inputs/day02.txt")
        .split(',')
        .flat_map(|range| {
            let mut nums = range.split('-').map(|n| n.parse::<i64>().unwrap());
            let start = nums.next().unwrap();
            let end = nums.next().unwrap();
            start..=end
        })
        .filter(|&value| is_invalid(value))
        .sum()
}

pub fn pow10(n: usize) -> i64 {
    10_i64.pow(n as u32)
}