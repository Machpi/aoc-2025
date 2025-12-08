pub fn check_adjacent(mat: &Vec<Vec<bool>>, i: usize, j: usize) -> usize {
    let rows = mat.len();
    let cols = mat[0].len();
    let mut count = 0;
    for di in -1..=1 {
        for dj in -1..=1 {
            if di != 0 || dj != 0 {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;
                if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                    if mat[ni as usize][nj as usize] {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

pub fn get_mat() -> Vec<Vec<bool>> {
    include_str!("../../inputs/day04.txt")
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect::<Vec<bool>>())
        .collect()
}
