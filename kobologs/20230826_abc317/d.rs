#[macro_use] extern crate text_io;
// use std::cmp::max;
use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn main() {
    let n: usize = read!();
    let mut sum: usize = 0;
    let mut g: usize = 0;
    let mut x: Vec<i64> = Vec::new();
    let mut y: Vec<i64> = Vec::new();
    let mut z: Vec<usize> = Vec::new();
    for _ in 0..n {
        let xx: i64 = read!();
        let yy: i64 = read!();
        let zz: usize = read!();

        x.push(xx);
        y.push(yy);
        z.push(zz);

        sum += zz;
        if xx > yy {
            g += zz;
        }
    }

    if g > sum / 2 {
        println!("{}", 0);
        return;
    }

    let mut dp: Vec<Vec<i64>> = vec![vec![0; sum+1 as usize]; n+1];
    for i in 1..n+1 {
        let req = (y[i-1] - x[i-1]) / 2 + 1;
        let zz = z[i-1];
        for j in 0..sum+1 {
            if y[i-1] - x[i-1] <= 0 {
                dp[i][j] = dp[i-1][j];
            } else {
                if j == zz || (j > zz && dp[i-1][j-zz] > 0) {
                    dp[i][j] = if dp[i-1][j] == 0 {
                        dp[i-1][j-zz] + req
                    } else {
                        min(dp[i-1][j], dp[i-1][j-zz] + req)
                    };
                } else {
                    dp[i][j] = dp[i-1][j];
                }
            }
        }
    }

    let wp = sum / 2 + 1 - g;
    let mut ans = 1_000_000_000_000_000_000;
    for i in wp..sum+1 {
        if dp[n][i] > 0 {
            ans = min(ans, dp[n][i]);
        }
    }
    println!("{}", ans);
}