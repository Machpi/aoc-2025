mod day09_common;
use day09_common::*;
use std::cmp::{max, min};

fn edges(a: &Point, b: &Point) -> (i64, i64, i64, i64) {
    let xmin = min(a.0, b.0);
    let xmax = max(a.0, b.0);
    let ymin = min(a.1, b.1);
    let ymax = max(a.1, b.1);
    (xmin, xmax, ymin, ymax)
}

fn all_rectangles(points: &Vec<Point>) -> Vec<(Point, Point)> {
    let mut rects = Vec::new();
    let n = points.len();
    for (i, p) in points[..n - 1].iter().enumerate() {
        for q in points[i + 1..].iter() {
            rects.push((*p, *q));
        }
    }
    rects
}

fn rectangle_is_valid(a: &Point, b: &Point, red: &Vec<Point>) -> bool {
    let len = red.len();
    let (xmin, xmax, ymin, ymax) = edges(a, b);

    for i in 0..len {
        let p1 = red[i];
        let p2 = red[(i + 1) % len];
        // Vertical segment
        if p1.0 == p2.0 {
            let x = p1.0;
            let (ylmin, ylmax) = (min(p1.1, p2.1), max(p1.1, p2.1));
            if x > xmin && x < xmax {
                if !(ymax <= ylmin || ymin >= ylmax) {
                    return false;
                }
            }
        }
        // Horizontal segment
        else if p1.1 == p2.1 {
            let y = p1.1;
            let (xlmin, xlmax) = (min(p1.0, p2.0), max(p1.0, p2.0));
            if y > ymin && y < ymax {
                if !(xmax <= xlmin || xmin >= xlmax) {
                    return false;
                }
            }
        }
    }
    true
}

fn biggest_area(red: &Vec<Point>) -> i64 {
    let mut rects = all_rectangles(red);
    rects.sort_by_key(|(a, b)| -area(a, b));
    for (a, b) in rects {
        if rectangle_is_valid(&a, &b, red) {
            return area(&a, &b);
        }
    }
    0
}

fn main() {
    let points = get_red_tiles();
    let res = biggest_area(&points);
    println!("{}", res);
}
