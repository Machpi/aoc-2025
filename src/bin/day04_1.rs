mod day04_common;
use day04_common::*;

fn main() {
    let mat = get_mat();
    let rows = mat.len();
    let cols = mat[0].len();
    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            if mat[i][j] {
                if check_adjacent(&mat, i, j) < 4 {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
