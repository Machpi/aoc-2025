mod day02_common;
use day02_common::*;

pub fn is_invalid(i: i64) -> bool {
    let string = i.to_string();
    if string.len() % 2 != 0 {
        return false;
    }
    let (first_half, second_half) = string.split_at(string.len() / 2);
    first_half == second_half
}

pub fn is_invalid_alt(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    let len = s.len();
    let k = len / 2;
    let ten_k = pow10(k);
    let ten_l = pow10(len);
    let r = (ten_l - 1) / (ten_k - 1);
    if n % r == 0 {
        return true;
    }
    false
}

fn main() {
    let res = process_file(is_invalid);
    println!("{}", res);
}
