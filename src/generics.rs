// ===== GENERICS & TRAITS =====

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

pub fn stack_example() {
    println!("\n====STACK<T> - INTEGERS====\n");

    let mut stack: Stack<i32> = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("Stack size: {}", stack.size());
    println!("Peek: {:?}", stack.peek());
    println!("Pop: {:?}", stack.pop());
    println!("Size after pop: {}\n", stack.size());
}

pub fn stack_strings_example() {
    println!("====STACK<T> - STRINGS====\n");

    let mut stack: Stack<String> = Stack::new();
    stack.push(String::from("Rust"));
    stack.push(String::from("is"));
    stack.push(String::from("awesome"));

    while !stack.is_empty() {
        println!("Pop: {}", stack.pop().unwrap());
    }

    println!();
}

pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> Pair<T, U> {
    pub fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    pub fn first_value(&self) -> &T {
        &self.first
    }

    pub fn second_value(&self) -> &U {
        &self.second
    }

    pub fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }

    pub fn display(&self)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("Pair: ({}, {})", self.first, self.second);
    }
}

pub fn pair_example() {
    println!("\n====PAIR<T, U> - INT & STRING====\n");

    let pair = Pair::new(42, String::from("Rust"));

    println!("First: {}", pair.first_value());
    println!("Second: {}", pair.second_value());
    pair.display();

    let swapped = pair.swap();
    swapped.display();

    println!();
}

pub fn pair_float_example() {
    println!("====PAIR<T, U> - FLOATS====\n");

    let pair = Pair::new(3.14, 2.71);

    println!("First: {}", pair.first_value());
    println!("Second: {}", pair.second_value());
    pair.display();

    println!();
}

pub trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius: {}", self.radius);
    }

    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

pub struct RectangleShape {
    pub width: f64,
    pub height: f64,
}

impl Drawable for RectangleShape {
    fn draw(&self) {
        println!("Drawing rectangle: {}x{}", self.width, self.height);
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub struct Square {
    pub side: f64,
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing square with side: {}", self.side);
    }

    fn area(&self) -> f64 {
        self.side * self.side
    }
}

pub fn display_shape<T: Drawable>(shape: &T) {
    shape.draw();
    println!("Area: {}\n", shape.area());
}

pub fn drawable_example() {
    println!("\n====DRAWABLE TRAIT====\n");

    let circle = Circle { radius: 5.0 };
    display_shape(&circle);

    let rect = RectangleShape {
        width: 10.0,
        height: 5.0,
    };
    display_shape(&rect);

    let square = Square { side: 7.0 };
    display_shape(&square);
}
