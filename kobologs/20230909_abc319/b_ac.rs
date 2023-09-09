#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::{collections::HashMap};
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn main() {
    let n: usize = read!();
    let mut yn = Vec::new();
    for i in 1..10 {
        if n % i == 0 {
            yn.push(i);
        }
    }
    let mut bn = Vec::new();
    for i in yn.iter() {
        bn.push(n / i);
    }

    print!("{}", 1);

    for i in 1..n+1 {
        let mut ok = false;
        for j in bn.iter() {
            if i % j == 0 {
                print!("{}", n/j);
                ok = true;
                break;
            }
        }
        if !ok {
            print!("{}", '-');
        }
    }
    println!();
}