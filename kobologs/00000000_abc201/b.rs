#[macro_use] extern crate text_io;

fn main() {
    let n: usize = read!();

    let mut mt: Vec<(i32, String)> = vec![(0, String::new()); n];
    for i in 0..n {
        let s: String = read!();
        let t: i32 = read!();

        mt[i] = (t, s);
    }

    mt.sort();
    
    println!("{}", mt[n-2].1)
}