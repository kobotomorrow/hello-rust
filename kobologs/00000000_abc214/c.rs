#[macro_use] extern crate text_io;
use std::cmp::{max, min};
// use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let n: usize = read!();
    let s: Vec<i64> = (0..n).map(|_| read!()).collect();
    let t: Vec<i64> = (0..n).map(|_| read!()).collect();
    let mut dp: Vec<i64> = vec![0; n];
    dp[0] = t[0];
    for i in 1..n {
        dp[i] = min(dp[i-1] + s[i-1], t[i]);
    }
    dp[0] = min(dp[n-1] + s[n-1], dp[0]);
    for i in 1..n {
        dp[i] = min(dp[i-1] + s[i-1], t[i]);
    }


    for i in 0..n {
        println!("{}", dp[i]);
    }
}