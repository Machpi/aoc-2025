mod day02_common;
use day02_common::*;

fn divisors(n: usize) -> Vec<usize> {
    let mut divs = Vec::new();
    if n <= 1 {
        return divs;
    }
    let limit = (n as f64).sqrt() as usize;
    for d in 1..=limit {
        if n % d == 0 {
            divs.push(d);
            let other = n / d;
            if other != d {
                divs.push(other);
            }
        }
    }
    divs
}

fn is_repeated_n(s: &str, n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    let len = s.len();
    if len % n != 0 {
        return false;
    }
    let seq_len = len / n;
    let bytes = s.as_bytes();
    for i in 0..seq_len {
        for j in 0..n {
            if bytes[i] != bytes[i + j * seq_len] {
                return false;
            }
        }
    }
    true
}

fn is_invalid(i: i64) -> bool {
    let s = i.to_string();
    let divs = divisors(s.len());
    for d in divs {
        if is_repeated_n(&s, d) {
            return true;
        }
    }
    false
}

fn is_invalid_alt(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();
    for d in 2..=len {
        if len % d != 0 {
            continue;
        }
        let k = len / d;
        let ten_k = pow10(k);
        let ten_l = pow10(len);
        let r = (ten_l - 1) / (ten_k - 1);
        if n % r == 0 {
            return true;
        }
    }
    false
}

use std::time::Instant;
fn main() {
    let repeats = 10;

    // Benchmark is_invalid
    let mut total_duration = std::time::Duration::ZERO;
    let mut res = 0;
    for _ in 0..repeats {
        let start = Instant::now();
        res = process_file(is_invalid);
        total_duration += start.elapsed();
    }
    println!(
        "is_invalid: {} (avg over {} runs: {:?})",
        res,
        repeats,
        total_duration / repeats as u32
    );

    // Benchmark is_invalid_alt
    total_duration = std::time::Duration::ZERO;
    let mut res_alt = 0;
    for _ in 0..repeats {
        let start = Instant::now();
        res_alt = process_file(is_invalid_alt);
        total_duration += start.elapsed();
    }
    println!(
        "is_invalid_alt: {} (avg over {} runs: {:?})",
        res_alt,
        repeats,
        total_duration / repeats as u32
    );
}
