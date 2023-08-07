#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn main() {
    let a: i32 = read!();
    let b: i32 = read!();
    let c: i32 = read!();

    if c % 2 == 0 {
        let a = a.abs();
        let b = b.abs();
        if a > b {
            println!(">");
        } else if a < b {
            println!("<");
        } else {
            println!("=");
        }
    } else {
        if a > b {
            println!(">");
        } else if a < b {
            println!("<");
        } else {
            println!("=");
        }
    }
}