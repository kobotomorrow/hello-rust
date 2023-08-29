#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn main() {
    let n: usize = read!();
    let k: usize = read!();
    let k = k - 1;
    let mut p = Vec::new();
    for _ in 0..n {
        let mut sum = 0;
        let a: i32 = read!();
        let b: i32 = read!();
        let c: i32 = read!();
        sum += a + b + c;
        p.push(sum);
    }
    let mut q = p.clone();
    q.sort_by(|a, b| b.cmp(a));
    for pp in p {
        if pp + 300 >= q[k] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}