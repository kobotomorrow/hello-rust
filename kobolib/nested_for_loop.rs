fn nested_for_loop(n: usize, max: usize, indices: &mut Vec<usize>) {
    if n == 0 {
        println!("{:?}", indices);
    } else {
        for i in 0..max {
            indices.push(i);
            nested_for_loop(n - 1, max, indices);
            indices.pop();
        }
    }
}

fn main() {
    let mut indices = Vec::new();
    nested_for_loop(2, 3, &mut indices);
}