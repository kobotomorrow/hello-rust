#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let x1: i64 = read!();
    let y1: i64 = read!();
    let x2: i64 = read!();
    let y2: i64 = read!();

    for i in -2..=2 {
        for j in -2..=2 {
            let l1 = (x1 - (x1 + i)).abs();
            let l2 = (y1 - (y1 + j)).abs();
            let r1 = (x2 - (x1 + i)).abs();
            let r2 = (y2 - (y1 + j)).abs();
            if l1 * l1 + l2 * l2 == 5 && r1 * r1 + r2 * r2 == 5 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No")
}