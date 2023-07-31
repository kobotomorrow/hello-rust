// Tessoku: A06
#[macro_use] extern crate text_io;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn main() {
    let n: i32 = read!();
    let q: i32 = read!();

    let mut a = vec![0; (n + 1) as usize];
    a[0] = 0;
    for i in 1..n+1 {
        let aa: i32 = read!();
        let ii = i as usize;
        a[ii] = a[ii - 1] + aa;
    }

    for _ in 0..q {
        let l: i32 = read!();
        let r: i32 = read!();
        
        let ans = a[r as usize] - a[l as usize - 1];
        println!("{}", ans);
    }
}