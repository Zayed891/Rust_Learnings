// ===== HOUR 3: OWNERSHIP & LIFETIMES =====

pub fn borrowing_example() {
    println!("\n====BORROWING====\n");

    let s1 = String::from("Hello");
    println!("s1: {}", s1);

    let s2 = &s1; // Immutable borrow
    println!("s2 (immutable borrow): {}", s2);
    println!("s1 still works: {}", s1);
}

pub fn mutable_borrow_example() {
    println!("\n====MUTABLE BORROW====\n");

    let mut s = String::from("Hello");
    println!("Before: {}", s);

    add_world(&mut s);
    println!("After: {}\n", s);
}

fn add_world(s: &mut String) {
    s.push_str(" World");
}

pub fn lifetime_example() {
    println!("====LIFETIMES====\n");

    let string1 = String::from("Hello");
    let string2 = "World";

    let result = get_first(&string1, string2);
    println!("First string: {}\n", result);
}

fn get_first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

pub fn shorter_lifetime() {
    println!("====LIFETIME: SHORTER STRING====\n");

    let string1 = String::from("Rust");
    let string2 = String::from("Programming");

    let result = shorter_str(&string1, &string2);
    println!("Shorter: {}\n", result);
}

fn shorter_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}

pub fn struct_with_lifetime() {
    println!("====STRUCT WITH LIFETIME====\n");

    let text = String::from("Important");
    let message = Message {
        content: &text,
    };

    println!("Message: {}\n", message.display());
}

pub struct Message<'a> {
    pub content: &'a str,
}

impl<'a> Message<'a> {
    pub fn display(&self) -> &'a str {
        self.content
    }
}

pub fn combined_lifetimes() {
    println!("====COMBINED LIFETIMES====\n");

    let string1 = String::from("The quick");
    let string2 = String::from("brown fox");

    let text = string1.as_str();
    println!("Text: {}", text);

    let combined = combine_strings(text, &string2);
    println!("Combined: {}\n", combined);
}

fn combine_strings<'a>(x: &'a str, y: &str) -> &'a str {
    println!("{} {}", x, y);
    x
}
