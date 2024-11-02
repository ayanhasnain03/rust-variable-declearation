# **Variable Creation in Rust**

Variable creation in Rust involves declaring a variable and optionally specifying its type and initial value. Understanding how to create variables, including their mutability and type, is essential for effective Rust programming. Below are the key concepts related to variable creation:

### 1. Basic Variable Declaration

In Rust, you can declare a variable using the `let` keyword. By default, variables are immutable, meaning their values cannot be changed once assigned.

```rust
let a: i32 = 5; // Declares an immutable variable a of type i32 with value 5.
```

### 2. Mutable Variables

If you want to create a variable whose value can be changed, you need to use the `mut` keyword when declaring the variable.

```rust
let mut b: i32 = 10; // Declares a mutable variable b of type i32 with value 10.
b = 20; // Changes the value of b to 20.
```

### 3. Type Inference

Rust supports type inference, meaning you can omit the type declaration, and the compiler will infer the type based on the assigned value.

```rust
let c = 15; // c is inferred to be i32 since it is assigned an integer value.
```

### 4. Shadowing

You can redeclare a variable with the same name in the same scope, which is known as shadowing. This allows you to change the value and type of the variable without making it mutable.

```rust
let d = 30; // d is initially 30.
let d = d + 10; // Shadows d with a new value, now d is 40.
```

### 5. Constants

You can also create constants using the `const` keyword. Constants are always immutable and must have a type annotation.

```rust
const MAX_POINTS: i32 = 100; // Declares a constant `MAX_POINTS` of type `i32`.

```

```rust
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

```

- **Immutability**: Variables are immutable by default. Use `mut` to allow changes.
- **Type Inference**: Rust can infer types, making declarations more concise.
- **Shadowing**: Allows reuse of variable names by declaring a new variable with the same name.
- **Constants**: Use `const` to declare constant values that cannot be changed.
