#[macro_use] extern crate text_io;
use  std::cmp::{max, min};
// use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    const AJ: i64 = 100000;
    let n: usize = read!();
    let mut a: Vec<i64> = Vec::new();
    let mut b: Vec<i64> = Vec::new();
    for i in 0..n {
        a.push(read!());
        b.push(read!());
    }
    
    let mut t: i64 = 0; // 時間
    let mut ans: i64 = 0; // 距離
    for i in 0..n {
        t += AJ * a[i] / b[i];
    }
    t /= 2;

    println!("t: {}", t);
    println!();
    for i in 0..n {
        ans += min(AJ * a[i], t * b[i]);
        t -= min(t, AJ * a[i] / b[i]);
        println!("ans: {}", ans);
        println!("t: {}", t);
    }
    println!("{}", ans as f64 / AJ as f64);
}
