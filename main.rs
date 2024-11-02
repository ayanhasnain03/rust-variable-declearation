fn main() {
    // Variable creation
    let a: i32 = 5; // Immutable variable
    println!("a is: {}", a);

    // Mutable variable
    let mut b: i32 = 10; // Mutable variable
    println!("b is: {}", b);
    b = 20; // Changing the value of b
    println!("b after change is: {}", b);

    // Shadowing
    let c = 15; // Initially c is 15
    let c = c + 5; // Shadows c with a new value, now c is 20
    println!("c is: {}", c);

    // Constant
    const MAX: i32 = 100;
    println!("The maximum value is: {}", MAX);
}
