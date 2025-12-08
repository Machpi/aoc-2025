mod day05_common;
use day05_common::*;

fn within_range(value: u64, range: (u64, u64)) -> bool {
    value >= range.0 && value <= range.1
}

fn main() {
    let (ranges, ingredients) = get_data();

    let fresh_ingredients = ingredients
        .iter()
        .filter(|&ingredient| ranges.iter().any(|&range| within_range(*ingredient, range)))
        .count();
    println!("{}", fresh_ingredients);
}
