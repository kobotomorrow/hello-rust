
#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::{collections::HashSet, hash::Hash};

fn main() {
    let s: String = read!();
    let s = s.chars().collect::<Vec<char>>();
    let a = ['c', 'h', 'o', 'k', 'u', 'd', 'a', 'i'];
    
    let mut dp: Vec<Vec<i64>> = vec![vec![0; 9]; s.len() + 1];
    for i in 0..s.len()+1{
        dp[i][0] = 1;
    }

    for i in 0..s.len() {
        for j in 0..8 {
            if a[j] == s[i] {
                dp[i+1][j+1] = dp[i][j+1] + dp[i][j] % 1000000007;
            } else {
                dp[i+1][j+1] = dp[i][j+1];
            }
        }
    }

    println!("{}", dp[s.len()][8] % 1000000007);
}