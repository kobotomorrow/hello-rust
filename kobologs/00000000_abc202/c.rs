#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;



fn main() {
    let n: i32 = read!();
    let a: Vec<usize> = (0..n).map(|_| read!()).collect();
    let b: Vec<usize> = (0..n).map(|_| read!()).collect();
    let c: Vec<usize> = (0..n).map(|_| read!()).collect();
    
    let mut count: Vec<i64> = vec![0; n as usize];
    for cc in c {
        count[b[cc-1]-1] += 1;
    }

    let mut ans: i64 = 0;
    for aa in a {
        ans += count[aa-1];
    }
    println!("{}", ans);
}