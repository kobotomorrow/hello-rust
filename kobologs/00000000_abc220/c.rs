#[macro_use] extern crate text_io;
// use std::cmp::{max, min};
// use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let n: i64 = read!();
    let a: Vec<i64> = (0..n).map(|_| read!()).collect();
    let sum: i64 = a.iter().sum();
    let x: i64 = read!();
    let div: i64 = x / sum;
    let mut total: i64 = div * sum;
    let mut ans: i64 = div * n;

    for aa in a {
        total += aa;
        ans += 1;
        if total > x {
            break;
        }
    }
    println!("{}", ans);
}
