mod day04_common;
use day04_common::*;

fn main() {
    let mut mat = get_mat();
    let rows = mat.len();
    let cols = mat[0].len();
    let mut count = 0;
    let mut indexes_to_remove = vec![];
    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        for i in 0..rows {
            for j in 0..cols {
                if mat[i][j] {
                    if check_adjacent(&mat, i, j) < 4 {
                        count += 1;
                        indexes_to_remove.push((i, j));
                        has_changed = true;
                    }
                }
            }
        }
        for (i, j) in &indexes_to_remove {
            mat[*i][*j] = false;
        }
        indexes_to_remove.clear();
    }
    println!("{}", count);
}
