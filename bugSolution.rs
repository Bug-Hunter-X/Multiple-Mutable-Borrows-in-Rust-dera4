fn main() {
    let mut x = 5;
    { //Creating a scope for the borrow
        let y = &mut x; 
        *y = 10; 
    }
    let z = &mut x;
    *z = 15;
    println!("x = {}", x);
}

//Alternative solution using cloning:
fn main() {
    let mut x = 5;
    let mut y = x;
    let mut z = x;
    y = 10;
    z = 15;
    println!("x = {}", x); // x remains 5, y = 10, z = 15
} 