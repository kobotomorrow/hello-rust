#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let n: usize = read!();
    let k: i32 = read!();

    let aa: Vec<i32> = (0..n).map(|_| read!()).collect();
    let bb: Vec<i32> = (0..n).map(|_| read!()).collect();
    let mut pres: HashSet<i32> = HashSet::new();
    pres.insert(aa[0]);
    pres.insert(bb[0]);

    for i in 1..n {
        let a = aa[i];
        let b =  bb[i];

        let mut pres_c = HashSet::new();

        for p in pres.iter() {
            if (p-a).abs() <= k {
                pres_c.insert(a);
            }
            if (p-b).abs() <= k {
                pres_c.insert(b);
            }
        }
        pres = pres_c;
    }
    if pres.len() == 0 {
        println!("No");
        return;
    } else {
        println!("Yes");
    }
}