#[macro_use] extern crate text_io;
// use std::cmp::{max, min};
//use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let r: &str = "1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
    let n: usize = read!();

    print!("3.");
    for rr in r.chars().take(n) {
        print!("{}", rr);
    }
    println!("");
}
