#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn kumi(n: i64, k: i64) -> i64 {
    let mut ans: i64 = 1;
    let mut nn = n;

    for i in 1..k+1 {
        ans *= nn;
        nn -= 1;
        ans /= i;
    }
    return ans;
}

fn middle(a: i64, b: i64, n: i64) -> i64 {
    return n * a / (a + b);
}

fn main() {
    let mut a: i64 = read!();
    let mut b: i64 = read!();
    let mut k: i64 = read!();
    
    let mut ans = String::new();
    
    for _i in 0..a+b {
        let r = kumi(a + b, a);
        let m = middle(a, b, r);
        if m < k {
            ans += "b";
            b -= 1;
            k -= m;
        } else {
            ans += "a";
            a -= 1;
        }
    }
    println!("{}", ans);
}