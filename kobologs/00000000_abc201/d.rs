#[macro_use] extern crate text_io;
use std::cmp::max;
use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn rec(dp: &mut Vec<Vec<i32>>, a: &Vec<Vec<i32>>, h: i32, w: i32, i: usize, j: usize) -> i32 {
    if dp[i][j] != -1 {
        return dp[i][j];
    }

    let ii = i as i32;
    let jj = j as i32;
    if (i + j) % 2 == 0 {
        dp[i][j] = if ii == h -1 {
            rec(dp, a, h, w, i, j + 1) + a[i][j + 1]
        } else if jj == w - 1 {
            rec(dp, a, h, w, i + 1, j) + a[i + 1][j]
        } else {
            max(rec(dp, a, h, w, i + 1, j) + a[i + 1][j], rec(dp, a, h, w, i, j + 1) + a[i][j + 1])
        }
    } else {
        dp[i][j] = if ii == h -1 {
            rec(dp, a, h, w, i, j + 1) - a[i][j + 1]
        } else if jj == w - 1 {
            rec(dp, a, h, w, i + 1, j) - a[i + 1][j]
        } else {
            min(rec(dp, a, h, w, i + 1, j) - a[i + 1][j], rec(dp, a, h, w, i, j + 1) - a[i][j + 1])
        }
    }

    return dp[i][j];
}


fn main() {
    let h: i32 = read!();
    let w: i32 = read!();
    let mut a: Vec<Vec<i32>> = Vec::new();
    for _i in 0..h {
        let mut v = vec![];
        let vv: String = read!();
        for c in vv.chars() {
            if c == '+' {
                v.push(1);
            } else {
                v.push(-1);
            }
        }
        a.push(v);
    }

    let mut dp = vec![vec![-1; w as usize]; h as usize];
    dp[h as usize - 1][w as usize - 1] = 0;
    rec(&mut dp, &a, h, w, 0, 0);
    if dp[0][0] > 0 {
        println!("Takahashi");
    } else if dp[0][0] < 0 {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}