#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let n: usize = read!();
    let m: usize = read!();
    let k: usize = read!();

    let md = 998244353;

    let mut dp = vec![vec![0 as i64; k+1]; n+1];
    dp[0][0] = 1;
    
    for i in 1..=n {
        for j in 1..=k {
            let mut cnt = 0;
            for kk in 1..=m {
                if j as i32 - kk as i32 >= 0 {
                    cnt += dp[i-1][j-kk];
                }
            }
            dp[i][j] = cnt % md;
        }
    }

    let mut ans = 0;
    for i in 0..=k {
        ans += dp[n][i];
        ans %= md;
    }
    println!("{}", ans);
}