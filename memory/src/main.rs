use std::mem;

fn main() {
    let xs: [i32; 5] = [4, 5, 6, 7, 8];
    println!("Elemnet of first array is {} length of array is {} and size of the value is {}", xs[0], xs.len(), mem::size_of_val(&xs));
}
