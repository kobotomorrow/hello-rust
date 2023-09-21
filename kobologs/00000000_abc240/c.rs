#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let n: usize = read!();
    let x: usize = read!();
    let mut dp = vec![vec![false; 10_001];n+1];
    dp[0][0] = true;

    for i in 0..n {
        let a: usize = read!();
        let b: usize = read!();
        for j in 0..=10_000 {
            if dp[i][j] {
                dp[i+1][j+a] = true;
                dp[i+1][j+b] = true;
            }
        }
    }

    if dp[n][x] {
        println!("Yes")
    } else {
        println!("No")
    }
}