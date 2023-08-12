#[macro_use] extern crate text_io;
// use std::cmp::{max, min};
//use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let n: usize = read!();
    let mut v: Vec<(usize, usize, Vec<i32>)> = vec![];
    for i in 0..n {
        let c: usize = read!();
        let mut v2: Vec<i32> = vec![];
        for _ in 0..c {
            let a: usize = read!();
            v2.push(a as i32);
        }
        v.push((i+1, c, v2));
    }
    let x: i32 = read!();

    let mut result = vec![];
    for i in 0..n {
        for j in 0..v[i].1 {
            if v[i].2[j] == x {
                result.push((v[i].1, v[i].0));
            }
        }
    }
    if result.len() == 0 {
        println!("0");
        return;
    }

    result.sort();
    let xx = result[0].0;
    let mut ans = vec![];
    for i in 0..result.len() {
        if result[i].0 == xx {
            ans.push(result[i].1);
        }
    }

    println!("{}", ans.len());
    for aa in ans {
        print!("{} ", aa);
    }
    println!("");
}
