#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let n: usize = read!();
    let x: i32 = read!();
    let mut a: Vec<i32> = (0..n-1).map(|_| read!()).collect();
    a.sort();

    let aa: Vec<i32> = a[0..n-2].to_vec();
    let sum: i32 = aa.iter().sum();
    if sum >= x {
        println!("{}", 0);
        return;
    }

    let aa: Vec<i32> = a[1..n-1].to_vec();
    let sum: i32 = aa.iter().sum();
    let m_sum: i32 = aa[0..n-3].iter().sum();
    if sum >= x {
        println!("{}", x - m_sum);
        return;
    }

    println!("{}", -1);
}
