// ===== HOUR 1: FUNDAMENTALS =====

pub fn variables_example() {
    println!("====RUST FUNDAMENTALS===\n");

    // Immutable variables
    let name = "Rust Learner";
    println!("Hello, {}!", name);

    // Mutable variables
    let mut age = 25;
    println!("Age is: {}", age);
    age = 26;
    println!("Birthday! New age: {}\n", age);

    // Shadowing
    let x = 5;
    let x = x + 1;
    println!("Shadowed x: {}\n", x);

    let name = "Rust Learner";
    println!("{}", name);

    let name = name.len();
    println!("{}", name);
}

pub fn data_types_example() {
    // Data Types
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';

    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}\n", character);

    // Basic math operations
    let sum = 10 + 5;
    let difference = 10 - 5;
    let product = 10 * 5;
    let quotient = 10 / 5;
    let remainder = 10 % 3;

    println!("10 + 5 = {}", sum);
    println!("10 - 5 = {}", difference);
    println!("10 * 5 = {}", product);
    println!("10 / 5 = {}", quotient);
    println!("10 % 3 = {}", remainder);
}
