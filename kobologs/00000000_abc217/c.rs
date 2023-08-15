#[macro_use] extern crate text_io;
// use std::cmp::{max, min};
//use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let n: usize = read!();
    let p: Vec<usize> = (0..n).map(|_| read!()).collect();
    let mut q: Vec<usize> = vec![0; n];
    for (i, pp) in p.iter().enumerate() {
        q[pp - 1] = i + 1;
    }
    for qq in q {
        print!("{} ", qq);
    }
    println!();
}
