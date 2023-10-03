#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let n: usize = read!();
    let mut k: usize = read!();
    let x: usize = read!();
    let mut a: Vec<usize> = (0..n).map(|_| read!()).collect();

    for aa in a.iter_mut() {
        let d = *aa / x;

        if k < d {
            *aa -= k * x;
            k = 0;
        } else {
            k -= d;
            *aa -= d * x;
        }

        if k == 0 {
            break;
        }
    }
    
    a.sort_by(|a, b| b.cmp(a));
    let mut sum = 0;
    for i in k..n {
        sum += a[i];
    }
    println!("{}", sum);
}