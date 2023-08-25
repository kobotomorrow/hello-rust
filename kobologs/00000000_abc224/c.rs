#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::{collections::HashMap, hash::Hash};
// use std::collections::VecDeque;
// use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn is_same_line(p1: &Point, p2: &Point, p3: &Point) -> bool {
    (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y) == 0
}

fn main() {
    let n: usize = read!();
    let points: Vec<Point> = (0..n).map(|_|
        Point {
            x: read!(),
            y: read!(),
        }
    ).collect();

    let mut ans = 0;
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                if !is_same_line(&points[i], &points[j], &points[k]) {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}