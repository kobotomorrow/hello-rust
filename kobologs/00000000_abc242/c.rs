#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let md = 998244353;
    let n: usize = read!();
    let mut dp: Vec<Vec<i64>> = vec![vec![-1; 9]; n];
    for i in 0..9 {
        dp[0][i] = 1;
    }

    for i in 1..n {
        for j in 0..9 {
            if j == 0 {
                dp[i][j] = (dp[i-1][j] + dp[i-1][j+1]) % md;
            } else if j == 8 {
                dp[i][j] = (dp[i-1][j-1] + dp[i-1][j]) % md;
            } else {
                dp[i][j] = (dp[i-1][j-1] + dp[i-1][j] + dp[i-1][j+1]) % md;
            }
        }
    }
    println!("{}", dp[n-1].iter().sum::<i64>() % md);
}
