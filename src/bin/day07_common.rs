pub fn get_grid() -> Vec<Vec<char>> {
    include_str!("../../inputs/day07.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}
