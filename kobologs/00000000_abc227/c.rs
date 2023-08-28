#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn main() {
    let n: i64 = read!();
    let mut ans = 0;
    for i in 1..n+1 {
        if i * i * i > n {
            break;
        }
        for j in i..n+1 {
            if i * j * j > n {
                break;
            }
            ans += n/(i*j) - j + 1;
        }
    } 
    println!("{}", ans);
}