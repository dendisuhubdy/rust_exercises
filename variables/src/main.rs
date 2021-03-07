fn main() {
    // i8, u8, i16, i32, i64, u64, isize, usize
    // f32, f64
    let a = 1 + 20; // addition
    let s = 30 - 20; // subtraction
    let m = 5 * 10; // multiplication
    let d = 4 / 6; // division
    let r = 49 % 2; // mod
    println!("a is {}", a);
    println!("Hello, world!");

    let c: char = 'z';

    let t: (i32, f64, char) = (42, 6.12, 'j');
    let (_, _, x) = t;

    let a = [1, 2, 3, 4, 5, 6];
    let a1 = a[0]; // allocated on the stack not on the heap, not flexible
}
