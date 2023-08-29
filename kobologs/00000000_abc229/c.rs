#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn main() {
    let n: usize = read!();
    let mut w: i64 = read!();
    let mut ch = Vec::new();
    for _ in 0..n {
        let a: i64 = read!();
        let b: i64 = read!();
        ch.push((a, b));
    }
    ch.sort_by(|a, b| b.0.cmp(&a.0));
    let mut sum: i64 = 0;
    for chch in ch {
        if chch.1 > w {
            for i in 0..chch.1 {
                if chch.1 - i <= w {
                    sum += chch.0 * (chch.1 - i);
                    break;
                }
            }
            break;
        }
        w -= chch.1;
        sum += chch.0 * chch.1;
    }
    println!("{}", sum);
}