mod day11_common;
use day11_common::*;
use std::collections::HashMap;

pub fn dfs(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    path: &mut Vec<String>,
    paths: &mut Vec<Vec<String>>,
) {
    path.push(current.to_string());
    if current == target {
        paths.push(path.clone());
    } else if let Some(neighbors) = graph.get(current) {
        for next in neighbors {
            dfs(graph, next, target, path, paths);
        }
    }
    path.pop();
}

fn main() {
    let graph = get_graph();
    let mut paths = Vec::<Vec<String>>::new();
    dfs(&graph, "you", "out", &mut Vec::new(), &mut paths);
    println!("{}", paths.len());
}
