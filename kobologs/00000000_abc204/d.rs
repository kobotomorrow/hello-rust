#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn main() {
    let n: usize = read!();
    let t: Vec<usize> = (0..n).map(|_| read!()).collect();
    let sum = t.iter().sum::<usize>();
    let mut dp = vec![vec![false; sum+1]; n+1];

    dp[0][0] = true;
    for i in 1..n+1 {
        for j in 0..sum+1 {
            if dp[i-1][j] {
                dp[i][j] = true;
            }
            if j >= t[i-1] && dp[i-1][j - t[i-1]] {
                dp[i][j] = true;
            }
        }
    }
    
    let mid = if sum % 2 == 0 { sum / 2 } else { sum / 2 + 1 };
    for i in 0..mid+1 {
        if dp[n][mid + i] {
            println!("{}", mid + i);
            return;
        }
    }
}