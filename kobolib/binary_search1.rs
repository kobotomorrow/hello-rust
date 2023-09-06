// #[macro_use] extern crate text_io;

fn main() {
    let mut v = vec![1, 2, 5, 11, 99, 3, 8, 100, 0];
    println!("{:?}", v);
    v.sort();
    println!("{:?}", v);
    let t = v.binary_search(&99).unwrap();
    println!("{}", t);
    println!("{}", v[t]);
}