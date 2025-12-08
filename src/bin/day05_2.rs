mod day05_common;
use day05_common::*;

fn add_ranges(ranges: &Vec<(u64, u64)>) -> u64 {
    let mut total = 0;
    let mut current_id = 0;
    for r in ranges {
        if current_id >= r.1 {
            // Range included in count
            continue;
        } else if current_id >= r.0 {
            // Range overlap
            total += r.1 - current_id;
            current_id = r.1;
        } else {
            // New range
            total += r.1 - r.0 + 1;
            current_id = r.1;
        }
    }
    total
}

fn main() {
    let (ranges, _) = get_data();
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let fresh_ingredients = add_ranges(&sorted_ranges);
    println!("{}", fresh_ingredients);
}
