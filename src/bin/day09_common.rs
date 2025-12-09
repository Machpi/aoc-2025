pub type Point = (i64, i64);

pub fn area(a: &Point, b: &Point) -> i64 {
    ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1)
}

pub fn get_red_tiles() -> Vec<Point> {
    include_str!("../../inputs/day09.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: i64 = parts.next().unwrap().trim().parse().unwrap();
            let y: i64 = parts.next().unwrap().trim().parse().unwrap();
            (x, y)
        })
        .collect::<Vec<Point>>()
}
