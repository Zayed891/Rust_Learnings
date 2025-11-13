// ===== HOUR 5: ENUMS =====

#[derive(Debug)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

pub fn day_example() {
    println!("\n====DAY ENUM====\n");

    let today = Day::Wednesday;
    match today {
        Day::Monday => println!("Start of week"),
        Day::Friday => println!("Almost weekend!"),
        Day::Saturday | Day::Sunday => println!("Weekend!"),
        _ => println!("Midweek"),
    }
}

#[derive(Debug)]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

impl Season {
    pub fn temperature(&self) -> i32 {
        match self {
            Season::Spring => 15,
            Season::Summer => 30,
            Season::Fall => 20,
            Season::Winter => 5,
        }
    }
}

pub fn season_example() {
    println!("\n====SEASON ENUM====\n");

    let season = Season::Summer;
    println!("Season: {:?}, Temperature: {}°C\n", season, season.temperature());
}

#[derive(Debug)]
pub enum Message {
    Text(String),
    Number(i32),
    Empty,
}

pub fn message_example() {
    println!("====MESSAGE ENUM====\n");

    let msg1 = Message::Text(String::from("Hello"));
    let msg2 = Message::Number(42);
    let msg3 = Message::Empty;

    match msg1 {
        Message::Text(s) => println!("Text: {}", s),
        _ => println!("Not text"),
    }

    match msg2 {
        Message::Number(n) => println!("Number: {}", n),
        _ => println!("Not a number"),
    }

    match msg3 {
        Message::Empty => println!("Empty message\n"),
        _ => println!("Not empty"),
    }
}

#[derive(Debug)]
pub enum UserStatus {
    Active,
    Inactive,
    Banned,
}

impl UserStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, UserStatus::Active)
    }
}

pub fn user_status_example() {
    println!("====USER STATUS ENUM====\n");

    let user1 = UserStatus::Active;
    let user2 = UserStatus::Banned;

    println!("User 1 active: {}", user1.is_active());
    println!("User 2 active: {}\n", user2.is_active());
}

#[derive(Debug)]
pub enum PaymentMethod {
    CreditCard,
    Debit,
    Cash,
    Check,
}

impl PaymentMethod {
    pub fn process_payment(&self, amount: f64) {
        match self {
            PaymentMethod::CreditCard => println!("Processing credit card payment: ${}", amount),
            PaymentMethod::Debit => println!("Processing debit payment: ${}", amount),
            PaymentMethod::Cash => println!("Processing cash payment: ${}", amount),
            PaymentMethod::Check => println!("Processing check payment: ${}", amount),
        }
    }
}

pub fn payment_method_example() {
    println!("====PAYMENT METHOD ENUM====\n");

    let method = PaymentMethod::CreditCard;
    method.process_payment(99.99);
}
