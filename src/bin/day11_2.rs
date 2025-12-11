mod day11_common;
use day11_common::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    None,
    Dac,
    Fft,
    Both,
}

fn next_state(state: State, node: &str) -> State {
    match (state, node) {
        (State::None, "dac") => State::Dac,
        (State::None, "fft") => State::Fft,
        (State::Dac, "fft") => State::Both,
        (State::Fft, "dac") => State::Both,
        _ => state,
    }
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    node: &str,
    state: State,
    memo: &mut HashMap<(String, State), u64>,
) -> u64 {
    let key = (node.to_string(), state);
    if let Some(v) = memo.get(&key) {
        return *v;
    }
    if node == "out" {
        return match state {
            State::Both => 1,
            _ => 0,
        };
    }
    let mut total = 0;
    if let Some(neighbors) = graph.get(node) {
        let new_state = next_state(state, node);
        for next in neighbors {
            total += count_paths(graph, next, new_state, memo);
        }
    }
    memo.insert(key, total);
    total
}

fn main() {
    let graph = get_graph();
    let mut memo = HashMap::new();
    let result = count_paths(&graph, "svr", State::None, &mut memo);
    println!("{}", result);
}
