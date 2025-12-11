use std::collections::HashMap;

pub fn parse_line(line: &str) -> (String, Vec<String>) {
    let mut parts = line.split(':');
    let node = parts.next().unwrap().trim().to_string();
    let neighbors = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    (node, neighbors)
}

pub fn get_graph() -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    let lines = include_str!("../../inputs/day11.txt").lines();
    for line in lines {
        let (node, neighbors) = parse_line(line);
        graph.insert(node, neighbors);
    }
    graph
}
