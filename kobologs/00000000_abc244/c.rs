use std::hash::Hash;

#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let n: usize = read!();
    let nn: usize = 2 * n + 1;

    let mut selected = HashSet::new();

    loop {
        for i in 1..=nn {
            if !selected.contains(&i) {
                selected.insert(i);
                println!("{}", i);
                break;
            }
        }

        let sc: usize = read!();
        if sc == 0 {
            return;
        }
        selected.insert(sc);
    }
}