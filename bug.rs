fn main() {
    let mut x = 5;
    let y = &mut x;
    let z = &mut x; // This is the error
    *y = 10;
    *z = 15;
    println!("x = {}", x);
}