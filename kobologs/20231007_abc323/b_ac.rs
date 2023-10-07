#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let n: usize = read!();
    let s: Vec<String> = (0..n).map(|_| read!()).collect();

    let mut vct = Vec::new();
    for i in 0..n {
        let mut cnt = 0;
        for c in s[i].chars() {
            if c == 'o' {
                cnt += 1;
            }
        }
        vct.push(cnt);
    }
    let mut vct_i = Vec::new();
    for i in 0..n {
        vct_i.push((vct[i], i+1));
    }
    vct_i.sort_by(|a, b| b.0.cmp(&a.0));
    
    for x in vct_i {
        print!("{} ", x.1);
    }
    println!();
}