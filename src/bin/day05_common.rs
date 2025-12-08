pub fn get_data() -> (Vec<(u64, u64)>, Vec<u64>) {
    let input = include_str!("../../inputs/day05.txt")
        .lines()
        .collect::<Vec<&str>>();
    let empty_line_index = input.iter().position(|line| line.is_empty()).unwrap_or(0);
    let ranges = input[..empty_line_index]
        .to_vec()
        .iter()
        .map(|range| {
            let parts: Vec<u64> = range
                .split('-')
                .map(|part| part.parse::<u64>().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .collect::<Vec<(u64, u64)>>();
    let ingredients = input[empty_line_index + 1..]
        .to_vec()
        .iter()
        .map(|ingredient| ingredient.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    (ranges, ingredients)
}
