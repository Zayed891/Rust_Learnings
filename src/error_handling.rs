// ===== HOUR 4: ERROR HANDLING =====

pub fn result_example_1() {
    println!("\n====RESULT<T, E> - SAFE SQRT====\n");

    let result1 = safe_sqrt(16.0);
    match result1 {
        Ok(val) => println!("√16 = {}", val),
        Err(e) => println!("Error: {}", e),
    }

    let result2 = safe_sqrt(-4.0);
    match result2 {
        Ok(val) => println!("√-4 = {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

fn safe_sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err(String::from("Cannot take sqrt of negative number"))
    } else {
        Ok(x.sqrt())
    }
}

pub fn option_example_1() {
    println!("\n====OPTION<T> - FIND EVEN====\n");

    let numbers = vec![1, 2, 3, 4, 5, 6];
    let result = find_even(&numbers);

    match result {
        Some(num) => println!("First even: {}", num),
        None => println!("No even number found"),
    }
}

fn find_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}

pub fn option_example_2() {
    println!("\n====OPTION<T> - VALIDATE AGE====\n");

    let age = validate_age(25);
    match age {
        Some(a) => println!("Valid age: {}", a),
        None => println!("Invalid age"),
    }

    let age = validate_age(150);
    match age {
        Some(a) => println!("Valid age: {}", a),
        None => println!("Invalid age"),
    }
}

fn validate_age(age: i32) -> Option<i32> {
    if age > 0 && age < 150 {
        Some(age)
    } else {
        None
    }
}

pub fn question_mark_operator_1() {
    println!("\n====? OPERATOR - PARSE AGE====\n");

    match parse_age("25") {
        Ok(age) => println!("Parsed age: {}", age),
        Err(e) => println!("Error: {}", e),
    }

    match parse_age("invalid") {
        Ok(age) => println!("Parsed age: {}", age),
        Err(e) => println!("Error: {}", e),
    }
}

fn parse_age(s: &str) -> Result<i32, String> {
    let age: i32 = s.parse().map_err(|_| String::from("Failed to parse"))?;
    if age < 0 {
        return Err(String::from("Age cannot be negative"));
    }
    Ok(age)
}

pub fn question_mark_operator_2() {
    println!("\n====? OPERATOR - PROCESS USER INPUT====\n");

    match process_user_input("John", "30") {
        Ok((name, age)) => println!("User: {}, Age: {}", name, age),
        Err(e) => println!("Error: {}", e),
    }

    match process_user_input("Jane", "invalid") {
        Ok((name, age)) => println!("User: {}, Age: {}", name, age),
        Err(e) => println!("Error: {}", e),
    }
}

fn process_user_input<'a>(name: &'a str, age_str: &str) -> Result<(&'a str, i32), String> {
    let age: i32 = age_str.parse().map_err(|_| String::from("Invalid age"))?;
    if age < 0 || age > 120 {
        return Err(String::from("Age out of valid range"));
    }
    Ok((name, age))
}
