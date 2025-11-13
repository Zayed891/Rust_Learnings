// ===== HOUR 2: CONTROL FLOW =====

pub fn conditional_exercise_1() {
    println!("\n====CONDITIONALS====\n");

    let age = 15;

    if age >= 18 {
        println!("You are an adult");
    } else if age >= 13 {
        println!("You are a teenager");
    } else {
        println!("You are a child");
    }

    println!();

    let x = 10;
    let y = 20;

    if x < y && y > 15 {
        println!("x is less than y AND y is greater than 15");
    }
}

pub fn loop_exercise_1() {
    println!("\n====LOOPS====\n");

    // For loop
    for i in 1..=5 {
        println!("i = {}", i);
    }

    println!();

    // While loop
    let mut count = 5;
    while count > 0 {
        println!("Countdown: {}", count);
        count -= 1;
    }
}

pub fn match_exercise_1() {
    println!("\n====MATCH EXPRESSION====\n");

    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
}

pub fn match_exercise_2() {
    println!("\n====MATCH WITH RANGES====\n");

    let score = 85;

    match score {
        90..=100 => println!("Grade: A"),
        80..=89 => println!("Grade: B"),
        70..=79 => println!("Grade: C"),
        _ => println!("Grade: F"),
    }
}

pub fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

pub fn run_factorial() {
    println!("\n====FACTORIAL USING MATCH====\n");
    println!("5! = {}", factorial(5));
}
