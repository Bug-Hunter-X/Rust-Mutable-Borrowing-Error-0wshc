fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y += 1; // Modifying x through y
    let z = &x; // z is an immutable reference to x
    println!("x = {}", x); // x = 6
    println!("z = {}", *z); // z = 6

    // The problem is when we try to use multiple mutable reference to the same variable
    // let a = &mut x; // This will cause a compiler error
    // let b = &mut x; // This will cause a compiler error
}