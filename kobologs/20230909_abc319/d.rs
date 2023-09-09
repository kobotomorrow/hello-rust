#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::{collections::HashMap};
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn line_count(s: &Vec<i64>, w: i64) -> i64 {
    let mut l_cnt = 1;
    let mut w_cnt = 0;
    for ss in s {
        if w_cnt + ss <= w {
            w_cnt += ss;
        } else {
            l_cnt += 1;
            w_cnt = *ss;
        }
        w_cnt += 1;
    }

    return l_cnt;
}

fn main() {
    let n: i64 = read!();
    let m: i64 = read!();
    let s: Vec<i64> = (0..n).map(|_| read!()).collect();

    let mut l = *s.iter().max().unwrap() - 1;
    let mut r: i64 = s.iter().sum();
    r = r + n - 1;

    while l+1 < r {
        let mid = (l + r) / 2;
        if line_count(&s, mid) <= m {
            r = mid;
        } else {
            l = mid;
        }
    }
    println!("{}", r);
}