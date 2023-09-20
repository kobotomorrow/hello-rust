#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
use num_bigint::BigInt;

fn sum(n: BigInt) -> BigInt {
    return &n * (&n + 1) / 2;
}

fn main() {
    let md = 998244353;
    let n: BigInt = read!();
    let mut ans: BigInt = BigInt::from(0);
    let mut pre: BigInt = BigInt::from(0);

    for i in 1..=18 {
        let x = 10i64.pow(i) - 1i64;
        let x = BigInt::from(x);
        
        if n > &x - 1 {
            ans += sum(&x - pre) % md;
            pre = x;
        } else {
            let xx = n - pre;
            ans += sum(xx) % md;
            break;
        }
    }
    println!("{}", ans % md);
}