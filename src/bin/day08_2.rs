mod day08_common;
use day08_common::*;
use disjoint_sets::UnionFind;
use std::collections::HashMap;

fn connect_until_connected(
    points: Vec<(i64, i64, i64)>,
    connections: &Vec<((i64, i64, i64), (i64, i64, i64))>,
) -> ((i64, i64, i64), (i64, i64, i64)) {
    let index: HashMap<Point, usize> = points
        .iter()
        .enumerate()
        .map(|(i, p)| (p.clone(), i))
        .collect();
    let mut uf = UnionFind::new(points.len());
    let mut sets_remaining = points.len();
    let mut last = None;
    for (a, b) in connections {
        let ia = index[a];
        let ib = index[b];
        if !uf.equiv(ia, ib) {
            uf.union(ia, ib);
            sets_remaining -= 1;
            last = Some((*a, *b));
        }
        if sets_remaining == 1 {
            break;
        }
    }
    last.unwrap()
}

fn main() {
    let points = include_str!("../../inputs/day08.txt")
        .lines()
        .map(|line| {
            let parts = line
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (parts[0], parts[1], parts[2])
        })
        .collect::<Vec<Point>>();
    let connections = find_connections(&points);
    let last_connection = connect_until_connected(points.clone(), &connections);
    let res = last_connection.0.0 * last_connection.1.0;
    println!("{}", res);
}
