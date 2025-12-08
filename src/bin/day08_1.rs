mod day08_common;
use day08_common::*;

fn find_circuits(connections: &Vec<(Point, Point)>) -> Vec<Vec<Point>> {
    let mut circuits = Vec::new();
    // DFS
    let mut visited = std::collections::HashSet::new();
    for (start, _) in connections.iter() {
        if visited.contains(start) {
            continue;
        }
        let mut stack = vec![*start];
        let mut circuit = Vec::new();
        while let Some(node) = stack.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            circuit.push(node);
            for (p1, p2) in connections.iter() {
                if *p1 == node && !visited.contains(p2) {
                    stack.push(*p2);
                } else if *p2 == node && !visited.contains(p1) {
                    stack.push(*p1);
                }
            }
        }
        if !circuit.is_empty() {
            circuits.push(circuit);
        }
    }
    circuits
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
    let connections = find_connections(&points)
        .iter()
        .take(1000)
        .cloned()
        .collect::<Vec<(Point, Point)>>();
    let circuits = find_circuits(&connections);
    let mut lengths: Vec<usize> = circuits.iter().map(|circuit| circuit.len()).collect();
    lengths.sort_by(|a, b| b.cmp(a));
    let res = lengths.iter().take(3).product::<usize>();
    println!("{}", res);
}
