#[macro_use] extern crate text_io;

fn main() {
    let mut a = vec![0; 3];

    for i in 0..3 {
        let aa = read!();
        a[i] = aa;
    }

    a.sort();

    if a[1] - a[0] == a[2] - a[1] {
        println!("Yes");
    } else {
        println!("No");
    }
}