#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn nested_for_loop(n: usize, cur: usize, b: &Vec<Vec<i64>>, que: &mut Vec<i64>, x: i64, ans: &mut i64) {
    if cur == n {
        let mut sum = 1;
        for q in que {
            if sum > x {
                break;
            }
            sum *= *q;
        }
        if sum == x {
            *ans += 1;
        }
    } else {
        for i in 0..b[cur].len() {
            que.push(b[cur][i]);
            nested_for_loop(n, cur + 1, b, que, x, ans);
            que.pop();
        }
    }
}



fn main() {
    let n: usize = read!();
    let x: i64 = read!();
    let mut b = Vec::new();

    for _ in 0..n {
        let mut bb = Vec::new();
        let l: usize = read!();
        for _ in 0..l {
            let a: i64 = read!();
            bb.push(a);
        }
        b.push(bb);
    }

    let mut que = Vec::new();
    let mut ans = 0;
    nested_for_loop(n, 0, &b, &mut que, x, &mut ans);
    println!("{}", ans);
}