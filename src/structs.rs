// ===== HOUR 5: STRUCTS =====

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
    pub published: u32,
}

impl Book {
    pub fn new(title: String, author: String, pages: u32, published: u32) -> Self {
        Book {
            title,
            author,
            pages,
            published,
        }
    }

    pub fn display_info(&self) {
        println!(
            "Title: {}, Author: {}, Pages: {}, Published: {}",
            self.title, self.author, self.pages, self.published
        );
    }

    pub fn is_long_book(&self) -> bool {
        self.pages > 500
    }
}

pub fn book_example() {
    println!("\n====BOOK STRUCT====\n");

    let book1 = Book::new(
        String::from("The Rust Programming Language"),
        String::from("Steve Klabnik"),
        600,
        2018,
    );
    book1.display_info();
    println!("Is long book: {}\n", book1.is_long_book());
}

#[derive(Debug)]
pub struct BankAccount {
    pub account_holder: String,
    pub balance: f64,
}

impl BankAccount {
    pub fn new(account_holder: String, balance: f64) -> Self {
        BankAccount {
            account_holder,
            balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited: ${}", amount);
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
            println!("Withdrew: ${}", amount);
        } else {
            println!("Insufficient funds");
        }
    }

    pub fn display_balance(&self) {
        println!("Account: {}, Balance: ${}", self.account_holder, self.balance);
    }
}

pub fn bank_account_example() {
    println!("\n====BANK ACCOUNT STRUCT====\n");

    let mut account = BankAccount::new(String::from("Alice"), 1000.0);
    account.display_balance();

    account.deposit(500.0);
    account.display_balance();

    account.withdraw(200.0);
    account.display_balance();

    account.withdraw(5000.0);
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    pub fn display(&self) {
        println!(
            "Rectangle: {}x{}, Area: {}, Perimeter: {}, Square: {}",
            self.width,
            self.height,
            self.area(),
            self.perimeter(),
            self.is_square()
        );
    }
}

pub fn rectangle_example() {
    println!("\n====RECTANGLE STRUCT====\n");

    let rect1 = Rectangle::new(10.0, 5.0);
    rect1.display();

    let rect2 = Rectangle::new(7.0, 7.0);
    rect2.display();
}
