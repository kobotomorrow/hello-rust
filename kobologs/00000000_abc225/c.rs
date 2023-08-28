#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn is_currect_row(row: &Vec<i32>) -> bool {
    let mut pre = row[0];
    if row.len() > 1 && row[0] % 7 == 0 {
        return false;
    }
    for i in 1..row.len() {
        if i != row.len()-1 && row[i] % 7 == 0 {
            return false;
        }
        if row[i] != pre + 1 {
            return false;
        }
        pre = row[i];
    }
    return true;
}

fn is_currect_columns(column: &Vec<i32>) -> bool {
    let mut pre = column[0];
    for i in 1..column.len() {
        if column[i] != pre + 7 {
            return false;
        }
        pre = column[i];
    }
    return true;
}

fn main() {
    let n: usize = read!();
    let m: usize = read!();
    let rows: Vec<Vec<i32>> = (0..n).map(|_| (0..m).map(|_| read!()).collect()).collect();
    let columns: Vec<Vec<i32>> = (0..m).map(|i| (0..n).map(|j| rows[j][i]).collect()).collect();

    for row in rows.iter() {
        if !is_currect_row(row) {
            println!("No");
            return;
        }
    }
    for column in columns.iter() {
        if !is_currect_columns(column) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}