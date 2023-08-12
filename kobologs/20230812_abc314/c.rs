#[macro_use] extern crate text_io;
// use std::cmp::{max, min};
//use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let n: usize = read!();
    let m: usize = read!();
    let mut x = vec![vec![]; m];
    let s: String = read!();
    for (i, c) in s.chars().enumerate() {
        let mut cl: usize = read!();
        cl -= 1;
        x[cl].push((c, i));
    }
    
    for i in 0..m {
        let h = x[i][0].1;
        let size = x[i].len()-1;
        for j in 0..size {
            x[i][j].1 = x[i][j+1].1;
        }
        x[i][size].1 = h;
    }

    let mut ans = vec!['.'; n];
    for i in 0..m {
        for j in 0..x[i].len() {
            ans[x[i][j].1] = x[i][j].0;
        }
    }
    for i in 0..n {
        print!("{}", ans[i]);
    }
    println!();
}
