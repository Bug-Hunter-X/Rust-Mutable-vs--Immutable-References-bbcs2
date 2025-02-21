fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modifying x through y is fine
    println!("x = {}", x); // Output: x = 6
}

//Alternative solution using shadowing
fn main() {
    let mut x = 5;
    let x = x + 1; //Using shadowing to create new immutable variable
    println!("x = {}", x); //Output: x = 6
} 