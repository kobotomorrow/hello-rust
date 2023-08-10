#[macro_use] extern crate text_io;
// use std::cmp::{max, min};
// use std::collections::{HashMap, VecDeque, HashSet};

fn gen(t: &mut Vec<(i32, usize, usize)>, n: usize) {
    t.sort();
    let mut idx = 1;
    for i in 0..n {
        if i > 0 && t[i-1].0 != t[i].0 {
            idx += 1;
        }
        t[i].2 = idx;
    }
    t.sort_by(|a, b| a.1.cmp(&b.1));
}

fn main() {
    let _h: i32 = read!();
    let _w: i32 = read!();
    let n: usize = read!();
    let mut y: Vec<(i32, usize, usize)> = Vec::new();
    let mut x: Vec<(i32, usize, usize)> = Vec::new();
    for i in 0..n {
        let a: i32 = read!();
        let b: i32 = read!();
        y.push((a, i, 0));
        x.push((b, i, 0));
    }
    gen(&mut y, n);
    gen(&mut x, n);
    for i in 0..n {
        println!("{} {}", y[i as usize].2, x[i as usize].2);
    }
}