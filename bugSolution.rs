fn main() {
    let mut x = 5;
    { // Create a new scope for the mutable reference
        let y = &mut x;
        *y += 1;
    } // The mutable reference y goes out of scope here
    let z = &x; // Now we can create an immutable reference 
    println!("x = {}", x); // x = 6
    println!("z = {}", *z); // z = 6
} 