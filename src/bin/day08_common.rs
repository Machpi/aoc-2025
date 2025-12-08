pub type Point = (i64, i64, i64);

pub fn dist(a: Point, b: Point) -> f64 {
    let dx = (a.0 - b.0) as f64;
    let dy = (a.1 - b.1) as f64;
    let dz = (a.2 - b.2) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn find_connections(points: &Vec<Point>) -> Vec<(Point, Point)> {
    let mut dist_arr = Vec::new();
    let len = points.len();
    for (i, point1) in points[..len - 1].iter().enumerate() {
        for point2 in points[i + 1..].iter() {
            dist_arr.push((dist(*point1, *point2), (*point1, *point2)));
        }
    }
    dist_arr.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    dist_arr.iter().map(|(_, p)| p).cloned().collect()
}