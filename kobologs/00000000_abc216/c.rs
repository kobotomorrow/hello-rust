#[macro_use] extern crate text_io;
// use std::cmp::{max, min};
//use std::collections::{HashMap, VecDeque, HashSet};

fn rec(x: i64, res: &mut Vec<char>) {
    if x == 0 {
        return;
    }

    if x % 2 == 0 {
        res.push('B');
        rec(x / 2, res);
    } else {
        res.push('A');
        rec(x - 1, res);
    }
}

fn main() {
    let n: i64 = read!();
    let mut res = Vec::new();
    rec(n, &mut res);
    res.reverse();
    println!("{}", res.iter().collect::<String>());
}
