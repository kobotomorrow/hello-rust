#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let n: String = read!();
    let mut x = -1;
    for c in n.chars() {
        let i = c as i32 - '0' as i32;
        if x == -1 {
            x = i;
        } else {
            if x <= i {
                println!("No");
                return;
            }
            x = i;
        }
    }
    println!("Yes");
}
