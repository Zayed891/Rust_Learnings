// ===== VECTORS =====

pub fn vec_exercise_1() {
    println!("\n====VEC CREATION & PUSH====\n");

    let mut numbers = vec![1, 2, 3];
    numbers.push(4);
    numbers.push(5);

    println!("Numbers: {:?}", numbers);
}

pub fn vec_exercise_2() {
    println!("\n====VEC POP & ACCESS====\n");

    let mut fruits = vec!["Apple", "Banana", "Cherry"];

    if let Some(last) = fruits.pop() {
        println!("Popped: {}", last);
    }

    println!("First fruit: {}", fruits[0]);
    println!("All fruits: {:?}\n", fruits);
}

pub fn vec_exercise_3() {
    println!("====VEC FILTER====\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let evens: Vec<i32> = numbers.iter().filter(|&&n| n % 2 == 0).copied().collect();

    println!("Original: {:?}", numbers);
    println!("Evens: {:?}\n", evens);
}

pub fn vec_exercise_4() {
    println!("====VEC MAP - DOUBLE====\n");

    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = numbers.iter().map(|&n| n * 2).collect();

    println!("Original: {:?}", numbers);
    println!("Doubled: {:?}\n", doubled);
}

pub fn vec_exercise_5() {
    println!("====VEC MAP - SQUARE====\n");

    let numbers = vec![1, 2, 3, 4, 5];

    let squared: Vec<i32> = numbers.iter().map(|&n| n * n).collect();

    println!("Original: {:?}", numbers);
    println!("Squared: {:?}\n", squared);
}

pub fn vec_exercise_6() {
    println!("====VEC FILTER + MAP====\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * 3)
        .collect();

    println!("Original: {:?}", numbers);
    println!("Even numbers * 3: {:?}\n", result);
}

pub fn vec_exercise_7() {
    println!("====VEC LENGTH, SUM, COUNT====\n");

    let numbers = vec![1, 2, 3, 4, 5];

    let length = numbers.len();
    let sum: i32 = numbers.iter().sum();
    let count = numbers.iter().count();

    println!("Numbers: {:?}", numbers);
    println!("Length: {}", length);
    println!("Sum: {}", sum);
    println!("Count: {}\n", count);
}

pub fn vec_exercise_8() {
    println!("====VEC REMOVE & CLEAR====\n");

    let mut items = vec!["A", "B", "C", "D"];
    println!("Before remove: {:?}", items);

    items.remove(1);
    println!("After removing index 1: {:?}", items);

    items.clear();
    println!("After clear: {:?}", items);
}
