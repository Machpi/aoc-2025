mod day09_common;
use day09_common::*;

fn biggest_area(points: &Vec<Point>) -> i64 {
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let area_ij = area(&points[i], &points[j]);
            if area_ij > max_area {
                max_area = area_ij;
            }
        }
    }
    max_area
}

fn main() {
    let input = get_red_tiles();
    let result = biggest_area(&input);
    println!("{}", result);
}
