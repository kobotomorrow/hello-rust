#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
use std::{collections::HashMap};
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn main() {
    let mut ranker = HashMap::new();
    ranker.insert("tourist", 3858);
    ranker.insert("ksun48", 3679);
    ranker.insert("Benq", 3658);
    ranker.insert("Um_nik", 3648);
    ranker.insert("apiad", 3638);
    ranker.insert("Stonefeang", 3630);
    ranker.insert("ecnerwala", 3613);
    ranker.insert("mnbvmar", 3555);
    ranker.insert("newbiedmy", 3516);
    ranker.insert("semiexp", 3481);

    let s: String = read!();
    let ss: &str = &s;
    println!("{}", ranker.get(ss).unwrap());
}